"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';
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
    if (data.organization_id) {
      localStorage.setItem('tenant_id', data.organization_id);
      localStorage.setItem('tenant', data.organization_id);
    }
    if (data.user_id) {
      localStorage.setItem('user_id', data.user_id);
    }
    localStorage.setItem('has_onboarded', 'true');
    router.push('/dashboard');
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-outfit selection:bg-indigo-100 selection:text-indigo-900">
      <div className="w-full max-w-2xl">
        <div className="text-center mb-10">
          <div className="inline-flex items-center justify-center p-3 bg-indigo-100 dark:bg-indigo-900/30 rounded-2xl mb-4">
            <span className="text-3xl">✨</span>
          </div>
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white tracking-tight mb-3">
            Zero-Click Business Generator
          </h1>
          <p className="text-lg text-gray-600 dark:text-gray-400 max-w-xl mx-auto">
            Instantly build your storefront, product catalog, and booking system with a single prompt.
          </p>
        </div>

        <OnboardingChatAgent onComplete={handleChatComplete} />

        <div className="text-center mt-8">
          <p className="text-sm font-semibold text-gray-500 flex items-center justify-center gap-1">
            ⚡ Powered by OHC
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
