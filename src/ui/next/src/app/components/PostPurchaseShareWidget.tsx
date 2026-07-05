"use client";

import React, { useState } from 'react';

interface PostPurchaseShareWidgetProps {
  tenantId: string;
  orderId?: string;
  storeName?: string;
}

export function PostPurchaseShareWidget({ tenantId, orderId, storeName = 'Our Store' }: PostPurchaseShareWidgetProps) {
  const [copied, setCopied] = useState(false);
  const [unlocked, setUnlocked] = useState(false);
  const referralLink = `https://ohc.app/shop/${tenantId}?ref=post_purchase_${orderId || 'default'}`;

  const handleUnlock = () => {
    // Track click
    fetch(`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}&source=post_purchase_share`, {
      method: 'POST'
    }).catch(() => {});
    setUnlocked(true);
  };

  const handleCopy = () => {
    if (referralLink) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      handleUnlock();
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleWhatsApp = () => {
    if (referralLink) {
      const url = `https://wa.me/?text=${encodeURIComponent(
        `I just bought something awesome from ${storeName}! Use my link to get 10% off your first order: ${referralLink}`
      )}`;
      window.open(url, '_blank');
      handleUnlock();
    }
  };

  const handleTwitter = () => {
    if (referralLink) {
      const url = `https://twitter.com/intent/tweet?text=${encodeURIComponent(
        `I just bought something awesome from ${storeName}! Use my link to get 10% off your first order: ${referralLink}\n\n⚡ Powered by OHC`
      )}`;
      window.open(url, '_blank');
      handleUnlock();
    }
  };

  return (
    <div className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-lg mt-6 mb-6">
      {unlocked ? (
        <div className="flex flex-col md:flex-row gap-6 items-center bg-gradient-to-r from-green-50/80 to-emerald-50/80 dark:from-green-900/20 dark:to-emerald-900/20 p-4 rounded-xl border border-green-400/50">
           <div className="w-12 h-12 bg-green-100 dark:bg-green-800 rounded-full flex items-center justify-center shrink-0">
                 <span className="text-2xl">✨</span>
           </div>
           <div className="flex-1">
              <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-1">
                 VIP Concierge Unlocked!
              </h3>
              <p className="text-sm text-gray-600 dark:text-gray-300">
                 You now have 7 days of free access to our VIP AI Concierge for your next order. Thank you for sharing!
              </p>
           </div>
        </div>
      ) : (
        <div className="flex flex-col md:flex-row gap-6 items-center w-full">
        <div className="flex-1 text-center md:text-left">
          <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 text-sm font-semibold border border-indigo-200 dark:border-indigo-800">
             <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
             Pro Feature
          </div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
            Unlock VIP Concierge
          </h2>
          <p className="text-gray-600 dark:text-gray-300 text-sm flex items-center gap-2 justify-center md:justify-start">
            <svg className="w-4 h-4 text-indigo-500 flex-shrink-0" width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            Love your purchase? Share this link with friends to unlock VIP Concierge for your next order.
          </p>
        </div>

        {!unlocked && (
        <div className="w-full md:w-auto">
          <div className="flex flex-col gap-3 w-full md:w-auto">
            <div className="flex items-center gap-2 bg-white dark:bg-black/20 p-2 rounded-lg border border-gray-200 dark:border-gray-700">
              <input
                id="post-purchase-share-link"
                type="text"
                readOnly
                value={referralLink}
                className="bg-transparent border-none outline-none text-sm w-full md:w-48 text-gray-700 dark:text-gray-200 px-2 truncate"
              />
              <button
                onClick={handleCopy}
                className="px-4 py-2 bg-gray-100 min-h-[44px] hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-medium rounded-md transition-colors whitespace-nowrap"
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
            </div>

            <button
              onClick={handleWhatsApp}
              className="w-full app-button min-h-[44px] bg-[#25D366] hover:bg-[#1ebd5a] text-white border-none py-2 px-4 rounded-md text-sm font-semibold transition-colors flex items-center justify-center gap-2"
            >
              Share on WhatsApp
            </button>

            <button
              onClick={handleTwitter}
              className="w-full app-button min-h-[44px] bg-black hover:bg-gray-800 text-white border-none py-2 px-4 rounded-md text-sm font-semibold transition-all shadow-sm flex items-center justify-center gap-2"
            >
              <svg className="w-4 h-4" width="16" height="16" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X (Twitter)
            </button>
          </div>
        </div>
        )}
      </div>
      )}
    </div>
  );
}
