"use client";

import React, { useEffect, useState } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';
import { SyncManager } from '../../../lib/sync/SyncManager';

interface StripeTerminalClientProps {
  onSuccess?: () => void;
  cart?: { product: any, quantity: number }[];
  amount: number;
  productId: string;
  tenantId: string;
  onOptimisticReserve?: () => void;
  onOptimisticRollback?: () => void;
}

export default function StripeTerminalClient({ amount, productId, cart, tenantId, onOptimisticReserve, onOptimisticRollback, onSuccess }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<any>(null);
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [reserving, setReserving] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [pendingReconciliation, setPendingReconciliation] = useState<any[]>([]);

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

    const handleSyncReconciliation = (event: any) => {
      if (event.detail && event.detail.pending_reconciliation) {
         setPendingReconciliation(event.detail.pending_reconciliation);
         setStatus('Offline sync completed with conflicts');
      }
    };

    if (typeof window !== 'undefined') {
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);
        window.addEventListener('ohc_sync_reconciliation', handleSyncReconciliation);
        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
          window.removeEventListener('ohc_sync_reconciliation', handleSyncReconciliation);
        };
    }
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
       onOptimisticReserve?.();
       // Mock the terminal process for offline
       setTimeout(async () => {
          const transactionId = `tx_offline_${Date.now()}_${Math.floor(Math.random() * 1000)}`;
          const tx = {
             id: transactionId,
             type: 'tap_to_pay',
             client_id: 'terminal_1',
             amount_cents: amount,
             amount: amount,
             currency: 'usd',
             product_id: cart ? cart[0].product.id : productId,
             quantity: cart ? cart[0].quantity : 1,
             payload: JSON.stringify((cart || [{product: {id: productId}, quantity: 1}]).map(c => ({ product_id: c.product.id, quantity: c.quantity }))),
             timestamp: new Date().toISOString()
          };

          for (const item of (cart || [{product: {id: productId}, quantity: 1}])) {
            const crdtTx = {
              id: `crdt_${transactionId}_${item.product.id}`,
              type: 'CRDT_MUTATION',
              timestamp: new Date().toISOString(),
              payload: {
                 entity_id: item.product.id,
                 data: {
                    pn_counter_n_increment: item.quantity
                 }
              }
            };
            await SyncManager.getInstance().enqueue(crdtTx);
          }
          await SyncManager.getInstance().enqueue(tx);

          setStatus('Payment saved offline. Will sync when network is restored.');
          setTimeout(() => setStatus('Terminal ready.'), 3000);
          setReserving(false);
       }, 1500);
       return;
    }

    setStatus('Reserving inventory...');
    onOptimisticReserve?.();

    let lockIds: string[] = [];
    let lockId = '';
    try {
      for (const item of (cart || [{product: {id: productId}, quantity: 1}])) {
        const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tenant_id: tenantId, product_id: item.product.id, quantity: item.quantity, ttl_seconds: 15 })
        });
        const reserveData = await reserveRes.json();
        if (!reserveData.success) {
          onOptimisticRollback?.();
          setStatus('Reservation failed: ' + (reserveData.error_message || 'Item just sold out'));
          setReserving(false);
          return;
        }
        lockIds.push(reserveData.lock_id);
      }
      lockId = lockIds[0]; // Legacy
    } catch (e: any) {
      onOptimisticRollback?.();
      setStatus('Reservation error: ' + e.message);
      setReserving(false);
      return;
    }

    setStatus('Creating payment intent...');
    try {
      const res = await fetch('/api/v1/payments/terminal/intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount_cents: amount, currency: 'usd' })
      });
      const data = await res.json();

      setStatus('Collecting payment method...');
      const collectResult = await terminal.collectPaymentMethod(data.client_secret);
      if (collectResult.error) {
        onOptimisticRollback?.();
        setStatus('Payment collection failed: ' + collectResult.error.message);
        setReserving(false);
        return;
      }

      setStatus('Processing payment...');
      const processResult = await terminal.processPayment(collectResult.paymentIntent);
      if (processResult.error) {
        onOptimisticRollback?.();
        setStatus('Payment processing failed: ' + processResult.error.message);
      } else {
        setStatus('Payment successful. Committing inventory...');

        try {
          let allCommitted = true;
          const items = cart || [{product: {id: productId}, quantity: 1}];
          for (let i = 0; i < items.length; i++) {
            const item = items[i];
            const currentLockId = lockIds[i];
            const commitRes = await fetch('/api/v1/payments/terminal/commit', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                tenant_id: tenantId,
                product_id: item.product.id,
                quantity: item.quantity,
                lock_id: currentLockId,
                amount_cents: i === 0 ? amount : 0
              })
            });
            const commitData = await commitRes.json();
            if (!commitData.success) {
               allCommitted = false;
               setStatus('Payment successful, but inventory commit failed for an item: ' + commitData.error_message);
            }
          }
          if (allCommitted) {
            setStatus('Payment successful!');
            if (onSuccess) onSuccess();
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
    <div id="pos-keypad" className="p-6 rounded-3xl shadow-2xl mt-6 relative overflow-hidden bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
      <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Tap to Pay via Terminal</h2>
      <p className={`text-sm mb-6 font-medium p-3 rounded-xl border ${status?.toLowerCase()?.includes('fail') || status?.toLowerCase()?.includes('error') || status?.toLowerCase()?.includes('sold out') ? 'bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-800 border-red-200' : 'text-gray-600 border-transparent'}`}>Status: {status}</p>

      {pendingReconciliation.length > 0 && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/50 backdrop-blur-[30px] saturate-[210%] p-4">
           <div className="bg-white/85 backdrop-blur-[40px] saturate-[210%] border border-white/40 rounded-3xl p-6 shadow-2xl max-w-sm w-full text-center">
             <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Inventory Conflict Detected</h2>
             <p className="text-sm text-gray-600 mb-6">Some offline sales conflicted with online inventory. The Operations Agent has drafted an alternative offer for the online customer.</p>
             <ul className="space-y-2 mb-6">
               {pendingReconciliation.map((pr, idx) => (
                 <li key={idx} className="text-xs text-gray-800 bg-gray-100/50 p-3 rounded-xl flex justify-between border border-gray-200">
                   <span className="font-medium">Product: {pr.product_id}</span>
                   <span className="font-bold text-red-500">Shortage: {pr.shortage}</span>
                 </li>
               ))}
             </ul>
             <div className="flex flex-col gap-3">
               <button className="w-full bg-red-100 hover:bg-red-200 text-red-800 font-bold py-3 px-4 rounded-xl transition-colors active:scale-[0.98] border border-red-200 text-sm">
                 Option A: Refund in-store customer
               </button>
               <button className="w-full bg-blue-100 hover:bg-blue-200 text-blue-800 font-bold py-3 px-4 rounded-xl transition-colors active:scale-[0.98] border border-blue-200 text-sm">
                 Option B: Cancel & refund online order
               </button>
               <button onClick={() => setPendingReconciliation([])} className="w-full mt-2 text-gray-500 font-bold py-2 px-4 rounded-xl hover:bg-gray-100 transition-colors active:scale-[0.98] text-sm">
                 Decide Later
               </button>
             </div>
           </div>
        </div>
      )}

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} className="w-full bg-[#0066FF] text-white px-4 py-3 min-h-[44px] rounded-xl font-bold hover:bg-blue-700 transition-colors shadow-md shadow-blue-500/20 active:scale-[0.98]">
            Discover Readers
          </button>
          <ul className="mt-4 space-y-2">
            {discoveredReaders.map(reader => (
              <li key={reader.id} className="flex justify-between items-center p-4 border border-white/50 rounded-2xl bg-white/60 backdrop-blur-[30px] saturate-[210%] shadow-sm transition-all hover:bg-white/80">
                <span className="font-medium text-gray-800 text-sm">{reader.label || reader.id}</span>
                <button onClick={() => connectReader(reader)} className="bg-[#34C759] text-white px-5 py-2 min-h-[44px] min-w-[44px] rounded-xl text-sm font-bold shadow-sm shadow-green-500/20 hover:bg-green-600 transition-colors active:scale-[0.98]">
                  Connect
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {connectedReader && (
        <div>
          <button id="charge-btn" onClick={processPayment} disabled={reserving} className={`w-full bg-gradient-to-b from-[#0066FF] to-[#0052CC] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all charge-btn ${reserving ? 'opacity-50' : 'hover:shadow-blue-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Charge $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      )}
    </div>
  );
}
