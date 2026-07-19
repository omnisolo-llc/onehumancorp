"use client";

import React, { useState } from 'react';

interface PostPurchaseShareWidgetProps {
  tenantId: string;
  orderId?: string;
  storeName?: string;
}

export function PostPurchaseShareWidget({ tenantId, orderId, storeName = 'Our Store' }: PostPurchaseShareWidgetProps) {
  const [copied, setCopied] = useState(false);
  const referralLink = `https://ohc.app/shop/${tenantId}?ref=post_purchase_${orderId || 'default'}`;

  const recordShare = () => {
    fetch(`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}&source=post_purchase_share`, {
      method: 'POST',
    }).catch(() => undefined);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    recordShare();
    setTimeout(() => setCopied(false), 2000);
  };

  const openShare = (url: string) => {
    window.open(url, '_blank');
    recordShare();
  };

  const shareText = `I just bought something from ${storeName}. Visit the store: ${referralLink}`;

  return (
    <div className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-lg mt-6 mb-6">
      <div className="flex flex-col md:flex-row gap-6 items-center w-full">
        <div className="flex-1 text-center md:text-left">
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Share your purchase</h2>
          <p className="text-gray-600 dark:text-gray-300 text-sm">
            Copy or share your tracked store link. Any referral reward is applied only after backend verification.
          </p>
        </div>

        <div className="w-full md:w-auto flex flex-col gap-3">
          <div className="flex items-center gap-2 bg-white/50 dark:bg-black/20 p-2 rounded-lg border border-gray-200 dark:border-gray-700">
            <input
              id="post-purchase-share-link"
              type="text"
              readOnly
              value={referralLink}
              className="bg-transparent border-none outline-none text-sm w-full md:w-48 text-gray-700 dark:text-gray-200 px-2 truncate"
            />
            <button onClick={handleCopy} className="px-4 py-2 bg-gray-100 min-h-[44px] text-gray-800 text-sm font-medium rounded-md">
              {copied ? 'Copied!' : 'Copy'}
            </button>
          </div>
          <button
            onClick={() => openShare(`https://wa.me/?text=${encodeURIComponent(shareText)}`)}
            className="w-full min-h-[44px] bg-[#25D366] text-white py-2 px-4 rounded-md text-sm font-semibold"
          >
            Share on WhatsApp
          </button>
          <button
            onClick={() => openShare(`https://twitter.com/intent/tweet?text=${encodeURIComponent(`${shareText}\n\n⚡ Powered by OHC`)}`)}
            className="w-full min-h-[44px] bg-black text-white py-2 px-4 rounded-md text-sm font-semibold"
          >
            Share on X
          </button>
        </div>
      </div>
    </div>
  );
}
