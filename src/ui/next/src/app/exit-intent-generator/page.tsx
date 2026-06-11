"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";
import Link from "next/link";

export default function ExitIntentGeneratorPage() {
  const [tenant, setTenant] = useState("unknown");
  const [hasPro, setHasPro] = useState(false);
  const [discount, setDiscount] = useState("10");
  const [headline, setHeadline] = useState("Wait! Don't leave yet.");
  const [subheading, setSubheading] = useState("Get 10% off your order if you complete checkout now.");
  const [buttonText, setButtonText] = useState("Claim 10% Off");
  const [theme, setTheme] = useState("light");
  const [removeBranding, setRemoveBranding] = useState(false);

  useEffect(() => {
    setTenant(localStorage.getItem('tenant_id') || "unknown");
    setHasPro(localStorage.getItem('tier') === 'Pro');
  }, []);

  const generateEmbedCode = () => {
    const baseUrl = typeof window !== 'undefined' ? (window.location.origin.includes('localhost') ? window.location.origin : 'https://ohc.app') : 'https://ohc.app';
    return `<!-- Exit Intent Widget -->
<div id="ohc-exit-intent" data-tenant="${tenant}" data-discount="${discount}" data-headline="${headline}" data-subheading="${subheading}" data-btn="${buttonText}" data-theme="${theme}" data-branding="${!removeBranding}"></div>
<script src="${baseUrl}/widgets/exit-intent.js" async></script>
${!removeBranding ? `<!-- ⚡ Powered by OHC -->\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="${baseUrl}/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>` : ''}`;
  };

  return (
    <AppShell title="Exit Intent Pop-up Generator">
      <main className="p-4 sm:p-8 max-w-7xl mx-auto flex flex-col lg:flex-row gap-8">
        <div className="flex-1 space-y-6">
          <div className="bg-white dark:bg-[#1D1D1F] border border-gray-200 dark:border-white/10 rounded-2xl p-6 shadow-sm">
            <h2 className="text-xl font-bold mb-4 font-outfit text-gray-900 dark:text-gray-100">Configure Exit Intent Pop-up</h2>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">Capture customers before they leave your storefront. Configure your offer below and embed the script on your site.</p>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Headline</label>
                <input
                  type="text"
                  value={headline}
                  onChange={(e) => setHeadline(e.target.value)}
                  className="w-full px-4 py-2 border rounded-xl dark:bg-black/50 dark:border-gray-700 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Subheading</label>
                <input
                  type="text"
                  value={subheading}
                  onChange={(e) => setSubheading(e.target.value)}
                  className="w-full px-4 py-2 border rounded-xl dark:bg-black/50 dark:border-gray-700 dark:text-white"
                />
              </div>

              <div className="flex gap-4">
                <div className="flex-1">
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Discount (%)</label>
                  <input
                    type="number"
                    value={discount}
                    onChange={(e) => setDiscount(e.target.value)}
                    className="w-full px-4 py-2 border rounded-xl dark:bg-black/50 dark:border-gray-700 dark:text-white"
                  />
                </div>
                <div className="flex-1">
                  <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Button Text</label>
                  <input
                    type="text"
                    value={buttonText}
                    onChange={(e) => setButtonText(e.target.value)}
                    className="w-full px-4 py-2 border rounded-xl dark:bg-black/50 dark:border-gray-700 dark:text-white"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-1">Theme</label>
                <select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value)}
                  className="w-full px-4 py-2 border rounded-xl dark:bg-black/50 dark:border-gray-700 dark:text-white"
                >
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
              </div>

              <div className="pt-4 border-t border-gray-100 dark:border-gray-800">
                <label className="flex items-center space-x-3 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={removeBranding}
                        onChange={(e) => {
                            if (!hasPro) {
                                alert('Upgrade to Pro to remove branding');
                                return;
                            }
                            setRemoveBranding(e.target.checked);
                        }}
                        className="w-5 h-5 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
                        disabled={!hasPro}
                    />
                    <span className="text-sm font-medium text-gray-700 dark:text-gray-300">Remove "Powered by OHC" Badge (Pro)</span>
                </label>
                {!hasPro && (
                  <p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
                    Make the widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
                  </p>
                )}
              </div>
            </div>
          </div>

          <div className="bg-gray-50 dark:bg-black/20 rounded-2xl p-6 border border-gray-200 dark:border-gray-800">
            <h3 className="text-sm font-bold text-gray-900 dark:text-white mb-2 font-outfit uppercase tracking-wider">Embed Code</h3>
            <pre className="text-xs font-mono bg-black text-green-400 p-4 rounded-xl overflow-x-auto whitespace-pre-wrap">
              {generateEmbedCode()}
            </pre>
            <button
                onClick={() => {
                  navigator.clipboard.writeText(generateEmbedCode());
                  alert('Copied to clipboard!');
                }}
                className="mt-4 w-full py-2 bg-indigo-600 text-white font-semibold rounded-xl hover:bg-indigo-700 transition-colors"
            >
                Copy Embed Code
            </button>
          </div>
        </div>

        <div className="flex-1">
          <div className="sticky top-8">
            <h2 className="text-sm font-bold mb-4 font-outfit uppercase tracking-wider text-gray-500 text-center">Live Preview</h2>
            <div className={`p-8 rounded-2xl shadow-2xl relative ${theme === 'dark' ? 'bg-[#1D1D1F] text-white border border-gray-800' : 'bg-white text-gray-900 border border-gray-200'}`}>
                <div className="text-center">
                    <h3 className="text-3xl font-bold font-outfit mb-3">{headline}</h3>
                    <p className={`mb-8 ${theme === 'dark' ? 'text-gray-300' : 'text-gray-600'}`}>{subheading}</p>
                    <button className="px-8 py-3 bg-indigo-600 text-white rounded-xl font-bold text-lg w-full hover:bg-indigo-700 transition-colors shadow-lg shadow-indigo-600/30">
                        {buttonText}
                    </button>
                    {!removeBranding && (
                        <div className="mt-6 pt-4 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                            <span className="text-xs font-semibold tracking-wide" style={{ color: '#6b7280' }}>⚡ Powered by OHC</span>
                        </div>
                    )}
                </div>
            </div>
          </div>
        </div>
      </main>
    </AppShell>
  );
}
