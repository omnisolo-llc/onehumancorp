"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function FlashSalePage() {
  const router = useRouter();
  const [product, setProduct] = useState('');
  const [discount, setDiscount] = useState('');
  const [duration, setDuration] = useState('24');
  const [isGenerating, setIsGenerating] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [generatedSnippet, setGeneratedSnippet] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleGenerate = async () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    setIsGenerating(true);

    try {
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
      const response = await fetch('/api/v1/growth/flash-sale/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ product, discount, duration, tenant }),
      });

      const data = await response.json();
      setGeneratedDraft(data.draft || '');
      setGeneratedSnippet(data.snippet || '');
    } catch (e) {
      console.error("Failed to generate flash sale", e);
    }

    setIsGenerating(false);
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI flash sales for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTimeout(() => {
      alert('Your 7-day Pro trial has been activated.');
      handleGenerate();
    }, 500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>AI Flash Sale Generator ⚡️</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-gradient-to-r from-red-50 to-orange-50 border border-red-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Drive Urgent Sales</h2>
           <p className="text-gray-600 text-sm">
             Generate a high-converting flash sale campaign with a countdown banner for your storefront to create urgency and spike revenue.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings */}
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h3 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Sale Details</h3>
              <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                  <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
              </div>
            </div>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="product-category" className="block text-sm font-medium text-gray-700 mb-1">Product or Category</label>
                <input
                  id="product-category"
                  type="text"
                  value={product}
                  onChange={(e) => setProduct(e.target.value)}
                  placeholder="e.g. Summer Collection"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-red-500"
                />
              </div>
              <div>
                <label htmlFor="discount-amount" className="block text-sm font-medium text-gray-700 mb-1">Discount Offer (%)</label>
                <input
                  id="discount-amount"
                  type="number"
                  value={discount}
                  onChange={(e) => setDiscount(e.target.value)}
                  placeholder="e.g. 30"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-red-500"
                />
              </div>
              <div>
                <label htmlFor="duration-hours" className="block text-sm font-medium text-gray-700 mb-1">Duration (Hours)</label>
                <select
                  id="duration-hours"
                  value={duration}
                  onChange={(e) => setDuration(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-red-500"
                >
                  <option value="12">12 Hours</option>
                  <option value="24">24 Hours</option>
                  <option value="48">48 Hours</option>
                  <option value="72">72 Hours</option>
                </select>
              </div>
              <button
                onClick={handleGenerate}
                disabled={!product || !discount || isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${(!product || !discount || isGenerating) ? 'bg-red-400 cursor-not-allowed' : 'bg-red-600 hover:bg-red-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? 'Generating...' : 'Launch Flash Sale'}
              </button>
            </div>
          </section>

          {/* AI Output Preview */}
          <section className="w-full md:w-1/2 flex flex-col gap-4">
            <div className="p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
              <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
                <span className="text-red-500">📣</span> Promotional Copy
              </h3>
              {generatedDraft ? (
                <div className="bg-gray-50 border border-gray-100 rounded-xl p-4">
                  <pre className="whitespace-pre-wrap text-sm text-gray-700 font-inter font-medium" style={{ fontFamily: 'inherit' }}>
                    {generatedDraft}
                  </pre>
                </div>
              ) : (
                <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                  <p className="text-sm font-medium">Your promotional copy will appear here.</p>
                </div>
              )}
            </div>

            <div className="p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
              <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
                <span className="text-orange-500">⏱️</span> Countdown Banner
              </h3>
              {generatedSnippet ? (
                <div className="flex flex-col gap-2">
                  <p className="text-sm text-gray-600 mb-2">Embed this snippet on your site to display a live countdown timer.</p>
                  <textarea
                    readOnly
                    value={generatedSnippet}
                    className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none font-mono text-xs"
                    rows={4}
                  />
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(generatedSnippet);
                      alert('Snippet copied!');
                    }}
                    className="w-full py-2 bg-gray-900 text-white rounded-lg text-sm font-semibold hover:bg-black transition-colors"
                  >
                    Copy Snippet
                  </button>
                </div>
              ) : (
                <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                  <p className="text-sm font-medium">Your embeddable banner code will appear here.</p>
                </div>
              )}
            </div>
          </section>
        </div>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
