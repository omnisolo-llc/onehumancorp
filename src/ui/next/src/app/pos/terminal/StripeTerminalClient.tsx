"use client";
import React, { useState, useEffect } from 'react';
import { loadStripeTerminal } from '@stripe/terminal-js';

interface StripeTerminalClientProps {
  amount: number;
  productId?: string;
  tenantId: string;
  onOptimisticReserve?: () => void;
  onOptimisticRollback?: () => void;
  onSuccess?: () => void;
}

export default function StripeTerminalClient({ amount, productId, tenantId, onOptimisticReserve, onOptimisticRollback, onSuccess }: StripeTerminalClientProps) {
  const [terminal, setTerminal] = useState<any>(null);
  const [status, setStatus] = useState<string>('Initializing Terminal...');
  const [isProcessing, setIsProcessing] = useState(false);
  const [hasNfc, setHasNfc] = useState<boolean | null>(null);

  useEffect(() => {
    async function init() {
      const StripeTerminal = await loadStripeTerminal();
      if (!StripeTerminal) {
        setStatus('Failed to load Stripe Terminal script.');
        setHasNfc(false);
        return;
      }

      const term = StripeTerminal.create({
        onFetchConnectionToken: async () => {
          const response = await fetch('/api/v1/terminal/connection_token', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant_id: tenantId }),
          });
          const data = await response.json();
          if (data.secret) {
            return data.secret;
          }
          throw new Error('Could not fetch connection token');
        },
        onUnexpectedReaderDisconnect: () => {
          setStatus('Disconnected from reader unexpectedly.');
        }
      });
      setTerminal(term);

      // Check for hardware capability here
      // This is a naive check; real implementation requires Stripe SDK specifics for Tap To Pay capability discovery
      // Assuming Tap to Pay is available for demonstration if terminal is loaded
      setHasNfc(true);
      setStatus('Ready for Tap to Pay');
    }
    init();
  }, [tenantId]);

  const handleTapToPay = async () => {
    if (!terminal) return;
    setIsProcessing(true);
    setStatus('Creating payment intent...');
    if (onOptimisticReserve) onOptimisticReserve();

    try {
      const response = await fetch('/api/v1/terminal/payment_intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount_cents: amount, currency: 'usd', tenant_id: tenantId, product_id: productId }),
      });
      const data = await response.json();

      if (!response.ok || !data.client_secret) {
        throw new Error(data.error || 'Failed to create payment intent');
      }

      setStatus('Waiting for tap...');

      const { error, paymentIntent } = await terminal.collectPaymentMethod(data.client_secret);

      if (error) {
        throw new Error(error.message);
      }

      setStatus('Processing payment...');
      const { error: processError } = await terminal.processPayment(paymentIntent);

      if (processError) {
        throw new Error(processError.message);
      }

      setStatus('Payment successful!');
      if (onSuccess) onSuccess();

    } catch (err: any) {
      console.error(err);
      setStatus(`Error: ${err.message}`);
      if (onOptimisticRollback) onOptimisticRollback();
    } finally {
      setIsProcessing(false);
    }
  };

  if (hasNfc === false) {
    return <div className="text-red-500 text-sm">Tap to Pay is not supported on this device.</div>;
  }

  return (
    <div className="flex flex-col items-center justify-center p-4 border rounded-xl bg-white shadow-sm mt-4">
      <div className="text-sm text-gray-500 mb-4">{status}</div>
      <button
        onClick={handleTapToPay}
        disabled={isProcessing || !terminal}
        className="w-full bg-black text-white px-4 py-3 rounded-lg font-medium hover:bg-gray-800 disabled:bg-gray-400 flex items-center justify-center gap-2"
      >
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
        {isProcessing ? 'Processing...' : 'Tap to Pay on Phone'}
      </button>
    </div>
  );
}
