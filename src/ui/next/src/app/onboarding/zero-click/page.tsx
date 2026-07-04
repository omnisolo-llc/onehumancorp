"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../../components/PoweredByOHC';
import { OnboardingChatAgent } from './components/OnboardingChatAgent';

export default function ZeroClickBuilderPage() {
  const router = useRouter();
  const [generatedStore, setGeneratedStore] = useState<any>(null);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleShare = () => {
    const shareText = `I just built my AI-powered business in 30 seconds using OHC! Start your own for free: https://ohc.app/zero-click-builder?ref=new_store \n\n⚡ Powered by OHC`;
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`;
    window.open(shareUrl, '_blank');
  };

  const handleChatComplete = (data: any) => {
    setGeneratedStore(data);
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-outfit selection:bg-indigo-100 selection:text-indigo-900">
      <div className="w-full max-w-2xl">
        <div className="text-center mb-10">
          <div className="inline-flex items-center justify-center p-3 bg-indigo-100 dark:bg-indigo-900/30 rounded-[16px] mb-4">
            <span className="text-3xl">✨</span>
          </div>
          <h1 className="text-4xl font-bold text-[#1D1D1F] dark:text-white tracking-tight mb-3">
            Tell us about your business
          </h1>
          <p className="text-lg text-[#424245] dark:text-[#A1A1A6] max-w-xl mx-auto">
            Instantly build your storefront, product catalog, and booking system with a single prompt.
          </p>
        </div>

        {!generatedStore ? (
          <OnboardingChatAgent onComplete={handleChatComplete} />
        ) : (
          <div className="glassmorphism p-8 mb-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="text-center mb-8">
              <div className="inline-flex items-center justify-center w-16 h-16 bg-green-100 dark:bg-green-900/30 text-green-600 rounded-full mb-4">
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
                </svg>
              </div>
              <h2 className="text-3xl font-bold text-[#1D1D1F] dark:text-white mb-2">
                Your business is live!
              </h2>
              <p className="text-[#424245] dark:text-[#A1A1A6]">
                We've configured everything you need to start selling.
              </p>
            </div>

            <div className="space-y-6">
              <div className="w-full h-[500px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] overflow-hidden relative bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%]">
                <iframe
                  src={`/builder?tenant=${generatedStore.organization_id}&preview=true`}
                  className="w-full h-full border-none"
                  title="Live Storefront Preview"
                />
              </div>

              <div className="flex flex-col gap-4 pt-4">
                <button
                  onClick={() => {
                    if (generatedStore.organization_id) {
                      localStorage.setItem('tenant_id', generatedStore.organization_id);
                      localStorage.setItem('tenant', generatedStore.organization_id);
                    }
                    if (generatedStore.user_id) {
                      localStorage.setItem('user_id', generatedStore.user_id);
                    }
                    router.push('/dashboard');
                  }}
                  className="w-full flex items-center justify-center gap-2 bg-[#0066FF] hover:bg-[#005bb5] text-white min-h-[44px] px-6 py-3 rounded-[8px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm hover:shadow-md"
                >
                  🚀 Launch My Store
                </button>

                <button
                  onClick={handleShare}
                  className="w-full flex items-center justify-center gap-2 bg-[#1DA1F2] hover:bg-[#1a91da] text-white min-h-[44px] px-6 py-3 rounded-[8px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm hover:shadow-md"
                >
                  🐦 Share on X (Twitter)
                </button>
              </div>
            </div>
          </div>
        )}

        <div className="text-center mt-8">
          <p className="text-sm font-semibold text-gray-500 flex items-center justify-center gap-1">
            <span id="dashboard-footer-viral-link">⚡ Powered by OHC</span>
            {!hasPro && (
              <a href="/pricing" className="text-indigo-500 hover:text-indigo-600 hover:underline ml-1">
                (Upgrade to remove)
              </a>
            )}
          </p>
          <div className="flex justify-center mt-2">
            <PoweredByOHC tenantId="ohc" />
          </div>
        </div>
      </div>
    </div>
  );
}
