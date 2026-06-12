"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function EmbedCheckoutGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [productName, setProductName] = useState('Premium Cake');
  const [price, setPrice] = useState('45.00');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [previewStatus, setPreviewStatus] = useState("");

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
  const embedUrl = `${baseUrl}/embed/checkout?tenant=${tenant}&theme=${theme}&product=${encodeURIComponent(productName)}&price=${encodeURIComponent(price)}`;
  const embedCode = `<iframe src="${embedUrl}" width="320" height="400" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="${baseUrl}/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=checkout_embed" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
        return {
            background: '#1D1D1F',
            color: '#ffffff',
            borderColor: '#333333'
        };
    }
    return {
        background: '#ffffff',
        color: '#1D1D1F',
        borderColor: '#E5E7EB'
    };
  };

  return (
    <div className="min-h-screen bg-gray-50 font-inter py-10 px-4 sm:px-6 lg:px-8">
      <main className="max-w-6xl mx-auto flex flex-col md:flex-row gap-8">

        {/* Configuration Panel */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div>
                <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight mb-2">Checkout Widget</h1>
                <p className="text-gray-500 text-sm">Embed a frictionless checkout flow anywhere online to convert social and blog traffic instantly.</p>
            </div>

            <div className="bg-white p-6 rounded-[20px] shadow-sm border border-gray-200">
                <h2 className="text-lg font-semibold font-outfit mb-4 text-gray-900">Configure</h2>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex bg-gray-100 p-1 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium rounded-md transition-all ${theme === 'light' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium rounded-md transition-all ${theme === 'dark' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Tenant ID</label>
                    <input
                        type="text"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. my-store"
                    />
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Product Name</label>
                    <input
                        type="text"
                        value={productName}
                        onChange={(e) => setProductName(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. Premium Cake"
                    />
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Price ($)</label>
                    <input
                        type="text"
                        value={price}
                        onChange={(e) => setPrice(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. 45.00"
                    />
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => setRemoveBranding(e.target.checked)}
                            className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                            disabled={!hasPro}
                        />
                        <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge</span>
                    </label>
                    {!hasPro && <p className="text-xs text-amber-600 mt-1">Requires Pro Plan</p>}
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-indigo-600 text-white font-medium rounded-xl hover:bg-indigo-700 transition-colors shadow-sm"
                >
                    Get Widget Code
                </button>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3 flex flex-col items-center">
            <h2 className="text-xl font-semibold font-outfit self-start mb-4" style={{ color: '#1D1D1F' }}>Live Preview</h2>
            <div className="w-full p-8 rounded-[24px] h-full flex flex-col items-center justify-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>

                <div className="relative z-10 w-[320px] h-[400px]" style={{ ...getThemeStyles(), borderRadius: '16px', boxShadow: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)' }}>
                    <div className="w-full h-40 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-t-[16px] relative flex items-center justify-center">
                        <span className="text-5xl text-white">🛍️</span>
                    </div>
                    <div className="p-5 flex flex-col h-[240px]">
                        <h4 className="font-bold text-lg font-outfit mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>{productName}</h4>
                        <p className="text-2xl font-bold mb-4" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>${price}</p>

                        <div className="mt-auto flex flex-col gap-2">
                          <button
                              type="button"
                              onClick={() => {
                                  setPreviewStatus('Checkout process initiated.');
                                  setTimeout(() => router.push('/checkout'), 1000);
                              }}
                              className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl text-sm flex items-center justify-center gap-2 transition-colors shadow-md hover:shadow-lg"
                          >
                              Buy Now
                          </button>
                          <button className="w-full py-2 bg-black hover:bg-gray-800 text-white font-semibold rounded-xl text-sm flex items-center justify-center gap-2 transition-colors">
                              Pay with Apple Pay
                          </button>
                        </div>
                        {previewStatus && <p className="mt-2 text-xs font-semibold text-indigo-600 text-center" role="status">{previewStatus}</p>}
                    </div>
                </div>
                {!removeBranding && (
                    <div className="mt-3 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                        <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=checkout_embed`} target="_blank" rel="noopener noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }} className="hover:text-indigo-600 transition-colors">
                            ⚡ Powered by OHC
                        </a>
                    </div>
                )}
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
            <div className="bg-white rounded-[24px] p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Checkout Widget</h2>
                <p className="text-gray-600 mb-6 text-sm">Copy and paste this HTML snippet into your website, blog, or Linktree.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 rounded-xl font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-indigo-600 transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-xl transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-xl transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@400;500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
