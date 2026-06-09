'use client';

import React, { useState, useEffect } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';
import { SyncManager } from '../../../lib/sync/SyncManager';

export default function StripeTerminalClient({ amount, productId, tenantId }: { amount: number, productId: string, tenantId: string }) {
  const [terminal, setTerminal] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [reserving, setReserving] = useState<boolean>(false);
  const [sessionId, setSessionId] = useState<string | null>(null);

  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        setStatus('Failed to load Stripe Terminal SDK.');
        return;
      }

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const res = await fetch('/api/terminal/connection_token', { method: 'POST' });
          const data = await res.json();
          return data.secret;
        },
        onUnexpectedReaderDisconnect: async () => {
          setStatus('Reader disconnected unexpectedly.');
          setConnectedReader(null);
          if (sessionId && navigator.onLine) {
            await fetch('/api/terminal/session/update', {
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
        fetch('/api/terminal/session/end', {
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
        await fetch('/api/terminal/session/update', {
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
        const res = await fetch('/api/terminal/session/start', {
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

          SyncManager.getInstance().enqueue({
             type: 'tap_to_pay',
             id: transactionId,
             product_id: productId,
             amount: amount,
             quantity: 1,
             idempotency_key: `idemp_${transactionId}`,
             currency: 'usd'
          });
          setStatus('Payment saved offline. Will sync when network is restored.');
          setReserving(false);
       }, 1500);
       return;
    }

    setStatus('Reserving inventory...');

    let lockId = '';
    try {
      const reserveRes = await fetch('/api/pos/terminal/reserve', {
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
      const res = await fetch('/api/terminal/create_payment_intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, currency: 'usd', product_id: productId })
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
          const commitRes = await fetch('/api/pos/terminal/commit', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: 1, lock_id: lockId })
          });
          const commitData = await commitRes.json();
          if (commitData.success) {
            setStatus('Payment successful!');
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

  return (
    <div className="p-6 border border-white/40 rounded-2xl shadow-lg bg-white/65 backdrop-blur-[30px] saturate-[210%] mt-6 relative">
      <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Tap to Pay via Terminal</h2>
      <p className="text-sm text-gray-600 mb-6 font-medium">Status: {status}</p>

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} className="w-full bg-[#0066FF] text-white px-4 py-3 rounded-xl font-bold hover:bg-blue-700 transition-colors shadow-md shadow-blue-500/20 active:scale-[0.98]">
            Discover Readers
          </button>
          <ul className="mt-4 space-y-2">
            {discoveredReaders.map(reader => (
              <li key={reader.id} className="flex justify-between items-center p-3 border border-gray-100 rounded-xl bg-white shadow-sm">
                <span className="font-medium text-gray-800 text-sm">{reader.label || reader.id}</span>
                <button onClick={() => connectReader(reader)} className="bg-[#34C759] text-white px-4 py-1.5 rounded-lg text-sm font-bold shadow-sm shadow-green-500/20 hover:bg-green-600 transition-colors active:scale-[0.98]">
                  Connect
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {connectedReader && (
        <div>
          <button onClick={processPayment} disabled={reserving} className={`w-full bg-[#0066FF] text-white px-4 py-4 rounded-xl font-bold shadow-md shadow-blue-500/20 transition-all ${reserving ? 'opacity-50' : 'hover:bg-blue-700 active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Charge $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      )}
    </div>
  );
}
