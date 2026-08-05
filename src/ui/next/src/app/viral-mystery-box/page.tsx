"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import '../globals.css';
import { PoweredByOHC } from "../components/PoweredByOHC";

export default function ViralMysteryBoxPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [boxName, setBoxName] = useState('Ultimate Tech Mystery Box');
  const [boxPrice, setBoxPrice] = useState('49.99');
  const [boxValue, setBoxValue] = useState('150.00');
  const [discountAmount, setDiscountAmount] = useState('15');
  const [copied, setCopied] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name') || 'my-business';
      setTenant(storedTenant);
    }
  }, []);

  const generatedLink = `https://ohc.app/mysterybox/${tenant.toLowerCase().replace(/\s+/g, '-')}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleSave = async () => {
    try {
      await fetch('/api/v1/growth/viral-mystery-box', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ boxName, boxPrice, boxValue, discountAmount }),
      });
      // Handle success notification
    } catch (error) {
      // Handle error notification
      console.error(error);
    }
  };

  if (!isClient) return null;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Mystery Box 🎁</h1>
        <div className="flex gap-4">
          <button
            onClick={handleSave}
            className="px-4 py-2 bg-indigo-600 text-white min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-indigo-700 transition-colors rounded-lg"
          >
            Save Configuration
          </button>
          <button
            onClick={() => router.push('/dashboard')}
            className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors rounded-lg"
          >
            Back to Dashboard
          </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[40px] saturate-[210%] border border-[rgba(255,255,255,0.4)] rounded-2xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Mystery Box Setup</h2>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Mystery Box Title</label>
              <input
                type="text"
                value={boxName}
                onChange={(e) => setBoxName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="e.g. Secret Sneaker Box"
              />
            </div>

            <div className="mb-4 flex gap-4">
              <div className="flex-1">
                <label className="block text-sm font-medium text-gray-700 mb-2">Price ($)</label>
                <input
                  type="number"
                  step="0.01"
                  value={boxPrice}
                  onChange={(e) => setBoxPrice(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                  placeholder="29.99"
                />
              </div>
              <div className="flex-1">
                <label className="block text-sm font-medium text-gray-700 mb-2">Est. Value ($)</label>
                <input
                  type="number"
                  step="0.01"
                  value={boxValue}
                  onChange={(e) => setBoxValue(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                  placeholder="100.00"
                />
              </div>
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Unboxing Share Discount (%)</label>
              <p className="text-xs text-gray-500 mb-2">Reward customers for sharing their unboxing video on social media.</p>
              <input
                type="number"
                min="0"
                max="100"
                value={discountAmount}
                onChange={(e) => setDiscountAmount(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="15"
              />
            </div>
          </div>

          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-indigo-50/70 backdrop-blur-[40px] border border-indigo-100/50 rounded-2xl">
            <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
              <span className="text-xl">🚀</span> Launch & Share
            </h3>
            <p className="text-sm text-indigo-800 mb-4">
              Copy your unique link to promote your Mystery Box on social media, in emails, or on your storefront.
            </p>

            <div className="flex items-center gap-2 bg-white rounded-lg border border-indigo-200 p-2 mb-4 overflow-hidden">
              <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono">
                {generatedLink}
              </div>
            </div>

            <button
              onClick={handleCopy}
              className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-lg min-h-[44px] transition-colors"
            >
              {copied ? 'Copied!' : 'Copy Link'}
            </button>
          </div>
        </div>

        <div className="w-full md:w-2/3 flex flex-col">
          <div className="flex-1 shadow-[0_20px_40px_rgb(0,0,0,0.15)] overflow-hidden flex flex-col bg-white/40 backdrop-blur-[40px] saturate-[210%] border border-white/50 rounded-2xl relative min-h-[600px]">
            <div className="bg-gray-200 py-3 px-4 flex items-center gap-2 border-b border-gray-300">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="mx-auto bg-white/60 text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate">
                Mobile Preview: Mystery Box
              </div>
            </div>

            <div className="flex-1 flex items-center justify-center p-8 bg-[#111] overflow-y-auto relative">
              <div
                className="w-full max-w-sm shadow-2xl p-0 flex flex-col relative overflow-hidden transition-all duration-300 rounded-3xl bg-white min-h-[600px]"
              >
                <div className="h-64 bg-gradient-to-br from-indigo-900 via-purple-900 to-black relative flex items-center justify-center">
                  <div className="absolute inset-0 opacity-30 mix-blend-overlay bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI4IiBoZWlnaHQ9IjgiPjxyZWN0IHdpZHRoPSI4IiBoZWlnaHQ9IjgiIGZpbGw9IiNmZmYiIGZpbGwtb3BhY2l0eT0iLjEiLz48cGF0aCBkPSJNMCAwbDhfOFpNOCAwTDAgOFoiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLW9wYWNpdHk9Ii4xIi8+PC9zdmc+')]"></div>
                  <div className="text-8xl animate-bounce drop-shadow-2xl z-10 relative">
                    🎁
                  </div>
                  <div className="absolute bottom-4 right-4 bg-yellow-400 text-yellow-900 text-xs font-bold px-3 py-1 rounded-full shadow-lg z-10 transform rotate-[-5deg]">
                    ${boxValue} Value!
                  </div>
                </div>

                <div className="p-6 flex flex-col flex-1">
                  <div className="flex justify-between items-start mb-2">
                    <h2 className="text-2xl font-bold font-outfit text-gray-900 leading-tight">
                      {boxName || 'Mystery Box'}
                    </h2>
                    <span className="text-xl font-bold text-indigo-600">${boxPrice || '0'}</span>
                  </div>

                  <p className="text-sm text-gray-500 mb-6 font-medium">
                    What's inside? It's a secret! But we guarantee items worth at least ${boxValue}.
                  </p>

                  <div className="bg-orange-50 border border-orange-200 rounded-xl p-4 mb-6 relative overflow-hidden">
                    <div className="absolute -right-4 -top-4 text-4xl opacity-10">📸</div>
                    <h3 className="font-bold text-orange-800 text-sm mb-1 flex items-center gap-2">
                      <span className="text-lg">✨</span> Viral Unboxing Bonus
                    </h3>
                    <p className="text-xs text-orange-700">
                      Film your unboxing reaction on TikTok or Instagram, tag <span className="font-semibold text-orange-900">@{tenant.replace(/\s+/g, '')}</span>, and get <span className="font-bold text-orange-900 bg-orange-200 px-1 rounded">{discountAmount}% OFF</span> your next purchase!
                    </p>
                  </div>

                  <div className="mt-auto space-y-3">
                    <button className="w-full py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl min-h-[56px] transition-all flex items-center justify-center gap-2 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5">
                      Buy Mystery Box Now
                    </button>
                    <button className="w-full py-3 bg-gray-100 hover:bg-gray-200 text-gray-700 font-bold rounded-xl min-h-[44px] transition-all flex items-center justify-center gap-2">
                      See Past Reveals
                    </button>
                  </div>

                  <div className="mt-6 border-t border-gray-100 pt-4 flex justify-center">
                    <PoweredByOHC tenantId="growth" />
                  </div>
                </div>
              </div>
            </div>
          </div>
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
