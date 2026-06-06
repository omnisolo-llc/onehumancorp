"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function SmartPricingPage() {
  const router = useRouter();
  const [enabled, setEnabled] = useState(false);
  const [discountPerishables, setDiscountPerishables] = useState(false);
  const [surgePricing, setSurgePricing] = useState(false);
  const [maxAdjustment, setMaxAdjustment] = useState(20);

  const basePrice = 10.0;
  const minPrice = (basePrice * (1 - maxAdjustment / 100)).toFixed(2);
  const maxPrice = (basePrice * (1 + maxAdjustment / 100)).toFixed(2);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Smart Pricing</h1>
        <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-6">
        <div className="text-center mb-4">
          <p className="text-lg" style={{ color: '#86868B' }}>Let AI automatically adjust your prices to maximize revenue and clear inventory, while staying within your safe limits.</p>
        </div>

        <div className="p-6 shadow-sm rounded-2xl flex items-center justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
          <div>
            <h3 className="text-lg font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Enable Smart Pricing</h3>
            <p className="text-sm text-gray-500 mt-1">Turn on autonomous hyper-local dynamic pricing.</p>
          </div>
          <button
            aria-label="Enable Smart Pricing"
            aria-pressed={enabled}
            data-testid="enable-smart-pricing-toggle"
            onClick={() => setEnabled(!enabled)}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${enabled ? 'bg-green-500' : 'bg-gray-300'}`}
          >
            <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${enabled ? 'translate-x-6' : 'translate-x-1'}`} />
          </button>
        </div>

        {enabled && (
          <div className="p-6 shadow-sm rounded-2xl flex flex-col gap-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h3 className="text-lg font-semibold font-outfit border-b pb-2" style={{ color: '#1D1D1F', borderColor: 'rgba(0,0,0,0.1)' }}>Configuration</h3>

            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium" style={{ color: '#1D1D1F' }}>Auto-discount perishables 2 hours before closing</p>
                <p className="text-xs text-gray-500 mt-1">Clear out remaining inventory today.</p>
              </div>
              <button
                aria-label="Auto-discount perishables"
                aria-pressed={discountPerishables}
                data-testid="discount-perishables-toggle"
                onClick={() => setDiscountPerishables(!discountPerishables)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${discountPerishables ? 'bg-blue-500' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${discountPerishables ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium" style={{ color: '#1D1D1F' }}>Surge pricing during high demand</p>
                <p className="text-xs text-gray-500 mt-1">Charge a premium during peak rush hours.</p>
              </div>
              <button
                aria-label="Surge pricing"
                aria-pressed={surgePricing}
                data-testid="surge-pricing-toggle"
                onClick={() => setSurgePricing(!surgePricing)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${surgePricing ? 'bg-blue-500' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${surgePricing ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>

            <div className="mt-4">
              <div className="flex justify-between items-center mb-2">
                <label className="font-medium" style={{ color: '#1D1D1F' }}>Maximum price adjustment bounds (+/-)</label>
                <span className="font-bold text-blue-600">{maxAdjustment}%</span>
              </div>
              <input
                aria-label="Maximum price adjustment bounds"
                type="range"
                min="5"
                max="50"
                step="5"
                value={maxAdjustment}
                onChange={(e) => setMaxAdjustment(parseInt(e.target.value))}
                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                data-testid="price-bounds-slider"
              />

              <div className="mt-6 p-4 rounded-xl border shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                <p className="text-sm font-semibold mb-3 text-gray-700">Preview: How a $10.00 item might fluctuate</p>
                <div className="flex justify-between items-center px-2">
                  <div className="text-center">
                    <p className="text-xs text-gray-500 mb-1">Floor</p>
                    <p className="font-bold text-green-600" data-testid="preview-min-price">${minPrice}</p>
                  </div>
                  <div className="flex-1 border-t-2 border-dashed border-gray-300 mx-4 relative">
                     <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-white px-2 text-xs font-bold text-gray-800 rounded shadow-sm border">$10.00</div>
                  </div>
                  <div className="text-center">
                    <p className="text-xs text-gray-500 mb-1">Ceiling</p>
                    <p className="font-bold text-orange-500" data-testid="preview-max-price">${maxPrice}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
