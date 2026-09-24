"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ReferralWidgetBuilderPage() {
  const [tenant, setTenant] = useState('my-store');
  const [hasPro, setHasPro] = useState(false);
  const [amount, setAmount] = useState('10');
  const [type, setType] = useState('%');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [paywallStatus, setPaywallStatus] = useState('');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    try {
      const storedTenant = localStorage.getItem('business_display_name') || 'my-store';
      const storedPro = localStorage.getItem('has_pro') === 'true';
      setTenant(storedTenant);
      setHasPro(storedPro);
    } catch {
      // ignore
    }
  }, []);

  const handleCheckboxChange = (checked: boolean) => {
    if (checked && !hasPro) {
      setShowPaywall(true);
      return;
    }
    setRemoveBranding(checked);
  };

  const handleShareToUnlock = async () => {
    const message = `Check out my new Referral Program built with OmniSolo! 🚀 #OmniSolo #SmallBiz https://omnisolo.co/invite/${tenant}`;
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}`;
    try {
      window.open(shareUrl, '_blank');
    } catch {
      // ignore
    }

    setPaywallStatus('Verifying Share...');

    try {
      const response = await fetch('/api/v1/growth/trial-extension/claim', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      });

      if (response.ok) {
        setPaywallStatus('Unlocked!');
        setHasPro(true);
        localStorage.setItem('has_pro', 'true');
        setTimeout(() => {
          setShowPaywall(false);
          setRemoveBranding(true);
          setPaywallStatus('');
        }, 1500);
      } else {
        setPaywallStatus('Failed to claim trial extension.');
        setTimeout(() => setPaywallStatus(''), 3000);
      }
    } catch {
      setPaywallStatus('Error claiming trial extension.');
      setTimeout(() => setPaywallStatus(''), 3000);
    }
  };

  const embedCode = `<iframe src="${typeof window !== 'undefined' ? window.location.origin : 'https://cloud.omnisolo.co'}/api/v1/growth/customer-referral/embed?tenant=${tenant}&theme=${theme}&give=${type === '$' ? '$' : ''}${amount}${type === '%' ? '%' : ''}&get=${type === '$' ? '$' : ''}${amount}${type === '%' ? '%' : ''}&hide_branding=${removeBranding}" width="100%" height="200" style="border:none;border-radius:16px;overflow:hidden;" title="OmniSolo Referral Widget"></iframe>`;

  const handleCopyCode = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-50 font-inter text-gray-900 pb-20">
      <nav className="sticky top-0 z-40 bg-white/80 backdrop-blur-[30px] saturate-[210%] border-b border-gray-200 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link href="/dashboard" aria-label="Back to Dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
          </Link>
          <h1 className="text-xl font-bold font-outfit text-gray-900 flex items-center gap-2">
            Referral Widget Builder
            <span className="bg-indigo-100 text-indigo-700 text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wider">
              Viral Growth
            </span>
          </h1>
        </div>
      </nav>

      <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col md:flex-row gap-8">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-2xl">
            <h2 className="text-lg font-bold font-outfit mb-6">Widget Configuration</h2>

            <div className="mb-4">
              <label htmlFor="referral-discount-amount" className="block text-sm font-medium text-gray-700 mb-2">Discount Amount</label>
              <input
                id="referral-discount-amount"
                type="text"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
              />
            </div>

            <div className="mb-4">
              <label htmlFor="referral-discount-type" className="block text-sm font-medium text-gray-700 mb-2">Discount Type</label>
              <select
                id="referral-discount-type"
                value={type}
                onChange={(e) => setType(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
              >
                <option value="%">Percentage (%)</option>
                <option value="$">Fixed Amount ($)</option>
              </select>
            </div>

            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
              <div className="flex bg-gray-100 p-1 rounded-lg">
                <button
                  type="button"
                  onClick={() => setTheme('light')}
                  className={`flex-1 py-2 text-sm font-medium rounded-md transition-all ${theme === 'light' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500'}`}
                >
                  Light
                </button>
                <button
                  type="button"
                  onClick={() => setTheme('dark')}
                  className={`flex-1 py-2 text-sm font-medium rounded-md transition-all ${theme === 'dark' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500'}`}
                >
                  Dark
                </button>
              </div>
            </div>

            <div className="mb-6">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  id="remove-branding-checkbox"
                  aria-label='Remove "OmniSolo" Branding'
                  checked={removeBranding}
                  onChange={(e) => handleCheckboxChange(e.target.checked)}
                  className="w-4 h-4 text-[#0066FF] border-gray-300 rounded focus:ring-[#0066FF]"
                />
                <span className="text-sm font-medium text-gray-700">Remove "OmniSolo" Branding</span>
              </label>
            </div>

            <button
              onClick={() => setShowModal(true)}
              className="w-full py-3 bg-[#0066FF] text-white font-medium rounded-xl hover:bg-[#0052CC] transition-colors shadow-sm"
            >
              Get Embed Code
            </button>
          </div>
        </div>

        <div className="w-full md:w-2/3 flex flex-col items-center">
          <h2 className="text-xl font-semibold font-outfit self-start mb-4 text-[#1D1D1F]">Live Preview</h2>
          <div className="w-full p-8 flex flex-col items-center justify-center bg-gradient-to-br from-gray-50 to-gray-200 border border-white/40 rounded-2xl">
            <div className={`w-[360px] p-6 rounded-2xl shadow-xl transition-all ${theme === 'dark' ? 'bg-[#16161a] text-white' : 'bg-white text-gray-900'}`}>
              <div className="text-center">
                <div className="w-16 h-16 mx-auto bg-indigo-100 text-indigo-600 rounded-full flex items-center justify-center mb-4 text-2xl">
                  🎁
                </div>
                <h3 className="text-xl font-bold font-outfit mb-2">
                  Give {type === '$' ? '$' : ''}{amount}{type === '%' ? '%' : ''}, Get {type === '$' ? '$' : ''}{amount}{type === '%' ? '%' : ''}
                </h3>
                <p className={`text-sm mb-6 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                  Share your link with friends. They get {type === '$' ? '$' : ''}{amount}{type === '%' ? '%' : ''} off their first order, and you get {type === '$' ? '$' : ''}{amount}{type === '%' ? '%' : ''} off your next!
                </p>

                <div className="flex bg-gray-100 dark:bg-gray-800 rounded-lg p-1 mb-4">
                  <input
                    type="text"
                    readOnly
                    aria-label="Referral link preview"
                    value={`https://cloud.omnisolo.co/setup.html?ref=${tenant}&promo=ref123`}
                    className="flex-1 bg-transparent border-none text-xs text-gray-600 dark:text-gray-300 px-2 focus:outline-none"
                  />
                  <button className="bg-white dark:bg-gray-700 text-indigo-600 dark:text-indigo-400 text-xs font-bold py-2 px-4 rounded-md shadow-sm">
                    Copy Link
                  </button>
                </div>

                {!removeBranding && (
                  <div className="mt-4 pt-4 border-t border-gray-100 dark:border-gray-800 text-xs text-center">
                    <a
                      href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="font-bold text-gray-500 hover:underline"
                    >
                      ⚡ Powered by OmniSolo
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
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
          <div className="bg-white dark:bg-gray-900 rounded-2xl p-8 max-w-xl w-full shadow-2xl relative">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-6 right-6 text-gray-400 hover:text-gray-600"
              aria-label="Close embed modal"
            >
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
            <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900 dark:text-white">Embed Referral Widget</h2>
            <p className="text-gray-600 dark:text-gray-300 mb-6 text-sm">Copy and paste this HTML snippet into your website or post-checkout page.</p>
            <textarea
              readOnly
              aria-label="Embed Code"
              value={embedCode}
              className="w-full h-32 p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl font-mono text-xs resize-none"
            />
            <div className="mt-6 flex gap-3">
              <button
                onClick={handleCopyCode}
                className="flex-1 py-3 bg-[#0066FF] hover:bg-[#0052CC] text-white font-medium rounded-xl transition-colors"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
              <button
                onClick={() => setShowModal(false)}
                className="flex-1 py-3 bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-white font-medium rounded-xl hover:bg-gray-200"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4">
          <div className="bg-white dark:bg-gray-900 w-full max-w-md rounded-2xl p-8 shadow-2xl relative text-center">
            <button
              onClick={() => setShowPaywall(false)}
              className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 text-xl"
              aria-label="Close paywall"
            >
              &times;
            </button>
            <div className="w-16 h-16 bg-indigo-100 text-indigo-600 rounded-2xl flex items-center justify-center text-3xl mx-auto mb-4">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Upgrade to Pro</h2>
            <p className="text-gray-600 dark:text-gray-300 text-sm mb-6">
              Make the Referral Widget 100% yours. Upgrade to Pro to remove the "Powered by OmniSolo" watermark.
            </p>
            {paywallStatus ? (
              <p className="text-indigo-600 font-semibold mb-4 text-sm">{paywallStatus}</p>
            ) : (
              <button
                type="button"
                onClick={handleShareToUnlock}
                className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-xl transition-all shadow-md"
              >
                Share on X to Unlock 7 Days
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
