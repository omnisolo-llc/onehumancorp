'use client';

import React, { useState, useEffect, useRef } from 'react';
import Link from 'next/link';

export default function UnboxingInsertsPage() {
  const [discountCode, setDiscountCode] = useState('WELCOME10');
  const [discountAmount, setDiscountAmount] = useState('10%');
  const [referralLink, setReferralLink] = useState('ohc.store/join?ref=my-store');
  const [tenant, setTenant] = useState('my-store');
  const insertRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setReferralLink(`ohc.store/join?ref=${storedTenant}`);
    }
  }, []);

  const handlePrint = () => {
    window.print();
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter text-gray-900 overflow-x-hidden">
      {/* Header */}
      <header className="px-5 pt-8 pb-4 bg-white/80 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-100">
        <div className="flex justify-between items-center mb-4">
          <Link href="/dashboard" className="text-gray-400 hover:text-gray-600 transition-colors" aria-label="Back to Dashboard">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
        </div>
        <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Printable Inserts</h1>
        <p className="text-sm text-gray-500 mt-1">Generate viral packaging inserts for your orders</p>
      </header>

      <main className="px-5 py-6 pb-24">
        {/* Configuration Section */}
        <div className="bg-white/60 backdrop-blur-[20px] rounded-2xl p-5 shadow-sm border border-white/40 mb-8">
          <h2 className="text-lg font-bold font-outfit mb-4">Customize Your Insert</h2>

          <div className="space-y-4">
            <div>
              <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-1">Discount Amount</label>
              <input
                type="text"
                value={discountAmount}
                onChange={(e) => setDiscountAmount(e.target.value)}
                className="w-full border border-gray-200 rounded-lg p-3 text-sm focus:ring-2 focus:ring-[#0071E3] outline-none"
              />
            </div>

            <div>
              <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-1">Discount Code</label>
              <input
                type="text"
                value={discountCode}
                onChange={(e) => setDiscountCode(e.target.value)}
                className="w-full border border-gray-200 rounded-lg p-3 text-sm focus:ring-2 focus:ring-[#0071E3] outline-none font-mono"
              />
            </div>
          </div>
        </div>

        {/* Preview Section */}
        <div className="mb-8">
          <h2 className="text-lg font-bold font-outfit mb-4">Preview</h2>

          <div className="overflow-hidden rounded-2xl shadow-lg border border-gray-200 bg-white" id="insert-preview">
            {/* The actual printable area */}
            <div
              ref={insertRef}
              className="w-full aspect-[1/1.4] p-8 flex flex-col justify-between items-center text-center relative overflow-hidden bg-gradient-to-br from-indigo-50 to-purple-50"
              style={{
                backgroundImage: 'url("data:image/svg+xml,%3Csvg width=\'60\' height=\'60\' viewBox=\'0 0 60 60\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cg fill=\'none\' fill-rule=\'evenodd\'%3E%3Cg fill=\'%234f46e5\' fill-opacity=\'0.05\'%3E%3Cpath d=\'M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z\'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E")',
              }}
            >
              <div className="mt-8">
                <h3 className="text-3xl font-black font-outfit text-gray-900 tracking-tighter mb-2 leading-tight">Thank You!</h3>
                <p className="text-gray-600 text-sm">We appreciate your support.</p>
              </div>

              <div className="bg-white/80 backdrop-blur-md p-6 rounded-2xl shadow-sm border border-white w-full max-w-[280px]">
                <p className="text-xs font-bold text-gray-500 uppercase tracking-widest mb-2">Next Order</p>
                <div className="text-4xl font-black text-indigo-600 mb-2">{discountAmount} OFF</div>
                <div className="inline-block bg-gray-100 text-gray-800 px-4 py-2 rounded-lg font-mono font-bold tracking-wider border border-gray-200 shadow-inner">
                  {discountCode}
                </div>
              </div>

              <div className="mb-6 w-full">
                <div className="h-px w-full bg-gradient-to-r from-transparent via-gray-300 to-transparent mb-6"></div>
                <div className="flex flex-col items-center justify-center gap-1">
                  <span className="text-xs text-gray-500 font-medium">Built with love on</span>
                  <div className="font-outfit font-black text-xl tracking-tighter bg-clip-text text-transparent bg-gradient-to-r from-indigo-600 to-purple-600">
                    OneHumanCorp
                  </div>
                  <div className="mt-3 text-[10px] text-gray-400 font-mono bg-white/50 px-3 py-1 rounded-full border border-gray-100">
                    {referralLink}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Floating Action Button */}
      <div className="fixed bottom-0 left-0 right-0 p-5 bg-white/80 backdrop-blur-[30px] saturate-[210%] border-t border-gray-100 z-30 flex gap-4">
        <button
          onClick={handlePrint}
          className="flex-1 bg-indigo-600 text-white font-bold py-4 rounded-xl shadow-lg shadow-indigo-200 transition-transform active:scale-95 flex justify-center items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" /></svg>
          Print Insert
        </button>
      </div>

      <style jsx global>{`
        @media print {
          body * {
            visibility: hidden;
          }
          #insert-preview, #insert-preview * {
            visibility: visible;
          }
          #insert-preview {
            position: absolute;
            left: 0;
            top: 0;
            width: 100%;
            height: 100vh;
            border: none;
            box-shadow: none;
            border-radius: 0;
          }
          @page {
            size: A6 portrait;
            margin: 0;
          }
        }
      `}</style>
    </div>
  );
}
