'use client';

import React, { useState, useEffect } from 'react';

export default function CheckoutSubscriptionPage() {
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);

  const handleVaultPayment = () => {
    setLoading(true);
    setTimeout(() => {
      setLoading(false);
      setSuccess(true);
    }, 1500);
  };

  if (success) {
    return (
      <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col justify-center items-center font-inter text-center">
        <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-green-600 text-3xl mb-4">
          ✓
        </div>
        <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Subscribed!</h1>
        <p className="text-gray-500 mb-6">Your recurring payment was successful. We sent a magic link to your email to manage your subscription.</p>
      </div>
    );
  }

  return (
    <div className="p-6 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter" id="checkout-subscription-view">
      <div className="flex flex-col items-center mb-8 pt-8">
        <div className="w-20 h-20 bg-gray-200 rounded-2xl mb-4"></div>
        <h1 className="text-xl font-bold font-outfit text-gray-900">VIP Membership</h1>
        <p className="text-sm text-gray-500 mt-1">$49.00 / month</p>
      </div>

      <div className="bg-white p-4 rounded-2xl border border-gray-100 shadow-sm mb-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
        <h2 className="text-sm font-bold text-gray-900 mb-3 uppercase tracking-wider">Payment Details</h2>
        <div className="flex justify-between items-center mb-3">
          <span className="text-gray-600 text-sm">Subtotal</span>
          <span className="font-medium text-gray-900">$49.00</span>
        </div>
        <div className="flex justify-between items-center pb-3 border-b border-gray-100 mb-3">
          <span className="text-gray-600 text-sm">Taxes</span>
          <span className="font-medium text-gray-900">$0.00</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="font-bold text-gray-900">Total Due Today</span>
          <span className="font-bold text-gray-900 text-lg">$49.00</span>
        </div>
      </div>

      <button
        onClick={handleVaultPayment}
        disabled={loading}
        id="subscribe-with-apple-pay"
        className="w-full py-4 bg-black text-white rounded-xl font-bold shadow-md active:scale-95 transition-all flex justify-center items-center gap-2"
      >
        {loading ? 'Processing...' : 'Subscribe with Apple Pay'}
      </button>
      <p className="text-xs text-center text-gray-400 mt-4">
        By subscribing, you agree to the terms. Cancel anytime.
      </p>
    </div>
  );
}
