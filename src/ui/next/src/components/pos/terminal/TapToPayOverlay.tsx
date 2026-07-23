'use client';

import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FiCheckCircle as CheckCircle2, FiLoader as Loader2, FiCreditCard as CreditCard, FiX as X } from 'react-icons/fi';
import { Button } from '@/components/ui/button';

interface TapToPayOverlayProps {
  isOpen: boolean;
  onClose: () => void;
  amount: number; // in cents
  currency: string;
  orderId?: string;
  onSuccess: (paymentIntentId: string) => void;
}

export function TapToPayOverlay({ isOpen, onClose, amount, currency, orderId, onSuccess }: TapToPayOverlayProps) {
  const [status, setStatus] = useState<'idle' | 'initializing' | 'ready' | 'processing' | 'success' | 'error'>('idle');
  const [errorMessage, setErrorMessage] = useState('');
  // In a real app we would use @stripe/terminal-js here to interact with the device secure element.
  // For the purpose of this implementation and testing, we will simulate the flow.

  useEffect(() => {
    if (isOpen) {
      setStatus('idle');
      setErrorMessage('');
    }
  }, [isOpen]);

  const handleStartPayment = async () => {
    try {
      setStatus('initializing');

      // 1. Get Connection Token
      const tokenRes = await fetch('/api/pos/terminal/connection-token', { method: 'POST' });
      if (!tokenRes.ok) throw new Error('Failed to initialize terminal.');

      // 2. Create Payment Intent
      setStatus('ready');
      const piRes = await fetch('/api/pos/terminal/payment-intent', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, currency, orderId })
      });
      if (!piRes.ok) throw new Error('Failed to create payment intent.');
      const piData = await piRes.json();

      // 3. Simulate customer tapping phone (Processing)
      setStatus('processing');
      await new Promise(resolve => setTimeout(resolve, 2000)); // Simulate NFC interaction time

      // 4. Capture Payment Intent
      const captureRes = await fetch('/api/pos/terminal/capture', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ paymentIntentId: piData.id })
      });
      if (!captureRes.ok) throw new Error('Payment capture failed.');

      setStatus('success');
      setTimeout(() => {
        onSuccess(piData.id);
      }, 1500);

    } catch (err: any) {
      console.error(err);
      setStatus('error');
      setErrorMessage(err.message || 'An unexpected error occurred.');
    }
  };

  const formatAmount = (cents: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: currency.toUpperCase() }).format(cents / 100);
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          className="fixed inset-0 z-50 flex items-end justify-center bg-black/40 backdrop-blur-sm sm:items-center"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          <motion.div
            className="w-full max-w-[375px] bg-white rounded-t-3xl sm:rounded-3xl p-6 shadow-2xl relative"
            initial={{ y: '100%' }}
            animate={{ y: 0 }}
            exit={{ y: '100%' }}
            transition={{ type: "spring", bounce: 0, duration: 0.4 }}
            data-testid="tap-to-pay-overlay"
          >
            <button
              onClick={onClose}
              disabled={status === 'processing'}
              className="absolute right-4 top-4 p-2 text-gray-500 hover:bg-gray-100 rounded-full disabled:opacity-50"
            >
              <X className="w-5 h-5" />
            </button>

            <div className="flex flex-col items-center pt-4 pb-8 space-y-6">

              {status === 'idle' && (
                <>
                  <div className="w-20 h-20 bg-blue-50 rounded-full flex items-center justify-center">
                    <CreditCard className="w-10 h-10 text-blue-600" />
                  </div>
                  <div className="text-center space-y-2">
                    <h2 className="text-2xl font-semibold text-gray-900">Total: {formatAmount(amount)}</h2>
                    <p className="text-gray-500 text-sm">Ready to accept payment</p>
                  </div>
                  <Button
                    size="lg"
                    className="w-full h-14 text-lg rounded-2xl bg-blue-600 hover:bg-blue-700 text-white shadow-lg shadow-blue-200"
                    onClick={handleStartPayment}
                  >
                    Accept Payment
                  </Button>
                </>
              )}

              {(status === 'initializing' || status === 'ready' || status === 'processing') && (
                <>
                  <div className="relative w-32 h-32 flex items-center justify-center">
                    <motion.div
                      className="absolute inset-0 border-4 border-blue-100 rounded-full"
                      animate={{ scale: [1, 1.2, 1], opacity: [0.5, 0.2, 0.5] }}
                      transition={{ duration: 2, repeat: Infinity }}
                    />
                    <div className="w-20 h-20 bg-blue-50 rounded-full flex items-center justify-center z-10 relative">
                      <CreditCard className="w-10 h-10 text-blue-600" />
                    </div>
                  </div>
                  <div className="text-center space-y-2">
                    <h2 className="text-3xl font-semibold text-gray-900">{formatAmount(amount)}</h2>
                    <div className="flex items-center justify-center space-x-2 text-blue-600 font-medium">
                      {status === 'processing' ? (
                        <>
                          <Loader2 className="w-5 h-5 animate-spin" />
                          <span>Processing payment...</span>
                        </>
                      ) : (
                        <span className="animate-pulse">Hold card or phone near reader</span>
                      )}
                    </div>
                  </div>
                </>
              )}

              {status === 'success' && (
                <>
                  <motion.div
                    initial={{ scale: 0 }}
                    animate={{ scale: 1 }}
                    className="w-24 h-24 bg-green-50 rounded-full flex items-center justify-center text-green-500"
                  >
                    <CheckCircle2 className="w-14 h-14" />
                  </motion.div>
                  <div className="text-center space-y-2">
                    <h2 className="text-2xl font-semibold text-gray-900">Payment Successful!</h2>
                    <p className="text-green-600 font-medium">{formatAmount(amount)}</p>
                  </div>
                </>
              )}

              {status === 'error' && (
                <>
                  <div className="w-20 h-20 bg-red-50 rounded-full flex items-center justify-center">
                    <X className="w-10 h-10 text-red-500" />
                  </div>
                  <div className="text-center space-y-2">
                    <h2 className="text-xl font-semibold text-gray-900">Payment Failed</h2>
                    <p className="text-red-500 text-sm px-4">{errorMessage}</p>
                  </div>
                  <Button
                    variant="outline"
                    className="w-full h-12"
                    onClick={() => setStatus('idle')}
                  >
                    Try Again
                  </Button>
                </>
              )}

            </div>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
