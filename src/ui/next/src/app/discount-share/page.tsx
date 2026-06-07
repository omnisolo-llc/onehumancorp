"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function DiscountSharePage() {
  const router = useRouter();
  const [discountTitle, setDiscountTitle] = useState('Weekend Sale');
  const [discountAmount, setDiscountAmount] = useState('20%');
  const [theme, setTheme] = useState<'light' | 'dark'>('dark');
  const [tenant, setTenant] = useState('my-store');
  const [copied, setCopied] = useState(false);
  const [showPreview, setShowPreview] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
    }
  }, []);

  const ogCardUrl = `/api/v1/growth/discount_share/og-card?tenant=${tenant}&title=${encodeURIComponent(discountTitle)}&amount=${encodeURIComponent(discountAmount)}&theme=${theme}`;
  const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
  const absoluteOgCardUrl = `${baseUrl}${ogCardUrl}`;
  const shareUrl = `${baseUrl}/share-card?url=${encodeURIComponent(`/storefront?tenant=${tenant}`)}&title=${encodeURIComponent(discountTitle)}&image=${encodeURIComponent(absoluteOgCardUrl)}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(shareUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-sans">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-50">
        <h1 className="text-xl font-bold text-gray-900 tracking-tight flex items-center gap-2">
          <span>🎁</span> Discount Share
        </h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="flex-1 max-w-5xl w-full mx-auto p-6 md:p-10 grid md:grid-cols-2 gap-10">
        <div className="space-y-8">
          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-bold text-gray-900 mb-6 flex items-center gap-2">
              <span>✏️</span> Create Discount Link
            </h2>

            <div className="space-y-5">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Occasion / Title</label>
                <input
                  type="text"
                  value={discountTitle}
                  onChange={(e) => setDiscountTitle(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-shadow"
                  placeholder="e.g. Summer Blowout"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Discount Amount</label>
                <input
                  type="text"
                  value={discountAmount}
                  onChange={(e) => setDiscountAmount(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-shadow"
                  placeholder="e.g. 15% OFF"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                <div className="flex gap-3">
                  <button
                    onClick={() => setTheme('light')}
                    className={`flex-1 py-3 rounded-xl border font-medium transition-all ${theme === 'light' ? 'bg-indigo-50 border-indigo-200 text-indigo-700 ring-2 ring-indigo-500 ring-offset-1' : 'bg-white border-gray-200 text-gray-600 hover:bg-gray-50'}`}
                  >
                    Light
                  </button>
                  <button
                    onClick={() => setTheme('dark')}
                    className={`flex-1 py-3 rounded-xl border font-medium transition-all ${theme === 'dark' ? 'bg-gray-900 border-gray-800 text-white ring-2 ring-gray-900 ring-offset-1' : 'bg-white border-gray-200 text-gray-600 hover:bg-gray-50'}`}
                  >
                    Dark
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-2xl shadow-sm border border-gray-200 p-6">
            <h2 className="text-lg font-bold text-gray-900 mb-4 flex items-center gap-2">
              <span>🔗</span> Share Link
            </h2>
            <p className="text-sm text-gray-500 mb-4">Copy this link to share on social media. It will generate a beautiful preview card automatically.</p>

            <div className="flex items-center gap-3 bg-gray-50 p-2 rounded-xl border border-gray-200">
              <input
                type="text"
                readOnly
                value={shareUrl}
                className="flex-1 bg-transparent border-none focus:ring-0 text-sm text-gray-600 truncate px-2 outline-none"
              />
              <button
                onClick={handleCopy}
                className={`shrink-0 px-4 py-2 rounded-lg text-sm font-bold transition-all ${copied ? 'bg-green-500 text-white' : 'bg-gray-900 text-white hover:bg-black'}`}
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>
          </div>
        </div>

        <div>
          <div className="sticky top-24">
            <h2 className="text-lg font-bold text-gray-900 mb-4 flex items-center gap-2">
              <span>👀</span> Social Preview
            </h2>

            {/* Glassmorphism Preview Card Container */}
            <div className="app-card overflow-hidden rounded-2xl border shadow-xl transition-all" style={{
              background: theme === 'dark' ? 'rgba(17, 24, 39, 0.85)' : 'rgba(255, 255, 255, 0.85)',
              backdropFilter: 'blur(20px) saturate(200%)',
              borderColor: theme === 'dark' ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)',
            }}>

              {/* Simulated OG Image Area */}
              <div
                className="w-full aspect-[1200/630] flex flex-col items-center justify-center relative overflow-hidden"
                style={{
                  background: theme === 'dark' ? 'linear-gradient(135deg, #1f2937 0%, #111827 100%)' : 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)',
                }}
              >
                {/* Decorative background elements */}
                <div className="absolute top-[-20%] left-[-10%] w-[50%] h-[50%] rounded-full opacity-20 blur-[60px]" style={{ background: theme === 'dark' ? '#6366f1' : '#818cf8' }}></div>
                <div className="absolute bottom-[-20%] right-[-10%] w-[60%] h-[60%] rounded-full opacity-20 blur-[60px]" style={{ background: theme === 'dark' ? '#8b5cf6' : '#a78bfa' }}></div>

                <div className="relative z-10 text-center px-8">
                  <h3
                    className="text-4xl sm:text-5xl font-black mb-4 tracking-tight drop-shadow-sm"
                    style={{ color: theme === 'dark' ? '#ffffff' : '#111827', fontFamily: 'Outfit, sans-serif' }}
                  >
                    {discountTitle || 'Special Offer'}
                  </h3>

                  <div className="inline-block px-6 py-2 rounded-full mb-6" style={{ background: theme === 'dark' ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.05)', backdropFilter: 'blur(10px)' }}>
                    <span
                      className="text-3xl font-bold"
                      style={{ color: theme === 'dark' ? '#4ade80' : '#16a34a' }}
                    >
                      {discountAmount || '10%'} OFF
                    </span>
                  </div>
                </div>

                <div className="absolute bottom-6 w-full flex justify-center">
                  <div className="px-4 py-2 rounded-xl flex items-center gap-2" style={{ background: theme === 'dark' ? 'rgba(0,0,0,0.5)' : 'rgba(255,255,255,0.8)', backdropFilter: 'blur(10px)', border: `1px solid ${theme === 'dark' ? 'rgba(255,255,255,0.1)' : 'rgba(0,0,0,0.1)'}` }}>
                    <span className="text-sm font-bold uppercase tracking-wider" style={{ color: theme === 'dark' ? '#e5e7eb' : '#374151' }}>
                      ⚡ Powered by OHC
                    </span>
                  </div>
                </div>
              </div>

              {/* Simulated Twitter/FB Post Text Area */}
              <div className="p-4 sm:p-5 border-t" style={{ borderColor: theme === 'dark' ? 'rgba(255, 255, 255, 0.05)' : 'rgba(0, 0, 0, 0.05)' }}>
                <p className="text-sm font-medium mb-1 truncate" style={{ color: theme === 'dark' ? '#9ca3af' : '#6b7280' }}>ohc.app</p>
                <p className="text-base font-bold truncate" style={{ color: theme === 'dark' ? '#f3f4f6' : '#111827' }}>Claim your {discountAmount} discount today!</p>
                <p className="text-sm truncate mt-1" style={{ color: theme === 'dark' ? '#6b7280' : '#4b5563' }}>Exclusive offer from {tenant}. Shop now and save big on your next purchase.</p>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Global styles for the app-card classes used in tests */}
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-sans { font-family: 'Inter', sans-serif; }
      `}} />
    </div>
  );
}
