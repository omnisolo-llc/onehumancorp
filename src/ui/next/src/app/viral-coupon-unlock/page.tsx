"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import '../globals.css';

import { PoweredByOHC } from "../components/PoweredByOHC";

export default function ViralCouponUnlockPage() {
  const router = useRouter();
  const [tenant] = useState('my-business');
  const [offerName, setOfferName] = useState('20% Off Your First Order');
  const [couponCode, setCouponCode] = useState('WELCOME20');
  const [sharesRequired, setSharesRequired] = useState(3);
  const [copied, setCopied] = useState(false);

  const generatedLink = `https://ohc.app/unlock/${tenant.toLowerCase().replace(/\s+/g, '-')}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Share-to-Unlock Coupon 🎁</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Coupon Settings</h2>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Offer Headline</label>
              <input
                type="text"
                value={offerName}
                onChange={(e) => setOfferName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="e.g. 20% Off Your First Order"
              />
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Hidden Coupon Code</label>
              <input
                type="text"
                value={couponCode}
                onChange={(e) => setCouponCode(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                placeholder="e.g. WELCOME20"
              />
            </div>

            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">Shares Required to Unlock</label>
              <input
                type="number"
                min="1"
                max="10"
                value={sharesRequired}
                onChange={(e) => setSharesRequired(parseInt(e.target.value) || 1)}
                className="w-full px-3 py-2 border border-gray-300/50 rounded-lg bg-white/50 backdrop-blur-sm min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
              />
            </div>
          </div>

          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-indigo-50/70 backdrop-blur-[40px] border border-indigo-100/50 rounded-2xl mt-6">
            <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
              <span className="text-xl">🚀</span> Share Your Link
            </h3>
            <p className="text-sm text-indigo-800 mb-4">
              Post this link to your audience. They must share it with {sharesRequired} friends to unlock the coupon code!
            </p>

            <div className="flex items-center gap-2 bg-white min-h-[44px] min-w-[44px] border border-indigo-200 p-1 mb-4 overflow-hidden">
              <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono">
                {generatedLink}
              </div>
            </div>

            <button
              onClick={handleCopy}
              className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors"
            >
              {copied ? 'Copied!' : 'Copy Link'}
            </button>
          </div>
        </div>

        <div className="w-full md:w-2/3 flex flex-col">
          <div className="flex-1 shadow-[0_20px_40px_rgb(0,0,0,0.15)] overflow-hidden flex flex-col bg-white/40 backdrop-blur-[40px] saturate-[200%] border border-white/50 rounded-2xl relative">
            <div className="bg-gray-200 py-3 px-4 flex items-center gap-2 border-b border-gray-300">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="mx-auto bg-white/60 text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate">
                Preview: Unlock Page
              </div>
            </div>

            <div className="flex-1 flex items-center justify-center p-8 bg-gradient-to-br from-indigo-500 to-purple-600 overflow-y-auto">
              <div
                className="w-full max-w-md shadow-2xl p-10 flex flex-col items-center relative overflow-hidden transition-all duration-300 rounded-3xl bg-white"
              >
                <div className="w-20 h-20 bg-pink-100 text-pink-500 rounded-full flex items-center justify-center text-4xl mb-6 shadow-inner">
                  🎁
                </div>
                <h2 className="text-3xl font-bold font-outfit text-center mb-2 text-gray-900">
                  {offerName || 'Special Offer!'}
                </h2>
                <p className="text-center text-gray-600 mb-8 font-medium">
                  Unlock this exclusive coupon by sharing with {sharesRequired} friends!
                </p>

                <div className="w-full mb-8 relative">
                    <div className="h-4 bg-gray-100 rounded-full overflow-hidden border border-gray-200">
                        <div className="h-full bg-gradient-to-r from-pink-500 to-purple-500 w-1/3 rounded-full transition-all duration-1000"></div>
                    </div>
                    <div className="absolute top-6 left-0 right-0 flex justify-between text-xs font-bold text-gray-400 uppercase tracking-wide">
                        <span>0 Shares</span>
                        <span className="text-pink-600">1 / {sharesRequired}</span>
                        <span>{sharesRequired} Shares</span>
                    </div>
                </div>

                <div className="w-full space-y-3 mb-8 mt-4">
                  <button className="w-full py-4 bg-gray-900 hover:bg-gray-800 text-white font-bold rounded-xl min-h-[56px] transition-all flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path></svg>
                    Share on X
                  </button>
                  <button className="w-full py-4 bg-[#25D366] hover:bg-[#1ebe53] text-white font-bold rounded-xl min-h-[56px] transition-all flex items-center justify-center gap-3 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                    Share on WhatsApp
                  </button>
                </div>

                <div className="w-full relative mt-4">
                    <div className="absolute inset-0 bg-gray-100 blur-sm rounded-xl"></div>
                    <div className="relative w-full p-4 border-2 border-dashed border-gray-300 bg-gray-50/80 rounded-xl text-center select-none flex flex-col items-center justify-center h-[80px]">
                        <p className="text-gray-400 font-bold uppercase tracking-widest text-lg blur-[4px]">
                            {couponCode || 'LOCKED'}
                        </p>
                        <div className="absolute inset-0 flex items-center justify-center">
                            <span className="bg-white/90 px-3 py-1 rounded-full text-xs font-bold text-gray-700 shadow-sm border border-gray-200">
                                🔒 Locked
                            </span>
                        </div>
                    </div>
                </div>

                <div className="mt-8 pt-6 border-t w-full text-center border-gray-100">
                  <PoweredByOHC tenantId="growth" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
