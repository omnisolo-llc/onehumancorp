"use client";

import React from 'react';
import Link from 'next/link';

export function GrowBusinessCard() {
  return (
    <div className="glassmorphism bg-white dark:bg-gray-800 rounded-[24px] shadow-sm border border-gray-100 dark:border-gray-700 overflow-hidden mt-6 mb-6">
      <div className="bg-gradient-to-r from-blue-500 to-cyan-500 p-1"></div>
      <div className="p-6">
        <div className="flex justify-between items-start">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl">
              🚀
            </div>
            <div>
              <h3 className="text-lg font-bold text-gray-900 dark:text-white font-outfit">Grow Business</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Deploy a zero-config edge-cached storefront for instant consumer discovery or build a viral widget.</p>
            </div>
          </div>
          <div className="flex gap-2">
            <Link
              id="promoter-agent-generator-btn"
              href="/viral-post-generator"
              className="px-4 py-2 bg-purple-50 hover:bg-purple-100 text-purple-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Promoter Agent
            </Link>
            <Link
              id="giveaway-btn"
              href="/giveaway"
              className="px-4 py-2 bg-blue-50 hover:bg-blue-100 text-blue-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Giveaway
            </Link>
            <Link
              id="viral-widget-btn"
              href="/viral-powered-by-ohc-widget"
              className="px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Viral Widget
            </Link>
            <Link
              id="digital-business-card-btn"
              href="/digital-business-card"
              className="px-4 py-2 bg-pink-50 hover:bg-pink-100 text-pink-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Digital Business Card
            </Link>
            <Link
              id="review-storefront-btn"
              href="/edge-storefront-setup"
              className="px-4 py-2 bg-[#0071E3] hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Review Storefront
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
