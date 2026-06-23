"use client";

import React, { useState } from 'react';

interface ShareAndSaveWidgetProps {
  tenantId: string;
  discountPercentage: number;
  onShareComplete: () => void;
}

export function ShareAndSaveWidget({ tenantId, discountPercentage, onShareComplete }: ShareAndSaveWidgetProps) {
  const [isShared, setIsShared] = useState(false);
  const referralLink = `${typeof window !== 'undefined' ? window.location.origin : ''}/onboarding?ref=${tenantId}&source=checkout_share_save`;
  const shareText = `Check out this awesome store! ${referralLink}`;

  const handleShare = (platform: 'twitter' | 'whatsapp') => {
    let shareUrl = '';
    if (platform === 'twitter') {
      shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`;
    } else if (platform === 'whatsapp') {
      shareUrl = `https://wa.me/?text=${encodeURIComponent(shareText)}`;
    }

    window.open(shareUrl, '_blank');

    // Optimistically apply discount after user clicks share
    setIsShared(true);
    onShareComplete();
  };

  if (isShared) {
    return (
      <div className="mb-6 p-4 rounded-xl glassmorphism border border-green-200/50 dark:border-green-800/30 bg-gradient-to-r from-green-50/80 to-emerald-50/80 dark:from-green-900/20 dark:to-emerald-900/20 shadow-sm" data-testid="share-and-save-success">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-green-100 dark:bg-green-800/50 rounded-full flex items-center justify-center shrink-0">
            <span className="text-green-600 dark:text-green-400 font-bold text-lg">✓</span>
          </div>
          <div>
            <h3 className="text-sm font-bold font-outfit text-gray-900 dark:text-white">Discount Applied!</h3>
            <p className="text-xs text-gray-600 dark:text-gray-300">
              Thanks for sharing. Your {discountPercentage}% discount has been added to your order.
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="mb-6 p-5 rounded-xl glassmorphism border border-indigo-200/60 dark:border-indigo-800/60 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] relative overflow-hidden group" data-testid="share-and-save-widget">
      <div className="flex flex-col sm:flex-row items-center sm:items-start gap-4">
        <div className="w-12 h-12 bg-indigo-100 dark:bg-indigo-900/50 rounded-full flex items-center justify-center shrink-0 shadow-sm border border-indigo-200 dark:border-indigo-800 group-hover:scale-110 transition-transform">
          <span className="text-xl">🎁</span>
        </div>
        <div className="flex-1 text-center sm:text-left">
          <h3 className="text-base font-bold font-outfit text-gray-900 dark:text-white mb-1">
            Share & Save {discountPercentage}%
          </h3>
          <p className="text-xs text-gray-600 dark:text-gray-300 mb-3">
            Share our store with your friends on social media to instantly unlock a {discountPercentage}% discount on this order!
          </p>
          <div className="flex flex-col sm:flex-row gap-2">
            <button
              onClick={() => handleShare('twitter')}
              className="flex-1 py-2 px-3 bg-black hover:bg-gray-800 text-white rounded-lg font-bold text-xs shadow-sm transition-all flex items-center justify-center gap-2"
              data-testid="share-x-btn"
            >
              <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X
            </button>
            <button
              onClick={() => handleShare('whatsapp')}
              className="flex-1 py-2 px-3 bg-[#25D366] hover:bg-[#1ebd5a] text-white rounded-lg font-bold text-xs shadow-sm transition-all flex items-center justify-center gap-2"
              data-testid="share-wa-btn"
            >
              <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
              WhatsApp
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
