'use client';

import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import { useRouter } from 'next/navigation';

export default function TipJarWidgetGenerator() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [displayName, setDisplayName] = useState('Creator Name');
  const [message, setMessage] = useState('Buy me a coffee! Your support helps me create more content.');
  const [amounts, setAmounts] = useState('5, 10, 20');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [removeBranding, setRemoveBranding] = useState(false);

  const [showModal, setShowModal] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/tip-jar/embed?tenant=${encodeURIComponent(tenant)}&name=${encodeURIComponent(displayName)}&message=${encodeURIComponent(message)}&amounts=${encodeURIComponent(amounts)}&theme=${theme}&branding=${!removeBranding}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="320" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
      if (theme === 'dark') {
          return { background: '#1f2937', color: '#f9fafb', border: '1px solid #374151' };
      }
      return { background: '#ffffff', color: '#111827', border: '1px solid #e5e7eb' };
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      <Head>
        <title>Tip Jar Widget Builder | OHC</title>
      </Head>

      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
         <div className="flex items-center gap-3">
             <span className="text-2xl font-bold text-indigo-600">⚡</span>
             <h1 className="text-xl font-bold font-outfit text-gray-900">Tip Jar Builder</h1>
         </div>
         <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-white/80 border border-gray-200 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors shadow-sm text-gray-700">
           Dashboard
         </button>
      </header>

      <main className="flex-1 max-w-6xl mx-auto w-full p-6 md:p-8 flex flex-col md:flex-row gap-8">

        {/* Configuration Panel */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 bg-white border border-gray-200 shadow-sm">
                <h2 className="text-xl font-bold font-outfit mb-6 text-gray-900">Widget Settings</h2>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Display Name</label>
                    <input
                        type="text"
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. Creator Name"
                        value={displayName}
                        onChange={(e) => setDisplayName(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Thank You Message</label>
                    <textarea
                        className="w-full px-3 py-2 border border-gray-300 min-h-[80px] resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="Why are you collecting tips?"
                        value={message}
                        onChange={(e) => setMessage(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Suggested Amounts ($)</label>
                    <input
                        type="text"
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. 5, 10, 20"
                        value={amounts}
                        onChange={(e) => setAmounts(e.target.value)}
                    />
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] transition-all ${theme === 'dark' ? 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6 pt-4 border-t border-gray-100">
                    <label className="flex items-start gap-3 cursor-pointer group">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => {
                                if (e.target.checked) {
                                    setShowSoftPaywall(true);
                                    setRemoveBranding(false);
                                } else {
                                    setRemoveBranding(false);
                                }
                            }}
                            className="mt-1 w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500"
                        />
                        <div>
                            <span className="text-sm font-medium text-gray-900">Remove "Powered by OHC" Badge</span>
                            <p className="text-xs text-gray-500 mt-1">Requires Pro plan or higher.</p>
                        </div>
                    </label>
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-indigo-600 text-white font-medium min-h-[44px] hover:bg-indigo-700 transition-colors shadow-sm"
                >
                    Get Widget Code
                </button>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-white/50 backdrop-blur-[30px] saturate-[210%] border border-white/80 shadow-lg">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                {/* The Widget Preview */}
                <div className="relative w-full max-w-sm shadow-2xl overflow-hidden" style={getThemeStyles()}>
                    <div className="p-6 flex flex-col">
                        <div className="flex justify-center mb-4">
                            <div className="w-16 h-16 rounded-full bg-gradient-to-tr from-indigo-500 to-pink-500 flex items-center justify-center text-2xl shadow-inner">
                                ☕
                            </div>
                        </div>

                        <h3 className="text-xl font-bold text-center mb-2 font-outfit" style={{ color: theme === 'dark' ? '#f9fafb' : '#111827' }}>
                            {displayName}
                        </h3>
                        <p className="text-sm text-center mb-6 leading-relaxed" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>
                            {message}
                        </p>

                        <div className="flex gap-2 justify-center mb-4 flex-wrap">
                            {amounts.split(',').map((amt, idx) => {
                                const val = amt.trim();
                                if (!val) return null;
                                return (
                                    <button
                                        key={idx}
                                        className="px-4 py-2 rounded-full border text-sm font-medium transition-all"
                                        style={{
                                            borderColor: theme === 'dark' ? '#4b5563' : '#e5e7eb',
                                            color: theme === 'dark' ? '#e5e7eb' : '#374151',
                                            background: theme === 'dark' ? '#374151' : '#f9fafb'
                                        }}
                                    >
                                        ${val}
                                    </button>
                                );
                            })}
                        </div>

                        <div className="flex mb-4">
                            <input
                                type="number"
                                placeholder="Custom amount"
                                className="w-full px-4 py-2 border rounded-l-lg text-sm focus:outline-none"
                                style={{
                                    borderColor: theme === 'dark' ? '#4b5563' : '#e5e7eb',
                                    background: theme === 'dark' ? '#1f2937' : '#ffffff',
                                    color: theme === 'dark' ? '#f9fafb' : '#111827'
                                }}
                            />
                            <button className="px-4 py-2 bg-indigo-600 text-white font-medium rounded-r-lg text-sm hover:bg-indigo-700 transition-colors">
                                Tip
                            </button>
                        </div>

                        {!removeBranding && (
                            <div className={`mt-2 pt-3 border-t text-center text-xs ${theme === 'dark' ? 'border-gray-700 text-gray-400' : 'border-gray-100 text-gray-500'}`}>
                                <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="font-bold hover:underline" style={{ color: '#6b7280' }}>
                                    ⚡ Powered by OHC
                                </a>
                            </div>
                        )}
                    </div>
                </div>

                <div className="mt-8 text-center max-w-md text-sm text-gray-500">
                    This preview shows exactly how the widget will look when embedded on your website using the generated iframe code.
                </div>
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="bg-white p-8 max-w-xl w-full shadow-2xl relative animate-fade-in-up">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Tip Jar</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, blog, or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500/20 focus:border-indigo-500 transition-all"
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
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium min-h-[44px] transition-colors"
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
          <div className="bg-white w-full max-w-md p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-indigo-100 flex items-center justify-center text-3xl shadow-inner text-indigo-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the Tip Jar 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and keep all your tips.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700"
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

        @keyframes fade-in-up {
          0% { opacity: 0; transform: translateY(20px); }
          100% { opacity: 1; transform: translateY(0); }
        }
        .animate-fade-in-up { animation: fade-in-up 0.2s ease-out forwards; }
      `}} />
    </div>
  );
}
