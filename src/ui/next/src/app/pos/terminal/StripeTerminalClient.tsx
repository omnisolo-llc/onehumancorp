"use client";

import React, { useEffect, useState } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';
import { SyncManager } from '../../../lib/sync/SyncManager';

interface StripeTerminalClientProps {
  amount: number;
  productId: string;
  tenantId: string;
  onOptimisticReserve?: () => void;
  onOptimisticRollback?: () => void;
}

export default function StripeTerminalClient({ amount, productId, tenantId, onOptimisticReserve, onOptimisticRollback }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<any>(null);
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [reserving, setReserving] = useState(false);
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

    if (typeof window !== 'undefined') {
        window.addEventListener('online', handleOnline);
        window.addEventListener('offline', handleOffline);
        return () => {
          window.removeEventListener('online', handleOnline);
          window.removeEventListener('offline', handleOffline);
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
             product_id: productId,
             quantity: 1,
             payload: JSON.stringify([{ product_id: productId, quantity: 1 }]),
             timestamp: new Date().toISOString()
          };

          const crdtTx = {
            id: `crdt_${transactionId}`,
            type: 'CRDT_MUTATION',
            timestamp: new Date().toISOString(),
            payload: {
               entity_id: productId,
               data: {
                  pn_counter_n_increment: 1
               }
            }
          };

          await SyncManager.getInstance().enqueue(tx);
          await SyncManager.getInstance().enqueue(crdtTx);

          setStatus('Synced locally. Will push to cloud when network is restored.');
          setTimeout(() => setStatus('Terminal ready.'), 3000);
          setReserving(false);
       }, 1500);
       return;
    }

    setStatus('Reserving inventory...');
    onOptimisticReserve?.();

    let lockId = '';
    try {
      const reserveRes = await fetch('/api/v1/payments/terminal/reserve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: tenantId, product_id: productId, quantity: 1, ttl_seconds: 15 })
      });
      const reserveData = await reserveRes.json();

      if (!reserveData.success) {
        onOptimisticRollback?.();
        setStatus('Reservation failed: ' + (reserveData.error_message || 'Item is currently being purchased elsewhere'));
        setReserving(false);
        return;
      }
      lockId = reserveData.lock_id;
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
        body: JSON.stringify({ amount, currency: 'usd' })
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
          const commitRes = await fetch('/api/v1/payments/terminal/commit', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              tenant_id: tenantId,
              product_id: productId,
              quantity: 1,
              lock_id: lockId,
              amount_cents: amount
            })
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
    <div className="p-6 border border-white/40 rounded-3xl shadow-2xl bg-white/40 backdrop-blur-[40px] saturate-[200%] mt-6 relative overflow-hidden ring-1 ring-black/5">
      <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Tap to Pay via Terminal</h2>
      <p className="text-sm text-gray-600 mb-6 font-medium">Status: {status}</p>

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} className="w-full bg-[#0066FF] text-white px-4 py-3 min-h-[44px] rounded-xl font-bold hover:bg-blue-700 transition-colors shadow-md shadow-blue-500/20 active:scale-[0.98]">
            Discover Readers
          </button>
          <ul className="mt-4 space-y-2">
            {discoveredReaders.map(reader => (
              <li key={reader.id} className="flex justify-between items-center p-4 border border-white/50 rounded-2xl bg-white/60 backdrop-blur-md shadow-sm transition-all hover:bg-white/80">
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
          <button onClick={processPayment} disabled={reserving} className={`w-full bg-gradient-to-b from-[#0066FF] to-[#0052CC] text-white px-6 py-4 min-h-[56px] rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 transition-all charge-btn ${reserving ? 'opacity-50' : 'hover:shadow-blue-500/40 hover:scale-[1.02] active:scale-[0.98]'}`}>
            {reserving ? 'Processing...' : `Collect Payment $${(amount / 100).toFixed(2)}`}
          </button>
        </div>
      )}
    </div>
  );
}
