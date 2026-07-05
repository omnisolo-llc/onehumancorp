"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function CustomerReferralProgramPage() {
  const router = useRouter();
  const [giveAmount, setGiveAmount] = useState('10');
  const [getAmount, setGetAmount] = useState('10');
  const [tenant, setTenant] = useState('my-store');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

  useEffect(() => {
    const tid = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store') : 'my-store';
    setTenant(tid);
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/customer-referral/embed?tenant=${tenant}&give=${encodeURIComponent(giveAmount)}&get=${encodeURIComponent(getAmount)}&hideBranding=${removeBranding}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="250" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `
<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

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
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-emerald-400 to-emerald-600 flex items-center justify-center text-white text-2xl shadow-lg">
            💸
          </div>
          <div>
            <h1 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Customer Referral Program</h1>
            <p className="text-gray-500 dark:text-gray-400 mt-1 font-medium">Turn your customers into advocates with a 'Give X, Get Y' loop.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Controls */}
          <div className="space-y-6">
            <div className="glassmorphism p-6 rounded-[24px] border border-white/40 dark:border-white/10 shadow-xl bg-white/40 backdrop-blur-[30px] saturate-[210%]">
              <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-6">Program Rules</h2>

              <div className="space-y-5">
                <div>
                  <label htmlFor="give-amount" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">They Give ($ Discount)</label>
                  <div className="relative">
                    <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">$</span>
                    <input
                      id="give-amount"
                      type="number"
                      value={giveAmount}
                      onChange={(e) => setGiveAmount(e.target.value)}
                      className="w-full pl-8 pr-4 py-3 bg-white dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-xl focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition-all dark:text-white"
                    />
                  </div>
                </div>

                <div>
                  <label htmlFor="get-amount" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">They Get ($ Reward)</label>
                  <div className="relative">
                    <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">$</span>
                    <input
                      id="get-amount"
                      type="number"
                      value={getAmount}
                      onChange={(e) => setGetAmount(e.target.value)}
                      className="w-full pl-8 pr-4 py-3 bg-white dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-xl focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 outline-none transition-all dark:text-white"
                    />
                  </div>
                </div>


                <div className="pt-2 border-t border-gray-100 dark:border-gray-800">
                  <label className="flex items-center gap-3 cursor-pointer group">
                    <div className="relative">
                      <input
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
                      <div className={`block w-12 h-6 rounded-full transition-colors ${removeBranding ? 'bg-emerald-500' : 'bg-gray-300 dark:bg-gray-700'}`}></div>
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
                    onClick={handleGenerate}
                    className="w-full py-4 bg-emerald-600 hover:bg-emerald-700 text-white font-bold rounded-xl shadow-lg shadow-emerald-500/30 transition-all active:scale-[0.98] flex items-center justify-center gap-2"
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
              <div className="absolute top-4 right-4 px-3 py-1 bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300 text-xs font-bold rounded-full tracking-wide">PREVIEW</div>

              <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-6">Give ${giveAmount}, Get ${getAmount}</h3>

              <div className="bg-white dark:bg-gray-800 rounded-2xl p-6 shadow-sm border border-gray-100 dark:border-gray-700 text-center relative">
                <div className="w-16 h-16 mx-auto bg-emerald-100 dark:bg-emerald-900/30 rounded-full flex items-center justify-center text-3xl mb-4">
                  🎁
                </div>
                <h4 className="text-xl font-bold text-gray-900 dark:text-white mb-2">Give ${giveAmount}, Get ${getAmount}</h4>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">Give your friends ${giveAmount} off their first order, and get ${getAmount} when they purchase.</p>

                <div className="flex items-center gap-2 bg-gray-50 dark:bg-gray-900 p-2 rounded-lg border border-gray-200 dark:border-gray-700 mb-4">
                  <span className="text-sm font-mono text-gray-500 dark:text-gray-400 flex-1 truncate select-all px-2">
                    https://ohc.app/ref/{tenant.slice(0,6)}
                  </span>
                  <button className="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white text-sm font-semibold rounded-md transition-colors">
                    Copy
                  </button>
                </div>

                <div className="flex items-center justify-center gap-3 mb-6">
                  <button className="w-10 h-10 rounded-full bg-[#25D366] text-white flex items-center justify-center hover:opacity-90 transition-opacity">
                     <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413Z"/></svg>
                  </button>
                  <button className="w-10 h-10 rounded-full bg-black text-white flex items-center justify-center hover:opacity-90 transition-opacity">
                     𝕏
                  </button>
                  <button className="w-10 h-10 rounded-full bg-[#0066FF] text-white flex items-center justify-center hover:opacity-90 transition-opacity">
                     <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%] animate-in fade-in">
          <div className="bg-white dark:bg-[#1D1D1F] rounded-3xl p-6 md:p-8 max-w-md w-full shadow-2xl border border-white/20 relative animate-in zoom-in-95">
            <button
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
              Make the Customer Referral Program 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock full white-label customization.
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
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%] animate-in fade-in">
          <div className="bg-white dark:bg-[#1D1D1F] rounded-3xl p-6 md:p-8 max-w-2xl w-full shadow-2xl border border-white/20 relative animate-in zoom-in-95">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-500 transition-colors"
            >
              ✕
            </button>

            <h3 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Embed Widget</h3>
            <p className="text-gray-600 dark:text-gray-400 mb-6">Paste this code into your website's HTML to embed the referral widget.</p>

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
