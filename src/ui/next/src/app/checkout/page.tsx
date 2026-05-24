"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

export default function CheckoutPage() {
  const router = useRouter();

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Checkout</h1>
      </header>

      <main id="checkout-screen" className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        <p className="text-gray-700">Please enter your payment details below.</p>

        <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <p className="text-sm text-gray-600">100% money back guarantee. Secure SSL payments.</p>

          <button
            onClick={() => {
              alert('Payment successful!');
              router.push('/dashboard');
            }}
            className="w-full px-4 py-3 bg-indigo-600 text-white rounded-[8px] font-medium hover:bg-indigo-700 transition-colors shadow-sm"
          >
            Pay Now
          </button>

          <button
            onClick={() => router.push('/pricing')}
            className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-[8px] font-medium hover:bg-gray-300 transition-colors"
          >
            Cancel
          </button>
        </div>
      </main>
    </div>
  );
}
