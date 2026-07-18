"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';
import Link from 'next/link';

export default function GroupBuyWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [productName, setProductName] = useState('Premium Coffee Subscription');
  const [originalPrice, setOriginalPrice] = useState('24.99');
  const [groupPrice, setGroupPrice] = useState('15.00');
  const [requiredBuyers, setRequiredBuyers] = useState('5');
  const [timeLimit, setTimeLimit] = useState('24'); // Hours
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [hideBranding, setHideBranding] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    document.title = "Group Buy Widget Builder | OHC";
  }, []);

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setHideBranding(e.target.checked);
    }
  };

  const embedUrl = `/api/v1/growth/group-buy-widget/embed?tenant=${encodeURIComponent(tenant)}&productName=${encodeURIComponent(productName)}&originalPrice=${encodeURIComponent(originalPrice)}&groupPrice=${encodeURIComponent(groupPrice)}&requiredBuyers=${encodeURIComponent(requiredBuyers)}&timeLimit=${encodeURIComponent(timeLimit)}&theme=${theme}&branding=${!hideBranding}`;
  const absoluteEmbedUrl = `https://ohc.app/api/v1/growth/group-buy-widget/embed?tenant=${encodeURIComponent(tenant)}&productName=${encodeURIComponent(productName)}&originalPrice=${encodeURIComponent(originalPrice)}&groupPrice=${encodeURIComponent(groupPrice)}&requiredBuyers=${encodeURIComponent(requiredBuyers)}&timeLimit=${encodeURIComponent(timeLimit)}&theme=${theme}&branding=${!hideBranding}`;

  const embedCode = `<iframe src="${absoluteEmbedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7] dark:bg-[#1D1D1F]">
      <div className="flex-1 p-4 md:p-8 max-w-6xl mx-auto w-full">
        <button onClick={() => router.back()} className="mb-6 flex items-center text-sm font-semibold text-gray-500 hover:text-gray-900 dark:hover:text-white transition-colors">
          <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Dashboard
        </button>

        <div className="flex items-center gap-3 mb-8">
          <div className="w-12 h-12 rounded-2xl bg-gradient-to-br from-pink-400 to-rose-600 flex items-center justify-center text-white text-2xl shadow-lg">
            🤝
          </div>
          <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">Group Buy Widget Builder</h1>
            <p className="text-gray-600 dark:text-gray-400 mt-1">Unlock viral sales by letting customers group buy to unlock discounts.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Controls */}
          <div className="space-y-6 bg-white dark:bg-gray-800 p-6 rounded-[24px] shadow-sm border border-gray-100 dark:border-gray-700">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Product Name</label>
              <input
                type="text"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-500 focus:border-transparent outline-none transition-all"
                placeholder="e.g. Premium Coffee Subscription"
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Original Price ($)</label>
                <input
                  type="number"
                  step="0.01"
                  value={originalPrice}
                  onChange={(e) => setOriginalPrice(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-500 focus:border-transparent outline-none transition-all"
                  placeholder="e.g. 24.99"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Group Price ($)</label>
                <input
                  type="number"
                  step="0.01"
                  value={groupPrice}
                  onChange={(e) => setGroupPrice(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-500 focus:border-transparent outline-none transition-all"
                  placeholder="e.g. 15.00"
                />
              </div>
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Required Buyers</label>
                <input
                  type="number"
                  value={requiredBuyers}
                  onChange={(e) => setRequiredBuyers(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-500 focus:border-transparent outline-none transition-all"
                  placeholder="e.g. 5"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Time Limit (Hours)</label>
                <input
                  type="number"
                  value={timeLimit}
                  onChange={(e) => setTimeLimit(e.target.value)}
                  className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-500 focus:border-transparent outline-none transition-all"
                  placeholder="e.g. 24"
                />
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Theme</label>
              <div className="flex gap-4">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="radio"
                    name="theme"
                    value="light"
                    checked={theme === 'light'}
                    onChange={() => setTheme('light')}
                    className="w-5 h-5 text-pink-600 focus:ring-pink-500"
                  />
                  <span className="text-gray-700 dark:text-gray-300">Light</span>
                </label>
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="radio"
                    name="theme"
                    value="dark"
                    checked={theme === 'dark'}
                    onChange={() => setTheme('dark')}
                    className="w-5 h-5 text-pink-600 focus:ring-pink-500"
                  />
                  <span className="text-gray-700 dark:text-gray-300">Dark</span>
                </label>
              </div>
            </div>

            <div className="pt-4 border-t border-gray-100 dark:border-gray-700">
              <label className="flex items-center justify-between cursor-pointer">
                <div>
                  <span className="block text-sm font-medium text-gray-900 dark:text-white">Remove OHC Branding</span>
                  <span className="block text-xs text-gray-500">Requires Pro plan</span>
                </div>
                <div className="relative">
                  <input
                    type="checkbox"
                    className="sr-only peer"
                    checked={hideBranding}
                    onChange={handleBrandingToggle}
                  />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-pink-300 dark:peer-focus:ring-pink-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-pink-600"></div>
                </div>
              </label>
            </div>

            <div className="pt-4">
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Embed Code</label>
              <div className="relative">
                <textarea
                  readOnly
                  value={embedCode}
                  className="w-full h-32 px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 text-gray-500 dark:text-gray-400 font-mono text-xs focus:ring-2 focus:ring-pink-500 outline-none resize-none"
                />
                <button
                  onClick={handleCopy}
                  className="absolute top-3 right-3 px-4 py-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg text-sm font-medium hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors shadow-sm"
                >
                  {copied ? 'Copied!' : 'Copy Code'}
                </button>
              </div>
            </div>
          </div>

          {/* Preview */}
          <div className="relative flex flex-col items-center justify-center p-8 bg-gray-100 dark:bg-gray-900 rounded-[32px] overflow-hidden border border-gray-200 dark:border-gray-800 min-h-[500px]">
            <div className="absolute top-4 left-4 bg-white/80 dark:bg-black/50 backdrop-blur-md px-3 py-1 rounded-full text-xs font-semibold text-gray-500 dark:text-gray-400">
              Live Preview
            </div>

            {/* Widget Preview Shell */}
            <div className={`w-full max-w-sm rounded-[24px] shadow-2xl overflow-hidden transition-colors duration-300 ${theme === 'dark' ? 'bg-[#1D1D1F]' : 'bg-white'}`}>
              <div className="p-6">
                <div className={`text-xs font-bold uppercase tracking-wider mb-2 ${theme === 'dark' ? 'text-pink-400' : 'text-pink-600'}`}>
                  ⚡ Viral Group Buy
                </div>
                <h3 className={`text-xl font-bold font-outfit mb-2 ${theme === 'dark' ? 'text-white' : 'text-gray-900'}`}>
                  {productName || 'Premium Product'}
                </h3>

                <div className="flex items-end gap-3 mb-6">
                  <div className={`text-3xl font-black ${theme === 'dark' ? 'text-white' : 'text-gray-900'}`}>
                    ${groupPrice || '0.00'}
                  </div>
                  <div className={`text-lg line-through mb-1 ${theme === 'dark' ? 'text-gray-500' : 'text-gray-400'}`}>
                    ${originalPrice || '0.00'}
                  </div>
                  <div className="bg-pink-100 text-pink-700 text-xs font-bold px-2 py-1 rounded-full mb-1">
                    Save ${((parseFloat(originalPrice) || 0) - (parseFloat(groupPrice) || 0)).toFixed(2)}
                  </div>
                </div>

                <div className={`p-4 rounded-2xl mb-6 ${theme === 'dark' ? 'bg-gray-800' : 'bg-gray-50'}`}>
                  <div className="flex justify-between items-center mb-2">
                    <span className={`text-sm font-semibold ${theme === 'dark' ? 'text-gray-300' : 'text-gray-700'}`}>
                      Unlock Progress
                    </span>
                    <span className={`text-sm font-bold ${theme === 'dark' ? 'text-pink-400' : 'text-pink-600'}`}>
                      1 / {requiredBuyers || 5} Buyers
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 mb-2 overflow-hidden">
                    <div className="bg-gradient-to-r from-pink-500 to-rose-500 h-3 rounded-full" style={{ width: `${(1 / parseInt(requiredBuyers || '5')) * 100}%` }}></div>
                  </div>
                  <div className={`text-xs text-center ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                    ⏳ {timeLimit || 24} hours left to unlock deal
                  </div>
                </div>

                <button className="w-full py-4 bg-gradient-to-r from-pink-500 to-rose-600 hover:from-pink-600 hover:to-rose-700 text-white rounded-xl font-bold text-lg shadow-lg shadow-pink-500/30 transition-all transform hover:scale-[1.02] active:scale-95">
                  Join Group Buy
                </button>

                <div className="mt-4 text-center">
                  <button className={`text-sm font-semibold underline decoration-2 underline-offset-2 ${theme === 'dark' ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'}`}>
                    Share with friends
                  </button>
                </div>
              </div>

              {!hideBranding && (
                <div className={`py-3 text-center border-t ${theme === 'dark' ? 'bg-gray-800 border-gray-700' : 'bg-gray-50 border-gray-100'}`}>
                  <PoweredByOHC tenantId={tenant} className={theme === 'dark' ? 'text-gray-400' : 'text-gray-500'} />
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
          <div className="bg-white dark:bg-gray-800 rounded-3xl p-8 max-w-md w-full shadow-2xl transform transition-all">
            <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-white text-2xl mx-auto mb-6 shadow-lg shadow-indigo-500/30">
              ✨
            </div>
            <h3 className="text-2xl font-bold text-center text-gray-900 dark:text-white mb-2 font-outfit">Upgrade to Pro</h3>
            <p className="text-gray-600 dark:text-gray-400 text-center mb-8">
              Removing OHC branding from widgets is a Pro feature. Upgrade to unlock this and other advanced growth tools.
            </p>
            <div className="space-y-3">
              <Link
                href={`/onboarding?ref=${tenant}&source=group_buy_widget_paywall`}
                className="block w-full py-3.5 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-xl font-bold text-center hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors"
              >
                Upgrade to Pro
              </Link>
              <button
                onClick={() => { setShowPaywall(false); setHideBranding(false); }}
                className="block w-full py-3.5 bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white rounded-xl font-bold text-center hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
              >
                Keep Branding
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
