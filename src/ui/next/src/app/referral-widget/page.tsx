"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ReferralWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [theme, setTheme] = useState('light');
  const [discountAmount, setDiscountAmount] = useState('10');
  const [discountType, setDiscountType] = useState('%');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [embedCode, setEmbedCode] = useState('');
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
        const storedTenant = localStorage.getItem('tenant') || 'my-store';
        setTenant(storedTenant);
        setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  useEffect(() => {
    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://app.onehumancorp.com';
    const iframeCode = `<iframe src="${origin}/embed/referral?tenant=${tenant}&theme=${theme}&discount=${discountAmount}${discountType === '%' ? 'pct' : 'flat'}&hideBranding=${removeBranding}" width="100%" height="200" style="border:none;border-radius:16px;overflow:hidden;" title="OHC Referral Widget"></iframe>`;
    setEmbedCode(iframeCode);
  }, [tenant, theme, discountAmount, discountType, removeBranding]);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleToggleBranding = (checked: boolean) => {
    if (checked && !hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    setRemoveBranding(checked);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { backgroundColor: '#1f2937', color: '#f9fafb' };
    }
    return { backgroundColor: '#ffffff', color: '#111827' };
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] text-gray-900 font-inter p-6 md:p-12">
      <header className="mb-10 text-center md:text-left">
        <h1 className="text-4xl font-bold font-outfit mb-3 tracking-tight">Referral Widget Builder</h1>
        <p className="text-lg text-gray-600 max-w-2xl">
          Turn your best customers into your marketing team. Embed this one-tap referral widget anywhere to acquire new customers.
        </p>
      </header>

      <main className="flex flex-col md:flex-row gap-8 max-w-7xl mx-auto">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-8 rounded-[24px] glassmorphism border border-white/40 shadow-sm relative z-10">
                <h2 className="text-xl font-bold font-outfit mb-6 flex items-center gap-2">
                   <svg className="w-5 h-5 text-indigo-500" fill="currentColor" viewBox="0 0 20 20"><path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z"></path></svg>
                   Configuration
                </h2>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Offer</label>
                    <div className="flex gap-2">
                        <input
                            type="number"
                            className="w-24 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            value={discountAmount}
                            onChange={(e) => setDiscountAmount(e.target.value)}
                        />
                        <select
                            className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            value={discountType}
                            onChange={(e) => setDiscountType(e.target.value)}
                        >
                            <option value="%">% Off</option>
                            <option value="$">$ Off</option>
                        </select>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
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
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => handleToggleBranding(e.target.checked)}
                            className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                        />
                        <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge (Pro)</span>
                    </label>
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-indigo-600 text-white font-medium rounded-xl hover:bg-indigo-700 transition-colors shadow-sm"
                >
                    Get Widget Code
                </button>
            </div>

            <div className="p-6 rounded-[20px] glassmorphism border border-white/40">
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">🚀</span> Viral Growth
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Referral loops lower your Customer Acquisition Cost. Add this widget to your post-checkout page to turn successful orders into new leads instantly.
                </p>
            </div>
        </div>

        <div className="w-full md:w-2/3">
            <div className="p-8 rounded-[24px] h-full flex flex-col items-center justify-center relative overflow-hidden bg-gradient-to-br from-indigo-50 to-purple-50 border border-white/50 shadow-inner">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                <div className="relative w-full max-w-md rounded-2xl shadow-xl overflow-hidden border border-gray-100" style={getThemeStyles()}>
                    <div className="p-6 text-center">
                        <div className="w-16 h-16 mx-auto bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center mb-4 text-2xl">
                            🎁
                        </div>
                        <h3 className="text-xl font-bold font-outfit mb-2" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                            Give {discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''}, Get {discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''}
                        </h3>
                        <p className="text-sm mb-6" style={{ color: theme === 'dark' ? '#d1d5db' : '#6b7280' }}>
                            Share your link with friends. They get {discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''} off their first order, and you get {discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''} off your next!
                        </p>

                        <div className="flex bg-gray-100 rounded-lg p-1 mb-4">
                            <input
                                type="text"
                                readOnly
                                value={`https://app.onehumancorp.com/onboarding?ref=${tenant}&promo=ref123`}
                                className="flex-1 bg-transparent border-none text-xs text-gray-600 px-2 focus:outline-none"
                            />
                            <button className="bg-white text-indigo-600 text-xs font-bold py-2 px-4 rounded-md shadow-sm hover:shadow transition-all">
                                Copy Link
                            </button>
                        </div>

                        {!removeBranding && (
                            <div className={`mt-4 pt-4 border-t text-xs ${theme === 'dark' ? 'border-gray-700 text-gray-400' : 'border-gray-100 text-gray-500'}`}>
                                <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="font-bold hover:underline" style={{ color: '#6b7280' }}>
                                    ⚡ Powered by OHC
                                </a>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
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

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Referral Widget</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your post-checkout page.</p>

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

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-indigo-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-indigo-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the Referral Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700"
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
        }
      `}} />
    </div>
  );
}
