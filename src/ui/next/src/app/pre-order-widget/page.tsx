"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function PreOrderWidgetPage() {
  const [productName, setProductName] = useState('');
  const [offerText, setOfferText] = useState('');
  const [theme, setTheme] = useState('light');
  const [showEmbed, setShowEmbed] = useState(false);

  const router = useRouter();
  const [removeBranding, setRemoveBranding] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [isUnlocking, setIsUnlocking] = useState(false);
  const [tenant, setTenant] = useState('demo');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
      setTenant(localStorage.getItem('tenant_id') || 'demo');
    }
  }, []);

  const handleBrandingToggle = () => {
    if (removeBranding) {
      setRemoveBranding(false);
    } else {
      if (hasPro) {
        setRemoveBranding(true);
      } else {
        setShowSoftPaywall(true);
      }
    }
  };

  const handleShareToUnlock = () => {
    setIsUnlocking(true);
    setTimeout(() => {
      setIsUnlocking(false);
      setShowSoftPaywall(false);
      setHasPro(true);
      setRemoveBranding(true);
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
      }
    }, 1500);
  };


  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-8 font-sans">
      <div className="max-w-4xl mx-auto">
        <Link href="/dashboard" className="text-[#0071E3] hover:underline mb-8 inline-block">
          &larr; Back to Dashboard
        </Link>

        <h1 className="text-4xl font-bold mb-2 text-gray-900 dark:text-white">Pre-Order Waitlist Engine</h1>
        <p className="text-gray-600 dark:text-gray-400 mb-8">
          Configure your viral waitlist widget. Capture emails and allow customers to reserve their spot.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
          {/* Configuration Form */}
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Product Name
              </label>
              <input
                type="text"
                placeholder="e.g. The Vegan Chocolate Cake"
                className="w-full px-4 py-2 border rounded-xl dark:bg-black/20 dark:border-white/10 dark:text-white"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Special Offer (Optional)
              </label>
              <input
                type="text"
                placeholder="e.g. Get 10% off your pre-order!"
                className="w-full px-4 py-2 border rounded-xl dark:bg-black/20 dark:border-white/10 dark:text-white"
                value={offerText}
                onChange={(e) => setOfferText(e.target.value)}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Theme
              </label>
              <div className="flex space-x-4">
                <button
                  onClick={() => setTheme('light')}
                  className={`px-4 py-2 rounded-lg border ${theme === 'light' ? 'bg-blue-50 border-blue-200 text-blue-700' : 'bg-white dark:bg-black/20 dark:border-white/10'}`}
                >
                  Light
                </button>
                <button
                  onClick={() => setTheme('dark')}
                  className={`px-4 py-2 rounded-lg border ${theme === 'dark' ? 'bg-blue-900 border-blue-700 text-blue-100' : 'bg-white dark:bg-black/20 dark:border-white/10 text-black dark:text-white'}`}
                >
                  Dark
                </button>
              </div>
            </div>


            <div className="flex items-center space-x-3 mt-4 p-4 bg-gray-50 dark:bg-black/20 rounded-xl border border-gray-200 dark:border-gray-800">
              <input
                type="checkbox"
                id="removeBranding"
                checked={removeBranding}
                onChange={handleBrandingToggle}
                className="w-5 h-5 rounded border-gray-300 text-[#0071E3] focus:ring-[#0066FF]"
              />
              <div className="flex flex-col">
                 <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 dark:text-gray-300">
                   Remove "Powered by OHC" branding
                 </label>
                 {!hasPro && !removeBranding && (
                   <span className="text-xs text-amber-600 dark:text-amber-500 font-medium mt-1">Requires Pro or Share to Unlock ✨</span>
                 )}
              </div>
            </div>

            <button
              onClick={() => setShowEmbed(true)}
              className="w-full py-3 bg-[#0071E3] hover:bg-blue-700 text-white rounded-xl font-medium transition-colors"
            >
              Get Widget Embed Code
            </button>
          </div>

          {/* Live Preview */}
          <div className={`p-8 rounded-2xl border ${theme === 'light' ? 'bg-white border-gray-200 text-black' : 'bg-gray-900 border-gray-800 text-white'}`}>
            <h3 className="text-sm font-semibold text-gray-400 mb-6 uppercase tracking-wider">Live Preview</h3>

            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-blue-100 dark:bg-blue-900/50 rounded-full flex items-center justify-center mx-auto text-2xl mb-4">
                ✨
              </div>
              <h2 className="text-2xl font-bold">
                {productName || 'Your Product Name'}
              </h2>
              {offerText && (
                <p className={`inline-block px-3 py-1 rounded-full text-sm font-medium ${theme === 'light' ? 'bg-green-100 text-green-800' : 'bg-green-900/50 text-green-300'}`}>
                  {offerText}
                </p>
              )}
              <p className={theme === 'light' ? 'text-gray-600' : 'text-gray-400'}>
                Join the waitlist to get notified when we launch. Spots are limited!
              </p>

              <div className="mt-6 flex space-x-2">
                <input
                  type="email"
                  placeholder="Enter your email"
                  className={`flex-1 px-4 py-2 rounded-lg border ${theme === 'light' ? 'bg-white border-gray-300' : 'bg-gray-800 border-gray-700 text-white'}`}
                />
                <button className="px-6 py-2 bg-[#0071E3] text-white rounded-lg font-medium hover:bg-blue-700">
                  Join
                </button>
              </div>

              <p className={`text-xs mt-4 ${theme === 'light' ? 'text-gray-500' : 'text-gray-500'}`}>
                Join 1,204 others on the waitlist
              </p>

              {!removeBranding && (
                <div style={{ fontFamily: 'sans-serif', textAlign: 'center', fontSize: '12px', marginTop: '16px' }}>
                    <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>⚡ Powered by OHC</a>
                </div>
              )}

            </div>
          </div>
        </div>

        {/* Embed Modal */}
        {showEmbed && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
            <div className="bg-white dark:bg-gray-900 rounded-2xl p-8 max-w-lg w-full">
              <div className="flex justify-between items-center mb-6">
                <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Embed Your Waitlist</h2>
                <button onClick={() => setShowEmbed(false)} className="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 text-2xl">
                  &times;
                </button>
              </div>
              <p className="text-gray-600 dark:text-gray-400 mb-4">
                Copy and paste this code into your website's HTML where you want the waitlist to appear.
              </p>
              <div className="bg-gray-100 dark:bg-black/50 p-4 rounded-xl font-mono text-sm text-gray-800 dark:text-gray-200 overflow-x-auto mb-6">
                {`<div id="ohc-pre-order-widget" data-product="${productName}" data-offer="${offerText}" data-theme="${theme}" data-tenant="${tenant}"></div>`}
                <br/>
                {`<script src="https://assets.onehumancorp.com/widgets/pre-order.js" async></script>`}
                {!removeBranding && (
                  <>
                    <br/>
                    {`<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://app.onehumancorp.com/onboarding?ref=${tenant}" target="_blank" rel="noreferrer" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`}
                  </>
                )}
              </div>
              <button onClick={() => setShowEmbed(false)} className="w-full py-3 bg-[#0071E3] hover:bg-blue-700 text-white rounded-xl font-medium transition-colors">
                Done
              </button>
            </div>
          </div>
        )}

        {/* Soft Paywall Modal */}
        {showSoftPaywall && (
          <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
            <div className="bg-white dark:bg-gray-900 w-full max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 dark:border-indigo-900 text-center">
              <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 dark:bg-indigo-900/20 rounded-bl-full -z-10"></div>

              <div className="flex justify-end mb-2">
                <button
                  onClick={() => setShowSoftPaywall(false)}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors w-8 h-8 flex items-center justify-center"
                >
                  <span className="text-xl leading-none">&times;</span>
                </button>
              </div>

              <div className="w-16 h-16 bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center text-3xl shadow-inner text-indigo-600 dark:text-indigo-400 mx-auto mb-6">
                ✨
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-3">Upgrade to Pro</h2>
              <p className="text-gray-600 dark:text-gray-400 mb-6 text-sm leading-relaxed">
                Make the Pre-Order Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
              </p>

              <button
                onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
                className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700"
              >
                Upgrade to Pro
              </button>
              <p className="text-sm font-medium text-gray-500 mb-3">or</p>
              <button
                onClick={handleShareToUnlock}
                disabled={isUnlocking}
                className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-all flex items-center justify-center gap-2"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                {isUnlocking ? 'Verifying Share...' : 'Share on X to Unlock'}
              </button>
            </div>
          </div>
        )}

      </div>

    </div>
  );
}
