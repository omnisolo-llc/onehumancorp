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

  const processCashSale = async () => {
    setReserving(true);
    setStatus('Processing cash sale...');
    onOptimisticReserve?.();

    if (!navigator.onLine) {
       setTimeout(async () => {
          const transactionId = `tx_offline_cash_${Date.now()}_${Math.floor(Math.random() * 1000)}`;
          const tx = {
             id: transactionId,
             type: 'cash_sale',
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
                    pn_counter_n_increment: -item.quantity
                 }
              }
            };
            await SyncManager.getInstance().enqueue(crdtTx);
          }
          await SyncManager.getInstance().enqueue(tx);

          setStatus('Cash sale saved offline. Will sync when network is restored.');
          setTimeout(() => {
             setStatus('Terminal ready.');
             if (onSuccess) onSuccess();
          }, 1500);
          setReserving(false);
       }, 500);
       return;
    }

    try {
        let allCommitted = true;
        let lockIds: string[] = [];
        const items = cart || [{product: {id: productId}, quantity: 1}];

        for (const item of items) {
          const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: tenantId, product_id: item.product.id, quantity: item.quantity, ttl_seconds: 15 })
          });
          const reserveData = await reserveRes.json();
          if (!reserveData.success) {
            onOptimisticRollback?.();
            setStatus('Error: Item is currently being checked out.');
            setReserving(false);

            const errorDiv = document.createElement('div');
            errorDiv.className = 'pos-error-overlay';
            errorDiv.style.position = 'fixed';
            errorDiv.style.inset = '0';
            errorDiv.style.display = 'flex';
            errorDiv.style.alignItems = 'center';
            errorDiv.style.justifyContent = 'center';
            errorDiv.style.background = 'rgba(255,255,255,0.8)';
            errorDiv.style.backdropFilter = 'blur(30px)';
            errorDiv.style.zIndex = '1000';
            errorDiv.innerHTML = `<div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 30px rgba(0,0,0,0.1); text-align: center; border: 1px solid rgba(255,59,48,0.4);">
                <h3 style="color: #FF3B30; font-family: Outfit; font-size: 1.25rem; font-weight: bold; margin-bottom: 0.5rem;">Item is currently being checked out.</h3>
                <p style="color: #666;">This item was purchased online just now.</p>
                <button onclick="this.parentElement.parentElement.remove()" style="margin-top: 1rem; padding: 0.5rem 1rem; background: #0066FF; color: white; border-radius: 0.5rem; font-weight: bold; cursor: pointer; border: none;">Got it</button>
            </div>`;
            document.body.appendChild(errorDiv);
            return;
          }
          lockIds.push(reserveData.lock_id);
        }

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
             setStatus('Cash sale successful, but inventory commit failed for an item: ' + commitData.error_message);
          }
        }
        if (allCommitted) {
          setStatus('Cash sale successful!');
          if (onSuccess) onSuccess();
        }
    } catch (e: any) {
        onOptimisticRollback?.();
        setStatus('Error processing cash sale: ' + e.message);
    } finally {
        setReserving(false);
    }
  };

  const processPayment = async () => {
    if (!terminal || !connectedReader) return;

    if (!navigator.onLine) {
       setStatus('Tap-to-Pay requires an active internet connection. Please use Cash Sale instead.');
       return;
    }

    setReserving(true);

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
          setStatus('Error: Item is currently being checked out.');
          setReserving(false);

          const errorDiv = document.createElement('div');
          errorDiv.className = 'pos-error-overlay';
          errorDiv.style.position = 'fixed';
          errorDiv.style.inset = '0';
          errorDiv.style.display = 'flex';
          errorDiv.style.alignItems = 'center';
          errorDiv.style.justifyContent = 'center';
          errorDiv.style.background = 'rgba(255,255,255,0.8)';
          errorDiv.style.backdropFilter = 'blur(30px)';
          errorDiv.style.zIndex = '1000';
          errorDiv.innerHTML = `<div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 30px rgba(0,0,0,0.1); text-align: center; border: 1px solid rgba(255,59,48,0.4);">
              <h3 style="color: #FF3B30; font-family: Outfit; font-size: 1.25rem; font-weight: bold; margin-bottom: 0.5rem;">Item is currently being checked out.</h3>
              <p style="color: #666;">This item was purchased online just now.</p>
              <button onclick="this.parentElement.parentElement.remove()" style="margin-top: 1rem; padding: 0.5rem 1rem; background: #0066FF; color: white; border-radius: 0.5rem; font-weight: bold; cursor: pointer; border: none;">Got it</button>
          </div>`;
          document.body.appendChild(errorDiv);
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
    <div id="pos-keypad" className="p-6 rounded-3xl shadow-2xl mt-6 relative overflow-hidden bg-white/70 backdrop-blur-[32px] saturate-[200%] border border-white/50">
      <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Tap to Pay via Terminal</h2>
      <p className={`text-sm mb-6 font-medium p-3 rounded-xl border ${status?.toLowerCase()?.includes('fail') || status?.toLowerCase()?.includes('error') || status?.toLowerCase()?.includes('sold out') ? 'bg-red-50/80 backdrop-blur-[30px] saturate-[210%] text-red-800 border-red-200' : 'text-gray-600 border-transparent'}`}>Status: {status}</p>

      {pendingReconciliation.length > 0 && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/50 backdrop-blur-[30px] saturate-[210%] p-4">
           <div className="bg-white/85 backdrop-blur-[40px] saturate-[210%] border border-white/50 rounded-3xl p-6 shadow-2xl max-w-sm w-full text-center">
             <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Inventory Conflict Detected</h2>
             <p className="text-sm text-gray-600 mb-6">Some offline sales conflicted with online inventory. The Operations Agent has drafted an alternative offer for the online customer.</p>
             <ul className="space-y-2 mb-6">
               {pendingReconciliation.map((pr, idx) => (
                 <li key={idx} className="text-xs text-gray-800 bg-gray-100/50 p-3 rounded-xl flex justify-between border border-gray-200">
                   <span className="font-medium">Product: {pr.product_id}</span>
                   <span className="font-bold text-[#FF3B30]">Shortage: {pr.shortage}</span>
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
        <div className="mt-4">
          <button id="tap-to-pay-btn" onClick={async () => {
            if (!terminal) return;
            if (typeof window !== 'undefined' && !navigator.onLine) {
              setStatus('Tap-to-Pay requires an active internet connection. Please use Cash Sale instead.');
              return;
            }
            setReserving(true);
            setStatus('Initializing Tap to Pay...');

            if (onOptimisticReserve) onOptimisticReserve();
            let lockIds = [];
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
                  if (onOptimisticRollback) onOptimisticRollback();
                  setStatus('Error: Item is currently being checked out.');
                  setReserving(false);
                  return;
                }
                lockIds.push(reserveData.lock_id);
              }
              lockId = lockIds[0];
            } catch (e) {
              if (onOptimisticRollback) onOptimisticRollback();
              setStatus('Reservation error: ' + e.message);
              setReserving(false);
              return;
            }

            try {
              setStatus('Discovering readers for Tap to Pay...');
              const discoverResult = await terminal.discoverReaders({ simulated: true });
              if (discoverResult.error || !discoverResult.discoveredReaders || discoverResult.discoveredReaders.length === 0) {
                if (onOptimisticRollback) onOptimisticRollback();
                setStatus('Failed to start Tap to Pay reader.');
                setReserving(false);
                return;
              }

              setStatus('Starting Tap to Pay...');
              const connectResult = await terminal.connectReader(discoverResult.discoveredReaders[0]);
              if (connectResult.error) {
                if (onOptimisticRollback) onOptimisticRollback();
                setStatus('Failed to connect to Tap to Pay reader.');
                setReserving(false);
                return;
              }
              setConnectedReader(connectResult.reader);

              setStatus('Creating payment intent...');
              const res = await fetch('/api/v1/payments/terminal/intent', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ amount_cents: amount, currency: 'usd' })
              });
              const data = await res.json();

              setStatus('Collecting payment method...');
              const collectResult = await terminal.collectPaymentMethod(data.client_secret);
              if (collectResult.error) {
                if (onOptimisticRollback) onOptimisticRollback();
                setStatus('Payment collection failed: ' + collectResult.error.message);
                setReserving(false);
                return;
              }

              setStatus('Processing payment...');
              const processResult = await terminal.processPayment(collectResult.paymentIntent);
              if (processResult.error) {
                if (onOptimisticRollback) onOptimisticRollback();
                setStatus('Payment processing failed: ' + processResult.error.message);
                setReserving(false);
                return;
              }

              setStatus('Payment successful. Committing inventory...');
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
            } catch (e) {
              setStatus('Error: ' + e.message);
            } finally {
              setReserving(false);
            }
          }} disabled={reserving || (typeof window !== 'undefined' && !navigator.onLine)} className={`w-full bg-gradient-to-b from-[#000000] to-[#333333] text-white px-4 py-4 min-h-[56px] rounded-xl font-bold text-lg transition-colors shadow-xl shadow-gray-500/20 active:scale-[0.98] ${reserving || (typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:bg-gray-800'}`}>
            {reserving ? 'Processing...' : 'Tap to Pay'}
          </button>
        </div>
      )}

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} disabled={typeof window !== 'undefined' && !navigator.onLine} className={`w-full bg-[#0066FF] text-white px-4 py-3 min-h-[44px] rounded-xl font-bold shadow-md shadow-blue-500/20 active:scale-[0.98] transition-colors ${(typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:bg-blue-700'}`}>
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

      {connectedReader ? (
        <div className="flex gap-2 mt-4">
          <button id="charge-btn" onClick={processPayment} disabled={reserving || typeof window !== 'undefined' && !navigator.onLine} className={`flex-1 bg-gradient-to-b from-[#0066FF] to-[#0052CC] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all charge-btn ${reserving || (typeof window !== 'undefined' && !navigator.onLine) ? 'opacity-50 cursor-not-allowed' : 'hover:shadow-blue-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Charge $${(amount / 100).toFixed(2)}`}
          </button>
          <button id="cash-btn" onClick={processCashSale} disabled={reserving} className={`flex-1 bg-gradient-to-b from-[#34C759] to-[#28A745] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-green-500/30 transition-all cash-btn ${reserving ? 'opacity-50' : 'hover:shadow-green-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Cash $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      ) : (
        <div className="flex gap-2 mt-4">
           <button id="cash-btn-offline" onClick={processCashSale} disabled={reserving} className={`w-full bg-gradient-to-b from-[#34C759] to-[#28A745] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-green-500/30 transition-all cash-btn ${reserving ? 'opacity-50' : 'hover:shadow-green-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
             {reserving ? 'Processing...' : `Record Cash Sale $${(amount / 100).toFixed(2)}`}
           </button>
        </div>
      )}
    </div>
  );
}
