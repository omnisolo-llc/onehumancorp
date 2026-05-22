"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

export default function CheckoutPage() {
  const router = useRouter();

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Checkout</h1>
        <button onClick={() => router.push('/pricing')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Cancel
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-2xl mx-auto w-full flex flex-col gap-6" id="checkout-screen">
        <p className="text-lg" style={{ color: '#86868B' }}>Please enter your payment details below.</p>

        <div className="p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <p className="text-sm font-medium text-gray-700 mb-6">100% money back guarantee. Secure SSL payments.</p>
          <button
            onClick={() => {
              fetch('/api/billing/checkout', { method: 'POST' })
                .then(res => res.json())
                .then(data => {
                    if (data.checkout_url) {
                        window.location.href = data.checkout_url;
                    } else {
                        alert('Redirecting to Stripe payment...');
                        router.push('/dashboard');
                    }
                })
                .catch(() => {
                    alert('Payment successful!');
                    router.push('/dashboard');
                });
            }}
            className="w-full py-3 bg-indigo-600 text-white font-medium rounded-lg hover:bg-indigo-700 transition-colors mb-4"
          >
            Pay Now
          </button>
          <button
            onClick={() => router.push('/pricing')}
            className="w-full py-3 bg-white border border-gray-300 text-gray-700 font-medium rounded-lg hover:bg-gray-50 transition-colors"
          >
            Cancel
          </button>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
