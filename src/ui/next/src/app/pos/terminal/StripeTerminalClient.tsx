'use client';

import React, { useState, useEffect } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';

export default function StripeTerminalClient({ amount, productId, tenantId }: { amount: number, productId: string, tenantId: string }) {
  const [terminal, setTerminal] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [reserving, setReserving] = useState<boolean>(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [paymentComplete, setPaymentComplete] = useState<boolean>(false);
  const [customerEmail, setCustomerEmail] = useState<string>('');
  const [receiptSent, setReceiptSent] = useState<boolean>(false);

  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        setStatus('Failed to load Stripe Terminal SDK.');
        return;
      }

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const res = await fetch('/api/v1/payments/terminal/token', { method: 'POST' });
          const data = await res.json();
          return data.secret;
        },
        onUnexpectedReaderDisconnect: async () => {
          setStatus('Reader disconnected unexpectedly.');
          setConnectedReader(null);
          if (sessionId && navigator.onLine) {
            await fetch('/api/v1/payments/terminal/session/update', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ session_id: sessionId, status: 'OFFLINE' })
            }).catch(console.error);
          }
        }
      });
      setTerminal(term);
      setStatus('Terminal initialized. Ready to discover readers.');
    }
    initTerminal();

    return () => {
      // End session on unmount
      if (sessionId && navigator.onLine) {
        fetch('/api/v1/payments/terminal/session/end', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'Keep-Alive': 'timeout=5, max=100' },
          body: JSON.stringify({ session_id: sessionId }),
          keepalive: true
        }).catch(console.error);
      }
    };
  }, [sessionId]);

  useEffect(() => {
    const handleOnline = async () => {
      if (sessionId && connectedReader) {
        await fetch('/api/v1/payments/terminal/session/update', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ session_id: sessionId, status: 'ACTIVE' })
        }).catch(console.error);
      }
    };
    const handleOffline = async () => {
      if (sessionId) {
        // Optimistic offline status locally, as we can't send a request when offline
        setStatus('Terminal is Offline');
      }
    };

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, [sessionId, connectedReader]);

  const discoverReaders = async () => {
    if (!terminal) return;
    setStatus('Discovering readers...');
    const result = await terminal.discoverReaders({ simulated: true });
    if (result.error) {
      setStatus('Discovery failed: ' + result.error.message);
    } else {
      setDiscoveredReaders(result.discoveredReaders);
      setStatus('Discovered ' + result.discoveredReaders.length + ' readers.');
    }
  };

  const connectReader = async (reader: any) => {
    if (!terminal) return;
    setStatus('Connecting to reader...');
    const result = await terminal.connectReader(reader);
    if (result.error) {
      setStatus('Connection failed: ' + result.error.message);
    } else {
      setConnectedReader(result.reader);
      setStatus('Connected to reader: ' + result.reader.label);

      // Start a session
      try {
        const res = await fetch('/api/v1/payments/terminal/session/start', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ device_id: result.reader.id })
        });
        const data = await res.json();
        if (data.success) {
          setSessionId(data.session_id);
          if (typeof window !== 'undefined') {
             localStorage.setItem('ohc_active_terminal_session_id', data.session_id);
          }
        } else {
          console.error("Failed to start terminal session:", data.error_message);
          setStatus("Connected, but session start failed: " + data.error_message);
        }
      } catch (err) {
        console.error("Failed to start terminal session", err);
        setStatus("Connected, but session start failed");
      }
    }
  };

  const processPayment = async () => {
    if (!terminal || !connectedReader) return;

    setReserving(true);

    if (!navigator.onLine) {
       setStatus('Processing offline payment...');
       // Mock the terminal process for offline
       setTimeout(() => {
          const transactionId = `tx_offline_${Date.now()}_${Math.floor(Math.random() * 1000)}`;
          const tx = {
             id: transactionId,
             client_id: 'terminal_1',
             amount_cents: amount,
             currency: 'usd',
             payload: JSON.stringify([{ product_id: productId, quantity: 1 }]),
             timestamp: new Date().toISOString()
          };
          // Also sync with OfflineStore directly to match page.tsx expectations
          const existingTxs = JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
          existingTxs.push(tx);
          localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(existingTxs));

          setStatus('Payment saved offline. Will sync when network is restored.');
          setReserving(false);
       }, 1500);
       return;
    }

    setStatus('Reserving inventory...');

    let lockId = '';
    try {
      const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: 1, ttl_seconds: 15 })
      });
      const reserveData = await reserveRes.json();

      if (!reserveData.success) {
        setStatus('Reservation failed: ' + (reserveData.error_message || 'Item is currently being purchased elsewhere'));
        setReserving(false);
        return;
      }
      lockId = reserveData.lock_id;
    } catch (e: any) {
      setStatus('Reservation error: ' + e.message);
      setReserving(false);
      return;
    }

    setStatus('Creating payment intent...');
    try {
      const res = await fetch('/api/v1/payments/terminal/intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, currency: 'usd' })
      });
      const data = await res.json();

      setStatus('Collecting payment method...');
      const collectResult = await terminal.collectPaymentMethod(data.client_secret);
      if (collectResult.error) {
        setStatus('Payment collection failed: ' + collectResult.error.message);
        setReserving(false);
        return;
      }

      setStatus('Processing payment...');
      const processResult = await terminal.processPayment(collectResult.paymentIntent);
      if (processResult.error) {
        setStatus('Payment processing failed: ' + processResult.error.message);
      } else {
        setStatus('Payment successful. Committing inventory...');

        try {
          const commitRes = await fetch('/api/v1/payments/terminal/commit', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: 1, lock_id: lockId })
          });
          const commitData = await commitRes.json();
          if (commitData.success) {
            setStatus('Payment successful!');
            setPaymentComplete(true);
          } else {
            setStatus('Payment successful, but inventory commit failed: ' + commitData.error_message);
          }
        } catch (commitErr: any) {
          setStatus('Payment successful, but inventory commit error: ' + commitErr.message);
        }
      }
    } catch (e: any) {
      setStatus('Error: ' + e.message);
    } finally {
      setReserving(false);
    }
  };

  const handleSendReceipt = async () => {
    if (!customerEmail) return;
    setStatus('Sending receipt...');
    try {
      const res = await fetch('/api/v1/payments/terminal/receipt', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: customerEmail, transaction_id: "pos_sale" })
      });
      const data = await res.json();
      if (data.success) {
        setReceiptSent(true);
        setStatus('Receipt sent and customer profile updated.');
      } else {
        setStatus('Failed to send receipt: ' + data.error_message);
      }
    } catch (e: any) {
      setStatus('Failed to send receipt: ' + e.message);
    }
  };

  const resetPayment = () => {
    setPaymentComplete(false);
    setReceiptSent(false);
    setCustomerEmail('');
    setStatus('Connected to reader: ' + connectedReader?.label);
  };

  return (
    <div className="p-6 border border-white/40 dark:border-white/10 rounded-2xl shadow-lg bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] mt-6 relative transition-all">
      <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tap to Pay via Terminal</h2>
      <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mb-6 font-medium">Status: {status}</p>

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} className="w-full bg-[#0066FF] text-white px-4 py-3 rounded-xl font-bold hover:bg-blue-700 transition-colors shadow-md shadow-blue-500/20 active:scale-[0.98]">
            Discover Readers
          </button>
          <ul className="mt-4 space-y-2">
            {discoveredReaders.map(reader => (
              <li key={reader.id} className="flex justify-between items-center p-3 border border-[#1D1D1F]/10 dark:border-white/10 rounded-xl bg-white/50 dark:bg-black/20 shadow-sm backdrop-blur-md">
                <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">{reader.label || reader.id}</span>
                <button onClick={() => connectReader(reader)} className="bg-[#34C759] text-white px-4 py-1.5 rounded-lg text-sm font-bold shadow-sm shadow-green-500/20 hover:bg-green-600 transition-colors active:scale-[0.98]">
                  Connect
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {connectedReader && !paymentComplete && (
        <div>
          <button onClick={processPayment} disabled={reserving} className={`w-full bg-[#0066FF] text-white px-4 py-4 rounded-xl font-bold shadow-md shadow-blue-500/20 transition-all ${reserving ? 'opacity-50' : 'hover:bg-blue-700 active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Charge $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      )}

      {paymentComplete && (
        <div className="mt-4 pt-4 border-t border-[#1D1D1F]/10 dark:border-white/10 animate-in fade-in slide-in-from-bottom-2 duration-300">
           <div className="flex items-center justify-center w-12 h-12 rounded-full bg-[#34C759]/10 text-[#34C759] mx-auto mb-3">
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
           </div>
           <h3 className="text-center font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Payment Successful</h3>

           {!receiptSent ? (
             <div className="space-y-3">
               <label className="block text-xs font-semibold text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 uppercase tracking-wider">Send Digital Receipt</label>
               <input
                 type="email"
                 placeholder="customer@email.com"
                 value={customerEmail}
                 onChange={(e) => setCustomerEmail(e.target.value)}
                 className="w-full bg-white/50 dark:bg-black/20 border border-[#1D1D1F]/10 dark:border-white/10 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-[#1D1D1F] dark:text-[#F5F5F7]"
               />
               <button
                 onClick={handleSendReceipt}
                 disabled={!customerEmail}
                 className="w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-black px-4 py-3 rounded-xl font-bold shadow-sm transition-all disabled:opacity-50 active:scale-[0.98]"
               >
                 Send Receipt & Add to CRM
               </button>
               <button
                 onClick={resetPayment}
                 className="w-full bg-transparent text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 px-4 py-2 rounded-xl font-medium text-sm transition-all hover:text-[#1D1D1F] dark:hover:text-[#F5F5F7]"
               >
                 No Receipt (New Sale)
               </button>
             </div>
           ) : (
             <div className="text-center space-y-4">
               <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60">Receipt sent to {customerEmail}. Agent updated customer profile.</p>
               <button
                 onClick={resetPayment}
                 className="w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-black px-4 py-3 rounded-xl font-bold shadow-sm transition-all active:scale-[0.98]"
               >
                 Start New Sale
               </button>
             </div>
           )}
        </div>
      )}
    </div>
  );
}
