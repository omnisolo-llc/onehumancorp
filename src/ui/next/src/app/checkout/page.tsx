"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function CheckoutPage() {
  const router = useRouter();
  const [isPaid, setIsPaid] = useState(false);
  const [copied, setCopied] = useState(false);
  const [referralLink, setReferralLink] = useState("");

  useEffect(() => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    setReferralLink(`https://ohc.store/join?ref=${tenant}`);
  }, []);

  if (isPaid) {
    return (
      <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
        <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Thank You!</h1>
        </header>

        <main id="post-purchase-screen" className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
          <div className="p-8 shadow-2xl flex flex-col gap-6 items-center text-center bg-white" style={{ borderRadius: '24px', border: '1px solid rgba(0,0,0,0.05)' }}>
            <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-3xl shadow-inner text-green-600 mb-2">
              🎉
            </div>

            <h2 className="text-3xl font-bold font-outfit text-gray-900">Payment Successful</h2>
            <p className="text-gray-600 mb-2 leading-relaxed">
              Your order is confirmed and on its way! While you wait, why not share the love?
            </p>

            <div className="w-full bg-gradient-to-r from-indigo-50 to-purple-50 p-6 rounded-2xl border border-indigo-100">
              <h3 className="text-xl font-bold text-indigo-900 mb-2">Give $10, Get $10</h3>
              <p className="text-sm text-indigo-700 mb-4">
                Share this link with a friend. They get $10 off their first order, and you get $10 off your next!
              </p>

              <div className="flex gap-2 mb-4">
                <input
                  type="text"
                  readOnly
                  value={referralLink}
                  className="flex-1 bg-white border border-indigo-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none"
                />
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(referralLink);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-500 text-white' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>

              <div className="grid grid-cols-2 gap-3 w-full">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent(`I just bought something awesome! Get $10 off your first order here: ${referralLink}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-2 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I just bought something awesome! Get $10 off your first order here: ${referralLink}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-2 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  X (Twitter)
                </a>
              </div>
            </div>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full mt-2 px-4 py-3 bg-gray-100 text-gray-800 rounded-xl font-semibold hover:bg-gray-200 transition-colors"
            >
              Continue to Dashboard
            </button>
          </div>
        </main>
      </div>
    );
  }

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
              setIsPaid(true);
            }}
            className="w-full px-4 py-3 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-sm"
          >
            Pay Now
          </button>

          <button
            onClick={() => router.push('/pricing')}
            className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
          >
            Cancel
          </button>
        </div>
      </main>
    </div>
  );
}
