"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from "../components/AppShell";

export default function LeadMagnetGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [title, setTitle] = useState('Unlock the Ultimate Business Checklist');
  const [description, setDescription] = useState('Enter your email below to get instant access to our top 10 secrets for scaling your business.');
  const [buttonText, setButtonText] = useState('Download Now');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/lead-magnet/embed?tenant=${tenant}&theme=${theme}&title=${encodeURIComponent(title)}&description=${encodeURIComponent(description)}&buttonText=${encodeURIComponent(buttonText)}&hideBranding=${removeBranding}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="350" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleBrandingToggle = () => {
    if (!removeBranding && !hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    setRemoveBranding(!removeBranding);
  };

  const handleUpgrade = () => {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setRemoveBranding(true);
    setShowSoftPaywall(false);
  };

  return (
    <AppShell
      title="Lead Magnet Generator"
      subtitle="Capture emails and grow your audience."
    >
      <div className="max-w-6xl mx-auto w-full grid grid-cols-1 lg:grid-cols-2 gap-8 font-inter">
        <div className="space-y-6">
          <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-200">
            <h2 className="font-semibold text-gray-900 mb-4">Configure Widget</h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Headline</label>
                <input
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 animate-all duration-200"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 animate-all duration-200"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Button Text</label>
                <input
                  type="text"
                  value={buttonText}
                  onChange={(e) => setButtonText(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 animate-all duration-200"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="theme"
                      checked={theme === 'light'}
                      onChange={() => setTheme('light')}
                      className="text-indigo-600 focus:ring-indigo-500"
                    />
                    <span className="text-sm">Light</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="theme"
                      checked={theme === 'dark'}
                      onChange={() => setTheme('dark')}
                      className="text-indigo-600 focus:ring-indigo-500"
                    />
                    <span className="text-sm">Dark</span>
                  </label>
                </div>
              </div>

              <div className="pt-4 border-t">
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={handleBrandingToggle}
                    className="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500"
                  />
                  <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Branding</span>
                  {!hasPro && (
                    <span className="text-[10px] uppercase font-bold tracking-wider bg-yellow-100 text-yellow-800 px-2 py-0.5 rounded">Pro</span>
                  )}
                </label>
              </div>
            </div>
          </div>

          <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-200">
            <h2 className="font-semibold text-gray-900 mb-4">Embed Code</h2>
            <div className="relative">
              <pre className="bg-gray-50 p-4 rounded-lg text-sm text-gray-600 overflow-x-auto whitespace-pre-wrap border border-gray-200">
                {embedCode}
              </pre>
              <button
                onClick={handleCopy}
                className="absolute top-2 right-2 bg-white px-3 py-1.5 rounded-md shadow-sm border text-sm font-medium hover:bg-gray-50 text-gray-700 cursor-pointer active:scale-[0.98] transition-all"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
            </div>
            <p className="text-xs text-gray-500 mt-3">
              Paste this HTML directly into your website (WordPress, Shopify, Wix, Squarespace, etc) to capture leads.
            </p>
          </div>
        </div>

        <div>
          <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 sticky top-24">
            <h2 className="font-semibold text-gray-900 mb-4">Live Preview</h2>
            <div className="p-4 bg-gray-100 dark:bg-gray-800 rounded-xl flex justify-center border border-gray-200 dark:border-gray-700">
              <div
                className={`w-full max-w-sm rounded-2xl overflow-hidden shadow-lg border p-6 text-center ${theme === 'dark' ? 'bg-gray-900 text-white border-gray-800' : 'bg-white text-gray-900 border-gray-200'}`}
              >
                <div className="mb-4">
                  <span className={`inline-block p-3 rounded-full ${theme === 'dark' ? 'bg-indigo-900 text-indigo-300' : 'bg-indigo-50 text-indigo-600'}`}>
                    <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                  </span>
                </div>
                <h3 className="text-xl font-bold font-outfit mb-2">{title}</h3>
                <p className={`text-sm mb-6 ${theme === 'dark' ? 'text-gray-300' : 'text-gray-600'}`}>{description}</p>
                <div className="space-y-3 flex flex-col items-center">
                  <input type="email" placeholder="Enter your email address" className={`w-full px-4 py-3 rounded-xl border text-sm ${theme === 'dark' ? 'bg-gray-800 border-gray-700 text-white placeholder-gray-400' : 'bg-white border-gray-200 text-gray-900 placeholder-gray-400'} focus:outline-none focus:ring-2 focus:ring-indigo-500`} readOnly />
                  <button className="min-h-[40px] px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-full shadow-md transition-all text-sm active:scale-[0.98] cursor-pointer inline-flex justify-center items-center">
                    {buttonText}
                  </button>
                </div>
                {!removeBranding && (
                  <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-800">
                    <a href={`https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} className={`text-xs font-semibold no-underline ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                      ⚡ Powered by OHC
                    </a>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-gray-900/40 backdrop-blur-[30px] saturate-[210%] z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-2xl shadow-xl max-w-md w-full p-6 text-center border border-gray-100">
            <div className="w-16 h-16 bg-indigo-50 rounded-full flex items-center justify-center mx-auto mb-4">
              <span className="text-2xl">✨</span>
            </div>
            <h3 className="text-xl font-bold text-gray-900 font-outfit mb-2">Upgrade to OHC Pro</h3>
            <p className="text-gray-600 text-sm mb-6">
              Remove the "Powered by OHC" branding and get access to premium templates, advanced analytics, and custom domains.
            </p>
            <div className="space-y-3">
              <button
                onClick={handleUpgrade}
                className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-xl transition-colors cursor-pointer"
              >
                Upgrade Now ($19/mo)
              </button>
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="w-full py-3 bg-gray-50 hover:bg-gray-100 text-gray-700 font-semibold rounded-xl transition-colors cursor-pointer"
              >
                Keep Branding
              </button>
            </div>
          </div>
        </div>
      )}
    </AppShell>
  );
}
