'use client';

import React, { useState, useEffect } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';

export default function StripeTerminalClient({ amount }: { amount: number }) {
  const [terminal, setTerminal] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing...');
  const [discoveredReaders, setDiscoveredReaders] = useState<any[]>([]);
  const [connectedReader, setConnectedReader] = useState<any>(null);

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
        onUnexpectedReaderDisconnect: () => {
          setStatus('Reader disconnected unexpectedly.');
          setConnectedReader(null);
        }
      });
      setTerminal(term);
      setStatus('Terminal initialized. Ready to discover readers.');
    }
    initTerminal();
  }, []);

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
    }
  };

  const processPayment = async () => {
    if (!terminal || !connectedReader) return;
    setStatus('Creating payment intent...');
    try {
      const res = await fetch('/api/terminal/create_payment_intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, currency: 'usd' })
      });
      const data = await res.json();

      setStatus('Collecting payment method...');
      const collectResult = await terminal.collectPaymentMethod(data.client_secret);
      if (collectResult.error) {
        setStatus('Payment collection failed: ' + collectResult.error.message);
        return;
      }

      setStatus('Processing payment...');
      const processResult = await terminal.processPayment(collectResult.paymentIntent);
      if (processResult.error) {
        setStatus('Payment processing failed: ' + processResult.error.message);
      } else {
        setStatus('Payment successful!');
      }
    } catch (e: any) {
      setStatus('Error: ' + e.message);
    }
  };

  return (
    <div className="p-4 border border-white/40 rounded-[16px] shadow-sm bg-white/65 backdrop-blur-[30px] saturate-[210%]">
      <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">Stripe Terminal</h2>
      <p className="mb-4 text-gray-700 text-sm">Status: {status}</p>

      {!connectedReader && (
        <div className="mb-4">
          <button onClick={discoverReaders} className="w-full bg-[#0071E3] hover:bg-[#0066FF] text-white px-4 py-3 min-h-[44px] rounded-[8px] font-bold shadow-md shadow-[#0071E3]/20 transition-colors">
            Discover Readers
          </button>
          <ul className="mt-4 flex flex-col gap-2">
            {discoveredReaders.map(reader => (
              <li key={reader.id} className="flex justify-between items-center bg-white p-3 border border-gray-100 rounded-[8px] shadow-sm">
                <span className="font-medium text-gray-800">{reader.label || reader.id}</span>
                <button onClick={() => connectReader(reader)} className="bg-green-100 hover:bg-green-200 text-[#34C759] font-bold px-4 py-2 min-h-[44px] rounded-[8px] transition-colors">
                  Connect
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {connectedReader && (
        <div>
          <button onClick={processPayment} className="w-full bg-[#0071E3] hover:bg-[#0066FF] text-white px-4 py-4 min-h-[44px] rounded-[8px] font-bold text-lg shadow-md shadow-[#0071E3]/20 transition-colors">
            Charge ${(amount / 100).toFixed(2)}
          </button>
        </div>
      )}
    </div>
  );
}
