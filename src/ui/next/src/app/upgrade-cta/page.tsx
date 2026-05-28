"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { PremiumSoftPaywall } from '../../components/PremiumSoftPaywall';

export default function UpgradeCTAPage() {
  const router = useRouter();
  const [showPaywall, setShowPaywall] = useState(false);

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Growth Tools</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8 items-center justify-center">
        <div className="text-center max-w-lg mb-8">
            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-4">Launch Advanced Campaigns</h2>
            <p className="text-gray-600 mb-8">
              Simulate triggering a premium feature to see the high-converting soft paywall in action.
            </p>
            <button
              onClick={() => setShowPaywall(true)}
              className="px-8 py-4 bg-indigo-600 text-white rounded-xl font-bold text-lg hover:bg-indigo-700 transition-colors shadow-lg shadow-indigo-200"
            >
              Simulate Action
            </button>
        </div>

        {/* The Soft Paywall Component */}
        {showPaywall && (
          <PremiumSoftPaywall
            featureName="Advanced Analytics"
            onDismiss={() => setShowPaywall(false)}
          />
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
