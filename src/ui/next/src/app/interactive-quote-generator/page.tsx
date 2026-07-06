"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function InteractiveQuoteGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [serviceName, setServiceName] = useState('Custom Cake Design');
  const [basePrice, setBasePrice] = useState(50);
  const [unitName, setUnitName] = useState('Guests');
  const [pricePerUnit, setPricePerUnit] = useState(5);
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const savedTenant = localStorage.getItem('tenant');
      if (savedTenant) setTenant(savedTenant);
    }
  }, []);

  const generatedLink = `${typeof window !== 'undefined' ? window.location.origin : ''}/quote-calculator?tenant=${tenant}&service=${encodeURIComponent(serviceName)}&basePrice=${encodeURIComponent(basePrice.toString())}&unitName=${encodeURIComponent(unitName)}&pricePerUnit=${encodeURIComponent(pricePerUnit.toString())}&theme=${theme}`;
  const iframeCode = `<iframe src="${generatedLink}" width="100%" height="400" frameborder="0" style="border-radius: 12px; border: 1px solid ${theme === 'dark' ? '#374151' : '#e5e7eb'}; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);"></iframe>
<div style="text-align:center; font-size:12px; margin-top:8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color:#6b7280;text-decoration:none;font-weight:600;font-family:sans-serif;">⚡ Powered by OHC</a></div>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(iframeCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    return theme === 'light'
        ? { background: '#ffffff', color: '#1f2937', border: '1px solid #e5e7eb' }
        : { background: '#111827', color: '#f9fafb', border: '1px solid #374151' };
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
        <div className="flex items-center gap-4">
          <button onClick={() => router.push('/dashboard')} className="p-2 hover:bg-black/5 rounded-full transition-colors text-gray-700">
            ← Back
          </button>
          <h1 className="text-2xl font-bold font-outfit text-gray-900">Interactive Quote Generator 🧮</h1>
        </div>
      </header>

      <main className="flex-1 p-6 md:p-8 max-w-7xl mx-auto w-full">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* Configuration Panel */}
            <div className="space-y-6">
                <div className="glassmorphism p-6 rounded-[24px] border border-white/40 shadow-sm bg-white/60 backdrop-blur-[30px] saturate-[210%]">
                    <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Widget Settings</h2>

                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Service Name</label>
                            <input
                                type="text"
                                value={serviceName}
                                onChange={(e) => setServiceName(e.target.value)}
                                className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none bg-white/80"
                                placeholder="e.g. Custom Cake Design"
                            />
                        </div>

                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Base Price ($)</label>
                                <input
                                    type="number"
                                    value={basePrice}
                                    onChange={(e) => setBasePrice(Number(e.target.value))}
                                    className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none bg-white/80"
                                    placeholder="50"
                                />
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Price per Unit ($)</label>
                                <input
                                    type="number"
                                    value={pricePerUnit}
                                    onChange={(e) => setPricePerUnit(Number(e.target.value))}
                                    className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none bg-white/80"
                                    placeholder="5"
                                />
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Unit Name</label>
                            <input
                                type="text"
                                value={unitName}
                                onChange={(e) => setUnitName(e.target.value)}
                                className="w-full px-4 py-2 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none bg-white/80"
                                placeholder="e.g. Guests, Hours, Pages"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                            <div className="flex gap-4">
                                <button
                                    onClick={() => setTheme('light')}
                                    className={`flex-1 py-2 px-4 rounded-xl border ${theme === 'light' ? 'border-indigo-500 bg-indigo-50 text-indigo-700 font-semibold' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'} transition-all`}
                                >
                                    Light
                                </button>
                                <button
                                    onClick={() => setTheme('dark')}
                                    className={`flex-1 py-2 px-4 rounded-xl border ${theme === 'dark' ? 'border-gray-800 bg-gray-900 text-white font-semibold' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'} transition-all`}
                                >
                                    Dark
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div className="glassmorphism p-6 rounded-[24px] border border-white/40 shadow-sm bg-white/60 backdrop-blur-[30px] saturate-[210%]">
                    <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Embed Code</h2>
                    <p className="text-sm text-gray-600 mb-4">Copy this HTML snippet to embed the interactive calculator on your website or blog.</p>
                    <div className="relative">
                        <textarea
                            readOnly
                            value={iframeCode}
                            className="w-full h-32 px-4 py-3 rounded-xl border border-gray-200 bg-gray-50 text-sm font-mono text-gray-800 outline-none resize-none"
                        />
                    </div>
                    <button
                        onClick={handleCopy}
                        className="mt-4 w-full py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold min-h-[44px] min-w-[44px] transition-colors flex items-center justify-center gap-2 shadow-md"
                    >
                        {copied ? 'Code Copied!' : 'Copy Embed Code'}
                    </button>
                </div>
            </div>

            {/* Preview Panel */}
            <div className="space-y-6">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Live Preview</h2>
                <div className="p-8 rounded-[32px] bg-gray-100/50 border-2 border-dashed border-gray-300 flex items-center justify-center min-h-[500px]">
                    <div
                        className="w-full max-w-sm rounded-2xl shadow-xl overflow-hidden transition-all duration-300"
                        style={getThemeStyles()}
                    >
                        <div className="p-6">
                            <h3 className="text-xl font-bold mb-4 font-outfit text-center">{serviceName} Quote</h3>
                            <div className="space-y-4">
                                <div className="flex justify-between items-center opacity-80">
                                    <span>Base Price</span>
                                    <span className="font-semibold">${basePrice.toFixed(2)}</span>
                                </div>
                                <div className="space-y-2">
                                    <div className="flex justify-between items-center text-sm font-medium">
                                        <label htmlFor="unit-slider">Number of {unitName}</label>
                                        <span className="text-indigo-500">10</span>
                                    </div>
                                    <input
                                        id="unit-slider"
                                        type="range"
                                        min="1"
                                        max="100"
                                        defaultValue="10"
                                        className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-indigo-600"
                                        disabled
                                    />
                                    <div className="text-right text-xs opacity-70">
                                        ${pricePerUnit.toFixed(2)} per {unitName.toLowerCase()}
                                    </div>
                                </div>
                                <div className="pt-4 border-t" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                                    <div className="flex justify-between items-end">
                                        <span className="text-lg">Estimated Total</span>
                                        <span className="text-3xl font-bold text-indigo-500">${(basePrice + (10 * pricePerUnit)).toFixed(2)}</span>
                                    </div>
                                </div>
                                <button
                                    className="w-full mt-6 py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl transition-colors disabled:opacity-50"
                                    disabled
                                >
                                    Request Quote
                                </button>
                            </div>
                        </div>
                        <div className="mt-2 py-3 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb', backgroundColor: theme === 'dark' ? '#1f2937' : '#f9fafb' }}>
                            <span className="text-xs font-semibold tracking-wide" style={{ color: '#6b7280' }}>⚡ Powered by OHC</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
      </main>
    </div>
  );
}
