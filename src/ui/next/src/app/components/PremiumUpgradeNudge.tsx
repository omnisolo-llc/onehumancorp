import React, { useState } from 'react';

export default function PremiumUpgradeNudge() {
  const [isVisible, setIsVisible] = useState(true);

  if (!isVisible) return null;

  return (
    <div className="fixed bottom-6 right-6 z-50 animate-fade-in-up">
      <div className="glassmorphism rounded-[20px] p-5 shadow-2xl border border-white/20 w-80 relative overflow-hidden bg-white/80 dark:bg-black/60 backdrop-blur-xl">
        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/20 blur-3xl -z-10 rounded-full pointer-events-none" />
        <div className="absolute bottom-0 left-0 w-24 h-24 bg-purple-500/20 blur-2xl -z-10 rounded-full pointer-events-none" />

        <button
          onClick={() => setIsVisible(false)}
          className="absolute top-3 right-3 text-gray-500 hover:text-gray-900 dark:hover:text-gray-200 transition-colors"
          aria-label="Dismiss"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>

        <div className="flex items-center gap-3 mb-3">
          <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-indigo-500 to-purple-500 flex items-center justify-center text-white shadow-lg">
            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <div>
            <h3 className="text-sm font-bold font-outfit text-gray-900 dark:text-white leading-tight">Unlock AI Superpowers</h3>
            <p className="text-xs text-gray-500 dark:text-gray-400">OHC Pro Plan</p>
          </div>
        </div>

        <p className="text-xs text-gray-600 dark:text-gray-300 mb-4 leading-relaxed font-inter">
          Upgrade to get unlimited AI agents, zero branding, and automated review campaigns.
        </p>

        <a
          href="/pricing"
          className="block w-full py-2.5 px-4 bg-gray-900 hover:bg-black dark:bg-white dark:hover:bg-gray-100 text-white dark:text-black text-xs font-semibold rounded-xl text-center shadow-md transition-all hover:shadow-lg active:scale-95"
        >
          View Pro Features
        </a>
      </div>
    </div>
  );
}
