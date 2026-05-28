"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

interface PremiumSoftPaywallProps {
  onDismiss?: () => void;
  featureName?: string;
}

export function PremiumSoftPaywall({ onDismiss, featureName = "Premium Features" }: PremiumSoftPaywallProps) {
  const router = useRouter();

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-black/50 backdrop-blur-sm transition-all duration-300">
      <div className="bg-white w-full max-w-lg rounded-3xl p-6 sm:p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 flex flex-col gap-6 transform transition-all">
        {/* Background Decorative Elements */}
        <div className="absolute top-0 right-0 w-48 h-48 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 rounded-bl-full -z-10 blur-2xl"></div>
        <div className="absolute bottom-0 left-0 w-32 h-32 bg-pink-500/10 rounded-tr-full -z-10 blur-xl"></div>

        {/* Header & Close Button */}
        <div className="flex justify-between items-start">
          <div className="w-14 h-14 bg-gradient-to-br from-indigo-50 to-purple-50 rounded-2xl flex items-center justify-center text-2xl shadow-inner border border-indigo-100/50">
            ✨
          </div>
          {onDismiss && (
            <button
              onClick={onDismiss}
              className="p-2 text-gray-400 hover:text-gray-700 hover:bg-gray-100 rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500"
              aria-label="Dismiss"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          )}
        </div>

        {/* Main Content */}
        <div>
          <h2 className="text-2xl sm:text-3xl font-bold font-outfit text-gray-900 mb-3 tracking-tight">
            Unlock {featureName}
          </h2>
          <p className="text-sm sm:text-base text-gray-600 leading-relaxed">
            You've reached the limit of your current plan. Upgrade to the Pro Plan to supercharge your business growth.
          </p>
        </div>

        {/* Feature List */}
        <div className="bg-indigo-50/50 rounded-2xl p-4 border border-indigo-100/50">
          <ul className="space-y-3">
            {[
              "Unlimited AI Marketing Agents",
              "Advanced Automated Review Campaigns",
              "Smart Cart Abandonment Recovery",
              "0% Additional Transaction Fees"
            ].map((feature, i) => (
              <li key={i} className="flex items-center gap-3 text-sm text-gray-700">
                <div className="flex-shrink-0 w-5 h-5 rounded-full bg-indigo-500 flex items-center justify-center text-white">
                  <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 13l4 4L19 7" />
                  </svg>
                </div>
                <span className="font-medium">{feature}</span>
              </li>
            ))}
          </ul>
        </div>

        {/* CTA Section */}
        <div className="flex flex-col gap-3 mt-2">
          <button
            onClick={() => router.push('/pricing')}
            className="w-full py-4 px-6 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white text-lg font-bold rounded-xl shadow-lg shadow-indigo-200 hover:shadow-xl hover:shadow-indigo-300 transition-all transform hover:-translate-y-0.5 active:translate-y-0 flex items-center justify-center gap-2"
          >
            Upgrade to Pro 🚀
          </button>
          <p className="text-center text-xs text-gray-500 font-medium">
            Starting at $79/mo. Cancel anytime.
          </p>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
