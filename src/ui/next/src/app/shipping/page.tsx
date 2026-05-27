'use client';

import React, { useState } from 'react';

export default function ShippingPage() {
  const [status, setStatus] = useState('');

  const handleBuyLabel = () => {
    setStatus('Label Generated Successfully');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header
        className="px-6 py-4 flex items-center justify-between border-b"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          borderBottom: '1px solid rgba(255, 255, 255, 0.4)',
          position: 'sticky',
          top: 0,
          zIndex: 50,
        }}
      >
        <h1
          className="text-2xl font-bold font-outfit"
          style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}
        >
          Order Shipping
        </h1>
        <button className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>
      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-8">
        <section className="bg-white rounded-2xl shadow-sm border p-6 border-gray-100">
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Order #1024 Details</h2>

          <div className="grid grid-cols-2 gap-4 mb-6 text-sm">
            <div>
              <p className="font-semibold text-gray-700">Shipping To:</p>
              <p className="text-gray-600">John Doe<br/>123 Main St<br/>San Francisco, CA 94105</p>
            </div>
            <div>
              <p className="font-semibold text-gray-700">Package Details:</p>
              <p className="text-gray-600">Weight: 2.5 lbs<br/>Dimensions: 10 x 8 x 6 in</p>
            </div>
          </div>

          <div className="bg-gray-50 p-4 rounded-lg border border-gray-200 mb-6">
            <h3 className="font-semibold text-gray-800 mb-2">Calculated Rate (via Shippo)</h3>
            <div className="flex justify-between items-center">
              <div className="flex items-center gap-2">
                <span className="font-medium text-gray-900">USPS Priority Mail</span>
                <span className="text-xs bg-green-100 text-green-800 px-2 py-1 rounded">Fastest</span>
              </div>
              <span className="font-bold text-lg text-gray-900">$8.45</span>
            </div>
          </div>

          {!status ? (
            <button
              onClick={handleBuyLabel}
              className="w-full py-3 text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2 bg-gray-900 hover:bg-black"
            >
              Buy Shipping Label
            </button>
          ) : (
            <div className="p-4 bg-green-50 text-green-800 border border-green-200 rounded-xl font-medium text-center flex flex-col items-center gap-2">
              <span className="text-2xl">✅</span>
              {status}
              <button className="mt-2 text-sm text-blue-600 hover:underline">Print Label PDF</button>
            </div>
          )}
        </section>
      </main>
    </div>
  );
}