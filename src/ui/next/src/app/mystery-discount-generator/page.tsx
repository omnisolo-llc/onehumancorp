"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function MysteryDiscountGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [hasPro, setHasPro] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

  // Widget settings
  const [widgetTitle, setWidgetTitle] = useState('Mystery Discount Box');
  const [description, setDescription] = useState('Unlock a surprise discount up to 50% off! Enter your email to reveal.');
  const [discountCodes, setDiscountCodes] = useState('MYSTERY10,MYSTERY20,MYSTERY50');
  const [theme, setTheme] = useState('light');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    if (typeof document !== 'undefined') {
      document.title = "Mystery Discount Generator | OHC";
    }
  }, []);

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
      return;
    }
    setRemoveBranding(e.target.checked);
  };

  const embedUrl = `https://ohc.app/api/v1/growth/mystery-discount/embed?tenant=${tenant}&title=${encodeURIComponent(widgetTitle)}&desc=${encodeURIComponent(description)}&codes=${encodeURIComponent(discountCodes)}&theme=${theme}&branding=${!removeBranding}`;
  const absoluteEmbedUrl = `/api/v1/growth/mystery-discount/embed?tenant=${tenant}&title=${encodeURIComponent(widgetTitle)}&desc=${encodeURIComponent(description)}&codes=${encodeURIComponent(discountCodes)}&theme=${theme}&branding=${!removeBranding}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="380" style="border:none; border-radius:16px; overflow:hidden;" title="Mystery Discount Box"></iframe>`;

  if (!isClient) return <div className="min-h-screen bg-gray-50" />;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 sticky top-0 z-50">
        <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Viral Mystery Discount Generator 🎁</h1>
        <div className="flex items-center gap-3">
          <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
            Back to Dashboard
          </button>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col lg:flex-row gap-8">
        {/* Editor Settings */}
        <section className="w-full lg:w-1/2 flex flex-col gap-6">
          <div className="bg-gradient-to-r from-purple-50 to-pink-50 border border-purple-100 rounded-2xl p-6 shadow-sm">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Engage & Capture Leads</h2>
            <p className="text-gray-600 text-sm">
              Embed a Mystery Box on your site. Customers enter their email to reveal a surprise discount code. Built-in viral loop encourages them to share your store!
            </p>
          </div>

          <div className="p-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-2xl">
            <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Widget Configuration</h2>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="widget-title" className="block text-sm font-medium text-gray-700 mb-1">Widget Title</label>
                <input
                  id="widget-title"
                  type="text"
                  value={widgetTitle}
                  onChange={(e) => setWidgetTitle(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>

              <div>
                <label htmlFor="widget-desc" className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  id="widget-desc"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                  rows={2}
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Discount Codes (Comma Separated)</label>
                <p className="text-xs text-gray-500 mb-2">The widget will randomly pick one of these to reveal.</p>
                <input
                  type="text"
                  value={discountCodes}
                  onChange={(e) => setDiscountCodes(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 font-mono text-sm"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input type="radio" name="theme" value="light" checked={theme === 'light'} onChange={() => setTheme('light')} className="text-purple-600 focus:ring-purple-500" />
                    <span className="text-sm text-gray-700">Light</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input type="radio" name="theme" value="dark" checked={theme === 'dark'} onChange={() => setTheme('dark')} className="text-purple-600 focus:ring-purple-500" />
                    <span className="text-sm text-gray-700">Dark</span>
                  </label>
                </div>
              </div>

              <div className="pt-4 border-t border-gray-100">
                <label className="flex items-start gap-3 cursor-pointer group">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={handleBrandingToggle}
                    className="mt-1 w-4 h-4 text-purple-600 rounded focus:ring-purple-500 border-gray-300"
                  />
                  <div>
                    <span className="text-sm font-medium text-gray-900">Remove "Powered by OHC" branding</span>
                    <p className="text-xs text-gray-500 mt-1">Make the widget 100% white-labeled. Requires Pro plan.</p>
                  </div>
                </label>
              </div>
            </div>
          </div>
        </section>

        {/* Live Preview & Embed */}
        <section className="w-full lg:w-1/2 flex flex-col gap-6">
          <div className="p-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-2xl flex flex-col gap-4">
            <h2 className="text-xl font-semibold font-outfit text-gray-900">Live Preview</h2>
            <div className={`w-full rounded-2xl overflow-hidden border ${theme === 'dark' ? 'border-gray-800 bg-gray-900' : 'border-gray-200 bg-gray-50'} flex items-center justify-center`} style={{ height: '380px' }}>
              <iframe
                src={absoluteEmbedUrl}
                width="100%"
                height="100%"
                className="border-none"
                title="Preview"
              />
            </div>
          </div>

          <div className="p-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm rounded-2xl flex flex-col gap-4">
            <h2 className="text-xl font-semibold font-outfit text-gray-900">Embed Code</h2>
            <p className="text-sm text-gray-600">Copy this code and paste it into your website's HTML where you want the Mystery Box to appear.</p>
            <div className="relative">
              <pre className="bg-gray-900 text-gray-100 p-4 rounded-xl text-sm font-mono overflow-x-auto">
                {embedCode}
              </pre>
            </div>
            <button
              onClick={() => {
                navigator.clipboard.writeText(embedCode);
                setCopied(true);
                setTimeout(() => setCopied(false), 2000);
              }}
              className={`w-full py-3 rounded-xl font-bold transition-all text-sm flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700' : 'bg-purple-600 text-white hover:bg-purple-700 shadow-md shadow-purple-200'}`}
            >
              {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
            </button>
          </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-purple-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-purple-50 rounded-bl-full -z-10"></div>
            <div className="flex justify-end mb-2">
              <button
                aria-label="Close paywall"
                onClick={() => setShowPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>
            <div className="w-16 h-16 bg-purple-50 rounded-2xl flex items-center justify-center mx-auto mb-6 border border-purple-100">
              <span className="text-3xl text-purple-600">🚀</span>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-8 text-sm leading-relaxed">
              Make the Mystery Box 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>
            <button
              onClick={() => { setShowPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-3 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-purple-600 hover:bg-purple-700 flex justify-center items-center gap-2"
            >
              Upgrade to Pro
            </button>
            <button
              onClick={() => setShowPaywall(false)}
              className="w-full py-3 rounded-xl font-bold transition-all text-gray-500 hover:text-gray-800 hover:bg-gray-50 text-sm"
            >
              Cancel
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
