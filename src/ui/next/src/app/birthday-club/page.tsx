"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function BirthdayClubBuilder() {
  const router = useRouter();

  const [tenant, setTenant] = useState('my-store');
  const [discountAmount, setDiscountAmount] = useState('15');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

  useEffect(() => {
    const tid = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store') : 'my-store';
    setTenant(tid);
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true' || localStorage.getItem('plan') === 'pro' || localStorage.getItem('plan') === 'business');
    }
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/birthday-club/embed?tenant=${tenant}&discount=${encodeURIComponent(discountAmount)}&hideBranding=${removeBranding}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `
<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=birthday_club" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleGenerate = () => {
    setShowModal(true);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-4 md:p-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <button onClick={() => router.back()} className="mb-6 flex items-center text-sm font-semibold text-gray-500 hover:text-gray-900 dark:hover:text-white transition-colors">
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Dashboard
        </button>

        <div className="flex items-center gap-3 mb-8">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-pink-400 to-pink-600 flex items-center justify-center text-white text-2xl shadow-lg">
            🎂
          </div>
          <div>
            <h1 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Birthday Club Builder</h1>
            <p className="text-gray-500 dark:text-gray-400 mt-1 font-medium">Build an automated birthday list and capture emails with a special gift.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Controls */}
          <div className="space-y-6">
            <div className="glassmorphism p-6 rounded-[24px] border border-white/40 dark:border-white/10 shadow-xl bg-white/40 backdrop-blur-[30px] saturate-[210%]">
              <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-6">Program Rules</h2>

              <div className="space-y-5">
                <div>
                  <label htmlFor="discount-amount" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">Birthday Gift (% Discount)</label>
                  <div className="relative">
                    <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">%</span>
                    <input
                      id="discount-amount"
                      type="number"
                      value={discountAmount}
                      onChange={(e) => setDiscountAmount(e.target.value)}
                      className="w-full pl-8 pr-4 py-3 bg-white dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-xl focus:ring-2 focus:ring-pink-500 focus:border-pink-500 outline-none transition-all dark:text-white"
                    />
                  </div>
                </div>

                <div className="pt-2 border-t border-gray-100 dark:border-gray-800">
                  <label className="flex items-center gap-3 cursor-pointer group">
                    <div className="relative">
                      <input
                        id="branding-toggle"
                        type="checkbox"
                        className="sr-only"
                        checked={removeBranding}
                        onChange={(e) => {
                          if (!hasPro && e.target.checked) {
                            setShowSoftPaywall(true);
                          } else {
                            setRemoveBranding(e.target.checked);
                          }
                        }}
                      />
                      <div className={`block w-12 h-6 rounded-full transition-colors ${removeBranding ? 'bg-pink-500' : 'bg-gray-300 dark:bg-gray-700'}`}></div>
                      <div className={`absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${removeBranding ? 'translate-x-6' : 'translate-x-0'}`}></div>
                    </div>
                    <div className="flex flex-col">
                      <span className="text-sm font-semibold text-gray-700 dark:text-gray-300">Remove "Powered by OHC" Badge (Pro)</span>
                      <span className="text-xs text-gray-500">Make the widget 100% white-labeled</span>
                    </div>
                  </label>
                </div>

                <div className="pt-4">
                  <button
                    id="generate-button"
                    onClick={handleGenerate}
                    className="w-full py-4 bg-pink-600 hover:bg-pink-700 text-white font-bold rounded-xl shadow-lg shadow-pink-500/30 transition-all active:scale-[0.98] flex items-center justify-center gap-2"
                  >
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    Generate Widget Embed
                  </button>
                </div>
                {!removeBranding && (
                    <div className="mt-6 pt-4 border-t border-gray-100 dark:border-gray-700 text-center">
                        <span className="text-xs font-semibold tracking-wide" style={{ color: '#6b7280' }}>⚡ Powered by OHC</span>
                    </div>
                )}
              </div>
            </div>
          </div>

          {/* Preview */}
          <div className="space-y-6">
            <div className="glassmorphism p-6 md:p-8 rounded-[24px] border border-white/40 dark:border-white/10 shadow-xl bg-white/40 backdrop-blur-[30px] saturate-[210%] relative overflow-hidden group">
              <div className="absolute top-4 right-4 px-3 py-1 bg-pink-100 dark:bg-pink-900/40 text-pink-700 dark:text-pink-300 text-xs font-bold rounded-full tracking-wide">PREVIEW</div>

              <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-6">Birthday Club</h3>

              <div className="bg-white dark:bg-gray-800 rounded-2xl p-6 shadow-sm border border-gray-100 dark:border-gray-700 text-center relative w-full h-[400px]">
                <iframe src={`/api/v1/growth/birthday-club/embed?tenant=${tenant}&discount=${encodeURIComponent(discountAmount)}&hideBranding=${removeBranding}`} width="100%" height="100%" frameBorder="0" scrolling="no" style={{border: 'none', overflow: 'hidden', borderRadius: '16px'}}></iframe>
              </div>
            </div>
          </div>
        </div>

      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div id="paywall-modal" className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%] animate-in fade-in">
          <div className="bg-white dark:bg-[#1D1D1F] rounded-3xl p-6 md:p-8 max-w-md w-full shadow-2xl border border-white/20 relative animate-in zoom-in-95">
            <button
              id="close-paywall"
              onClick={() => setShowSoftPaywall(false)}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-500 transition-colors"
            >
              ✕
            </button>
            <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center text-white text-3xl mb-6 shadow-lg">
              ✨
            </div>
            <h3 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Pro Feature</h3>
            <p className="text-gray-600 dark:text-gray-400 mb-6">
              Make the Birthday Club 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock full white-label customization.
            </p>
            <div className="space-y-3">
              <button
                onClick={() => router.push('/pricing')}
                className="w-full py-3.5 bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-bold rounded-xl shadow-lg hover:scale-[1.02] transition-all"
              >
                Upgrade to Pro
              </button>
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="w-full py-3.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 font-semibold rounded-xl hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
              >
                Keep Branding
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Code Generation Modal */}
      {showModal && (
        <div id="embed-modal" className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%] animate-in fade-in">
          <div className="bg-white dark:bg-[#1D1D1F] rounded-3xl p-6 md:p-8 max-w-2xl w-full shadow-2xl border border-white/20 relative animate-in zoom-in-95">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-500 transition-colors"
            >
              ✕
            </button>

            <h3 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Embed Widget</h3>
            <p className="text-gray-600 dark:text-gray-400 mb-6">Paste this code into your website's HTML to embed the birthday club widget.</p>

            <div className="relative">
              <pre className="bg-gray-50 dark:bg-[#000000] p-4 rounded-xl text-sm text-gray-800 dark:text-gray-300 font-mono overflow-x-auto border border-gray-200 dark:border-gray-800 whitespace-pre-wrap break-all">
                {embedCode}
              </pre>
              <button
                onClick={handleCopy}
                className="absolute top-4 right-4 px-4 py-2 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 border border-gray-200 dark:border-gray-700 rounded-lg text-sm font-semibold shadow-sm transition-colors text-gray-900 dark:text-white flex items-center gap-2"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
            </div>

            <div className="mt-6 flex items-start gap-3 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-xl text-blue-700 dark:text-blue-400">
              <span className="text-xl">ℹ️</span>
              <p className="text-sm">The <strong>Powered by OHC</strong> badge helps us grow the community. If a new business owner signs up through your widget, you earn $50 in platform credits!</p>
            </div>
          </div>
        </div>
      )}
      <PoweredByOHC tenantId={tenant} />
    </div>
  );
}
