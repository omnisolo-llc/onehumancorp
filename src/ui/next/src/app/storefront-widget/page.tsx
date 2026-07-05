"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function StorefrontWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [previewStatus, setPreviewStatus] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
    }
    document.title = "Embed Your Store | OHC";
  }, []);

  const embedCode = `<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=${tenant}&theme=${theme}" width="320" height="400" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { background: '#1D1D1F', color: '#ffffff', borderColor: '#333333' };
    }
    return { background: '#ffffff', color: '#111827', borderColor: '#e5e7eb' };
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-3">
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Embed Your Store 🌐</h1>
             <span className="bg-blue-100 text-blue-800 text-xs font-semibold px-2 py-1 rounded">New Growth Loop</span>
         </div>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Editor Sidebar */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', boxShadow: '0 4px 24px rgba(0,0,0,0.04)' }}>
                <h2 className="text-lg font-semibold font-outfit mb-4">Widget Settings</h2>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex bg-gray-100 p-1 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Store ID (Tenant)</label>
                    <input
                        type="text"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. my-store"
                    />
                    <p className="text-xs text-gray-500 mt-2">Used to link the widget to your store.</p>
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => setRemoveBranding(e.target.checked)}
                            className="w-4 h-4 text-[#0071E3] border-gray-300 rounded focus:ring-[#0066FF]"
                        />
                        <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge (Pro)</span>
                    </label>
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-[#0071E3] text-white font-medium min-h-[44px] min-w-[44px] hover:bg-blue-700 transition-colors shadow-sm"
                >
                    Get Widget
                </button>
            </div>

            <div className="p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', boxShadow: '0 4px 24px rgba(0,0,0,0.04)' }}>
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">📈</span> Why embed?
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Turn any blog post, partner website, or link-in-bio into a point of sale.
                    Stores using embedded widgets see up to a <strong>24% increase</strong> in organic sales.
                </p>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                <div className="relative z-10 w-[320px] h-[400px]" style={{ ...getThemeStyles(), borderRadius: '16px', boxShadow: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)' }}>
                    {/* Widget Content for Preview (matches the real iframe output loosely) */}
                    <div className="w-full h-48 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-t-[16px] relative flex items-center justify-center">
                        <span className="text-4xl">🛍️</span>
                        <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 text-white text-xs font-bold px-3 py-1 rounded-full">
                            Featured
                        </div>
                    </div>
                    <div className="p-5 flex flex-col h-[208px]">
                        <h4 className="font-bold text-lg font-outfit mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>Premium Collection</h4>
                        <p className="text-sm mb-4 line-clamp-2" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>Discover our exclusive, high-quality products curated just for you. Buy directly from this widget!</p>

                        <div className="flex items-center justify-between mb-4">
                            <span className="font-bold text-2xl font-outfit" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>$49.99</span>
                            <span className={`text-xs font-semibold px-2 py-1 rounded ${theme === 'dark' ? 'bg-green-900/30 text-green-400' : 'bg-green-50 text-green-600'}`}>In Stock</span>
                        </div>

                        <button
                            type="button"
                            onClick={() => {
                                setPreviewStatus('Preview product added to checkout.');
                                router.push('/checkout');
                            }}
                            className="w-full py-2.5 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-lg text-sm flex items-center justify-center gap-2 transition-colors"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
                            Buy Now
                        </button>
                        {previewStatus && <p className="mt-2 text-xs font-semibold text-[#0071E3]" role="status">{previewStatus}</p>}
                    </div>
                </div>
                {!removeBranding && (
                    <div className="mt-2 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                        <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>
                            ⚡ Powered by OHC
                        </a>
                    </div>
                )}
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Storefront</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, blog, or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-[#0071E3] transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium min-h-[44px] min-w-[44px] transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}
    </div>
  );
}
