"use client";

import React, { useEffect, useState } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';
import { SyncManager } from '../../../lib/sync/SyncManager';

interface StripeTerminalClientProps {
  amount: number;
  items: any[];
  tenantId: string;
  onSuccess?: () => void;
}

export default function StripeTerminalClient({ amount, items, tenantId, onSuccess }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<any>(null);
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);
  const [status, setStatus] = useState<string>('Ready');
  const [reserving, setReserving] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);

  useEffect(() => {
    async function initTerminal() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) return;

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const res = await fetch('/api/v1/payments/terminal/token', { method: 'POST' });
          const data = await res.json();
          return data.secret;
        },
        onUnexpectedReaderDisconnect: async () => {
          setConnectedReader(null);
          setStatus('Disconnected');
        }
      });
      setTerminal(term);
    }
    initTerminal();
  }, []);

  const discoverReaders = async () => {
    if (!terminal) return;
    setStatus('Searching...');
    const result = await terminal.discoverReaders({ simulated: true });
    if (!result.error) {
      setDiscoveredReaders(result.discoveredReaders);
      setStatus('Found ' + result.discoveredReaders.length + ' readers');
    }
  };

  const connectReader = async (reader: any) => {
    if (!terminal) return;
    const result = await terminal.connectReader(reader);
    if (!result.error) {
      setConnectedReader(result.reader);
      setStatus('Connected');

      const res = await fetch('/api/v1/payments/terminal/session/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ device_id: result.reader.id })
      });
      const data = await res.json();
      if (data.success) setSessionId(data.session_id);
    }
  };

  const processPayment = async () => {
    if (!terminal || !connectedReader) return;
    setReserving(true);
    setStatus('Processing...');

    if (!navigator.onLine) {
       // Offline processing logic
       const transactionId = `tx_offline_${Date.now()}`;
       const tx = {
          id: transactionId,
          type: 'tap_to_pay',
          amount: amount,
          currency: 'usd',
          items: items,
          payload: JSON.stringify(items),
          timestamp: new Date().toISOString()
       };
       await SyncManager.getInstance().enqueue(tx);
       setStatus('Saved Offline');
       setReserving(false);
       setTimeout(() => onSuccess?.(), 1500);
       return;
    }

    try {
      // 1. Create intent
      const res = await fetch('/api/v1/payments/terminal/intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, currency: 'usd' })
      });
      const data = await res.json();

      // 2. Collect & Process
      const collectResult = await terminal.collectPaymentMethod(data.client_secret);
      if (collectResult.error) throw new Error(collectResult.error.message);

      const processResult = await terminal.processPayment(collectResult.paymentIntent);
      if (processResult.error) throw new Error(processResult.error.message);

      // 3. Commit (Unified)
      await fetch('/api/v1/payments/terminal/commit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tenant_id: tenantId,
          items: items.map(i => ({
            product_id: i.product_id,
            quantity: i.quantity,
            amount_cents: i.price_cents * i.quantity
          })),
          amount_cents: amount,
          terminal_session_id: sessionId,
          payment_method: 'TAP_TO_PAY'
        })
      });

      setStatus('Success!');
      setTimeout(() => onSuccess?.(), 1000);
    } catch (e: any) {
      setStatus('Error: ' + e.message);
    } finally {
      setReserving(false);
    }
  };

  return (
    <div className="space-y-4">
      {!connectedReader ? (
        <div className="space-y-3">
          {discoveredReaders.length === 0 ? (
            <button onClick={discoverReaders} className="w-full bg-white text-gray-900 border border-gray-200 px-4 py-3 rounded-2xl font-bold text-sm shadow-sm">
              {status === 'Searching...' ? status : 'Setup Card Reader'}
            </button>
          ) : (
            <div className="space-y-2">
              {discoveredReaders.map(r => (
                <button key={r.id} onClick={() => connectReader(r)} className="w-full flex justify-between items-center p-4 bg-white border border-blue-100 rounded-2xl">
                  <span className="font-bold text-sm text-gray-800">{r.label || 'Reader'}</span>
                  <span className="text-[10px] bg-blue-50 text-blue-600 px-2 py-1 rounded-full uppercase font-bold tracking-tighter">Tap to Connect</span>
                </button>
              ))}
            </div>
          )}
        </div>
      ) : (
        <button
          onClick={processPayment}
          disabled={reserving}
          className="w-full bg-[#0066FF] text-white px-6 py-4 rounded-2xl font-bold text-lg shadow-xl shadow-blue-500/30 active:scale-[0.98] transition-all disabled:opacity-50"
        >
          {reserving ? status : `Charge $${(amount / 100).toFixed(2)}`}
        </button>
      )}

      {connectedReader && !reserving && (
        <p className="text-center text-[10px] text-gray-400 font-bold uppercase tracking-widest">
          Connected to {connectedReader.label}
        </p>
      )}
    </div>
  );
}
