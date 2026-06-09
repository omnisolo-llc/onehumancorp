"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ShareAndSaveWidgetPage() {
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
    const iframeCode = `<iframe src="${origin}/embed/share-and-save?tenant=${tenant}&theme=${theme}&discount=${discountAmount}${discountType === '%' ? 'pct' : 'flat'}&hideBranding=${removeBranding}" width="100%" height="200" style="border:none;border-radius:16px;overflow:hidden;" title="OHC Share and Save Widget"></iframe>`;
    setEmbedCode(iframeCode);
  }, [tenant, theme, discountAmount, discountType, removeBranding]);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleRemoveBrandingClick = (e: React.MouseEvent) => {
    if (!hasPro) {
      e.preventDefault();
      setShowSoftPaywall(true);
    }
  };

  const handleUpgradeClick = () => {
      router.push('/settings?tab=billing&upgrade=true');
  };

  return (
    <div className="min-h-screen bg-gray-50/50 p-6 md:p-12 font-inter relative overflow-hidden">
      {/* Background Orbs */}
      <div className="absolute top-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-gradient-to-br from-indigo-200/40 to-purple-200/40 blur-3xl -z-10 mix-blend-multiply"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[40%] h-[40%] rounded-full bg-gradient-to-br from-blue-200/40 to-teal-200/40 blur-3xl -z-10 mix-blend-multiply"></div>

      <div className="max-w-5xl mx-auto">
        <header className="mb-10 text-center md:text-left">
            <h1 className="text-3xl md:text-4xl font-semibold tracking-tight text-gray-900 mb-3">
              Share & Save Widget
            </h1>
            <p className="text-gray-500 max-w-2xl text-lg">
                Turn your customers into your best marketers. Let them share your store on social media to earn a discount on their next order.
            </p>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-8">
            {/* Left Column: Configuration Form */}
            <div className="lg:col-span-7 space-y-6">
                <div className="bg-white rounded-2xl p-6 md:p-8 shadow-sm border border-gray-100/60 backdrop-blur-sm relative z-10">
                    <h2 className="text-xl font-semibold text-gray-900 mb-6 flex items-center">
                        <span className="w-8 h-8 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center mr-3 text-sm font-bold">1</span>
                        Configure Incentive
                    </h2>

                    <div className="space-y-6">
                        <div className="grid grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">Discount Value</label>
                                <input
                                    type="number"
                                    value={discountAmount}
                                    onChange={(e) => setDiscountAmount(e.target.value)}
                                    className="w-full px-4 py-2.5 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition-colors"
                                />
                            </div>
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">Type</label>
                                <select
                                    value={discountType}
                                    onChange={(e) => setDiscountType(e.target.value)}
                                    className="w-full px-4 py-2.5 rounded-xl border border-gray-200 focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition-colors bg-white"
                                >
                                    <option value="%">% Off</option>
                                    <option value="$">$ Off</option>
                                </select>
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">Widget Theme</label>
                            <div className="flex gap-3">
                                <button
                                    onClick={() => setTheme('light')}
                                    className={`flex-1 py-3 px-4 rounded-xl border ${theme === 'light' ? 'border-indigo-500 bg-indigo-50 text-indigo-700 ring-1 ring-indigo-500' : 'border-gray-200 hover:bg-gray-50 text-gray-700'} font-medium transition-all`}
                                >
                                    Light Theme
                                </button>
                                <button
                                    onClick={() => setTheme('dark')}
                                    className={`flex-1 py-3 px-4 rounded-xl border ${theme === 'dark' ? 'border-indigo-500 bg-gray-900 text-white ring-1 ring-indigo-500' : 'border-gray-200 hover:bg-gray-50 text-gray-700'} font-medium transition-all`}
                                >
                                    Dark Theme
                                </button>
                            </div>
                        </div>
                    </div>
                </div>

                <div className="bg-white rounded-2xl p-6 md:p-8 shadow-sm border border-gray-100/60 backdrop-blur-sm relative z-10">
                     <h2 className="text-xl font-semibold text-gray-900 mb-6 flex items-center">
                        <span className="w-8 h-8 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center mr-3 text-sm font-bold">2</span>
                        Brand Settings
                    </h2>

                    <label className="flex items-start gap-3 cursor-pointer group" onClickCapture={handleRemoveBrandingClick}>
                        <div className="relative flex items-start">
                            <div className="flex items-center h-5">
                                <input
                                    type="checkbox"
                                    checked={removeBranding}
                                    onChange={(e) => {
                                        if (hasPro) setRemoveBranding(e.target.checked);
                                    }}
                                    className="w-5 h-5 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500 transition-colors cursor-pointer"
                                />
                            </div>
                        </div>
                        <div className="flex flex-col">
                            <span className="text-sm font-medium text-gray-900 group-hover:text-indigo-600 transition-colors">
                                Remove "Powered by OHC" branding
                            </span>
                            <span className="text-xs text-gray-500 mt-1 flex items-center gap-1">
                                {!hasPro && (
                                    <span className="inline-flex items-center justify-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-amber-100 text-amber-800">
                                        PRO
                                    </span>
                                )}
                                Make the widget 100% your own brand.
                            </span>
                        </div>
                    </label>
                </div>
            </div>

            {/* Right Column: Preview & Code */}
            <div className="lg:col-span-5 space-y-6">
                <div className="bg-white rounded-2xl shadow-sm border border-gray-100/60 backdrop-blur-sm overflow-hidden sticky top-6">
                    <div className="p-4 border-b border-gray-100 bg-gray-50/50">
                        <h3 className="text-sm font-semibold text-gray-700 uppercase tracking-wider">Live Preview</h3>
                    </div>

                    <div className="p-6 md:p-8 bg-gray-100/50 flex justify-center items-center min-h-[300px]">
                        {/* Interactive Widget Preview */}
                        <div className={`w-full max-w-sm rounded-2xl shadow-lg border overflow-hidden ${theme === 'dark' ? 'bg-gray-900 border-gray-800 text-white' : 'bg-white border-gray-100 text-gray-900'}`}>
                            <div className="p-6 text-center">
                                <div className={`w-12 h-12 rounded-full mx-auto mb-4 flex items-center justify-center ${theme === 'dark' ? 'bg-indigo-900/50 text-indigo-400' : 'bg-indigo-100 text-indigo-600'}`}>
                                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
                                    </svg>
                                </div>
                                <h4 className="text-lg font-bold mb-2">Love our store?</h4>
                                <p className={`text-sm mb-5 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-500'}`}>
                                    Share us with your friends on social media and get <strong className={theme === 'dark' ? 'text-white' : 'text-gray-900'}>{discountType === '$' ? '$' : ''}{discountAmount}{discountType === '%' ? '%' : ''} off</strong> your next order!
                                </p>

                                <div className="grid grid-cols-2 gap-3 mb-4">
                                    <button className="flex items-center justify-center gap-2 py-2 px-4 rounded-xl bg-blue-600 text-white text-sm font-medium hover:bg-blue-700 transition-colors">
                                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
                                        Share
                                    </button>
                                    <button className="flex items-center justify-center gap-2 py-2 px-4 rounded-xl bg-black text-white text-sm font-medium hover:bg-gray-800 transition-colors">
                                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                                        Post
                                    </button>
                                </div>
                            </div>

                            {/* Powered By Watermark */}
                            {!removeBranding && (
                                <div className={`py-3 text-center text-xs font-medium border-t ${theme === 'dark' ? 'border-gray-800 bg-gray-900/50' : 'border-gray-100 bg-gray-50'}`}>
                                    <a
                                        href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        className={`hover:underline transition-colors ${theme === 'dark' ? 'text-gray-400 hover:text-gray-200' : 'text-gray-500 hover:text-gray-800'}`}
                                    >
                                        ⚡ Powered by OHC
                                    </a>
                                </div>
                            )}
                        </div>
                    </div>

                    <div className="p-6 border-t border-gray-100 bg-white">
                         <h3 className="text-sm font-semibold text-gray-700 mb-3">Embed Code</h3>
                         <div className="relative group">
                            <pre className="bg-gray-900 text-gray-300 p-4 rounded-xl text-xs sm:text-sm overflow-x-auto font-mono leading-relaxed border border-gray-800">
                                <code>{embedCode}</code>
                            </pre>
                            <button
                                onClick={handleCopy}
                                className="absolute top-3 right-3 p-2 bg-white/10 hover:bg-white/20 text-white rounded-lg backdrop-blur-md transition-all opacity-0 group-hover:opacity-100 focus:opacity-100 flex items-center gap-2 text-xs font-medium"
                            >
                                {copied ? (
                                    <>
                                        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                                        Copied!
                                    </>
                                ) : (
                                    <>
                                        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                                        Copy Code
                                    </>
                                )}
                            </button>
                        </div>
                        <p className="text-xs text-gray-500 mt-3 text-center">
                            Paste this code snippet right before the closing <code className="bg-gray-100 px-1 py-0.5 rounded">&lt;/body&gt;</code> tag on your site.
                        </p>
                    </div>
                </div>
            </div>
        </div>
      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-gray-900/40 backdrop-blur-sm z-[100] flex items-center justify-center p-4 sm:p-6 animate-in fade-in duration-200">
          <div className="bg-white w-full max-w-md rounded-3xl p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-50/50">
            {/* Modal Deco */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-indigo-100 to-purple-50 rounded-bl-[100px] -z-10 opacity-60"></div>

            <button
                onClick={() => setShowSoftPaywall(false)}
                className="absolute top-4 right-4 p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-50 rounded-full transition-colors"
            >
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>

            <div className="text-center mt-2">
              <div className="mx-auto w-16 h-16 bg-indigo-50 rounded-2xl flex items-center justify-center mb-6 border border-indigo-100/50">
                <svg className="w-8 h-8 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                </svg>
              </div>
              <h3 className="text-2xl font-bold text-gray-900 mb-3 tracking-tight">Make it Yours</h3>
              <p className="text-gray-600 mb-8 leading-relaxed">
                Upgrade to Pro to remove the <span className="font-semibold text-gray-900">"Powered by OHC"</span> watermark and unlock full white-label customization.
              </p>

              <div className="space-y-3">
                <button
                  onClick={handleUpgradeClick}
                  className="w-full py-3.5 px-4 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-semibold shadow-sm hover:shadow-md transition-all flex items-center justify-center gap-2 group"
                >
                  Upgrade to Pro
                  <svg className="w-4 h-4 group-hover:translate-x-0.5 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>
                </button>
                <button
                  onClick={() => setShowSoftPaywall(false)}
                  className="w-full py-3.5 px-4 bg-white border border-gray-200 hover:bg-gray-50 text-gray-700 rounded-xl font-medium transition-colors"
                >
                  Keep Watermark
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
