"use client";

import React, { useState } from 'react';

interface PostCheckoutShareProps {
  referralLink: string;
  isSubscription: boolean;
}

export default function PostCheckoutShare({ referralLink, isSubscription }: PostCheckoutShareProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    if (referralLink) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const shareText = isSubscription
    ? `I just subscribed to an amazing service! Use my link to get 10% off your first month: ${referralLink} ⚡ Powered by OHC`
    : `I just bought an amazing product from this store! Use my link to get 10% off your first order: ${referralLink} ⚡ Powered by OHC`;

  const handleWhatsApp = () => {
    window.open(`https://wa.me/?text=${encodeURIComponent(shareText)}`, '_blank');
  };

  const handleX = () => {
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, '_blank');
  };

  const handleFacebook = () => {
    window.open(`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(referralLink)}&quote=${encodeURIComponent(shareText)}`, '_blank');
  };

  return (
    <div className="flex flex-col gap-4">
      {isSubscription ? (
        <p className="text-gray-600 mb-2 text-sm leading-relaxed">
          You're in! We'll text you a magic link to manage your subscription anytime. Love what you bought? Share with your friends! When they buy, they get 10% off and you earn a <strong className="text-gray-900">$10 credit</strong>.
        </p>
      ) : (
        <p className="text-gray-600 mb-2 text-sm leading-relaxed">
          Your order is confirmed. Love what you bought? Share with your friends! When they buy, they get 10% off and you earn a <strong className="text-gray-900">$10 credit</strong>.
        </p>
      )}

      <div>
        <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Your Unique Link</label>
        <div className="flex gap-2">
          <input
            type="text"
            readOnly
            value={referralLink}
            className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none focus:ring-2 focus:ring-green-500"
          />
          <button
            onClick={handleCopy}
            className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all shadow-sm active:scale-95 ${copied ? 'bg-green-100 text-green-700 border border-green-200' : 'bg-gray-900 text-white hover:bg-black border border-gray-900'}`}
          >
            {copied ? 'Copied!' : 'Copy'}
          </button>
        </div>
      </div>

      <div className="relative py-2">
        <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
        <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">Or Share Via</span></div>
      </div>

      <div className="flex flex-col gap-3">
        <button
          onClick={handleWhatsApp}
          className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all active:scale-[0.98]"
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
          WhatsApp
        </button>
        <button
          onClick={handleX}
          className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all active:scale-[0.98]"
        >
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
          X (Twitter)
        </button>
        <button
          onClick={handleFacebook}
          className="flex items-center justify-center gap-2 bg-[#1877F2]/80 text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#166fe5] transition-all active:scale-[0.98]"
        >
          <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.469h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.469h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/></svg>
          Share on Facebook
        </button>
      </div>
    </div>
  );
}
