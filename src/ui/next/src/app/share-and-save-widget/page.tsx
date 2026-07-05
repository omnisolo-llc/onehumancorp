"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ShareAndSaveWidgetPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('DEFAULT');
  const [showCode, setShowCode] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const tid = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'DEFAULT';
      setTenantId(tid);
    }
  }, []);

  const handleShareOnTwitter = () => {
    const text = "I'm checking out this amazing store on OHC! Discover more at:";
    const url = `https://ohc.app/store/${tenantId}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}&url=${encodeURIComponent(url)}`, '_blank');

    // Reveal the discount code after clicking share
    setTimeout(() => {
        setShowCode(true);
    }, 1500);
  };

  const copyCode = () => {
    navigator.clipboard.writeText("SHARE10");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      {/* Back Button */}
      <div className="w-full max-w-md mb-6">
        <button
          onClick={() => router.push('/dashboard')}
          className="text-gray-600 hover:text-indigo-600 font-medium text-sm flex items-center gap-2 transition-colors"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Dashboard
        </button>
      </div>

      {/* Widget Container */}
      <div className="ohc-growth-card w-full max-w-md p-8 rounded-[24px] backdrop-blur-[30px] saturate-[210%] bg-white shadow-xl border border-white/50 dark:bg-black/40 dark:border-white/10 flex flex-col items-center text-center">

        {/* Header Icon */}
        <div className="w-16 h-16 bg-gradient-to-tr from-indigo-500 to-purple-500 rounded-2xl flex items-center justify-center text-3xl mb-6 shadow-lg shadow-indigo-500/30">
          💸
        </div>

        <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
          Unlock 10% Off!
        </h1>

        <p className="text-gray-600 dark:text-gray-300 mb-8 px-4 text-sm leading-relaxed">
          Love our products? Share our store with your friends on X (Twitter) and we'll instantly give you a 10% discount code for your next order.
        </p>

        {!showCode ? (
          <button
            onClick={handleShareOnTwitter}
            className="w-full relative group overflow-hidden bg-black text-white font-semibold py-4 px-6 rounded-xl shadow-md transition-transform active:scale-95 flex items-center justify-center gap-3"
          >
            <div className="absolute inset-0 w-full h-full bg-gradient-to-r from-gray-800 to-black opacity-0 group-hover:opacity-100 transition-opacity"></div>
            <svg className="w-5 h-5 relative z-10" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
            <span className="relative z-10">Share on X to Unlock</span>
          </button>
        ) : (
          <div className="w-full flex flex-col items-center animate-in fade-in zoom-in duration-300">
            <p className="text-sm text-[#34C759] font-bold mb-3 flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
              Unlocked!
            </p>
            <div className="w-full flex items-center gap-2 bg-indigo-50 dark:bg-indigo-900/30 border border-indigo-100 dark:border-indigo-800 p-2 rounded-xl">
              <div className="flex-1 text-center font-mono font-bold text-lg tracking-widest text-indigo-700 dark:text-indigo-300 py-2">
                SHARE10
              </div>
              <button
                onClick={copyCode}
                className="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-3 rounded-lg text-sm font-semibold transition-colors flex items-center gap-2"
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>
            <p className="text-xs text-gray-500 mt-4">Apply this code at checkout.</p>
          </div>
        )}

      </div>
    </div>
  );
}
