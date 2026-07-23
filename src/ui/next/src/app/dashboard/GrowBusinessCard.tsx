"use client";

import React from 'react';
import Link from 'next/link';

export function GrowBusinessCard() {
  return (
    <div className="rounded-[24px] shadow-sm overflow-hidden mt-6 mb-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 transform transition-all hover:scale-[1.01]">
      <div className="bg-gradient-to-r from-blue-500 to-cyan-500 p-1"></div>
      <div className="p-6">
        <div className="flex justify-between items-start">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl">
              🚀
            </div>
            <div>
              <h3 className="text-lg font-bold text-gray-900 dark:text-[#F5F5F7] font-outfit">Grow Business</h3>
              <p className="text-sm text-gray-600 dark:text-gray-300 mt-1">Deploy a zero-config edge-cached storefront for instant consumer discovery or build a viral widget.</p>
            </div>
          </div>
          <div className="flex flex-wrap gap-2">

            <Link
              id="streak-widget-btn"
              href="/viral-streak-widget"
              className="px-4 py-2 bg-orange-50 dark:bg-orange-900/30 hover:bg-orange-100 dark:hover:bg-orange-800/50 text-orange-700 dark:text-orange-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-orange-800/30 shadow-sm"
            >
              Streak Widget
            </Link>
            <Link
              id="goal-tracker-btn"
              href="/viral-goal-tracker"
              className="px-4 py-2 bg-emerald-50 dark:bg-emerald-900/30 hover:bg-emerald-100 dark:hover:bg-emerald-800/50 text-emerald-700 dark:text-emerald-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-emerald-800/30 shadow-sm"
            >
              Goal Tracker
            </Link>
            <Link
              id="promoter-agent-generator-btn"
              href="/viral-post-generator"
              className="px-4 py-2 bg-purple-50 dark:bg-purple-900/30 hover:bg-purple-100 dark:hover:bg-purple-800/50 text-purple-700 dark:text-purple-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-purple-800/30 shadow-sm"
            >
              Promoter Agent
            </Link>
            <Link
              id="giveaway-btn"
              href="/giveaway"
              className="px-4 py-2 bg-blue-50 dark:bg-blue-900/30 hover:bg-blue-100 dark:hover:bg-blue-800/50 text-blue-700 dark:text-blue-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-blue-800/30 shadow-sm"
            >
              Giveaway
            </Link>
            <Link
              id="group-buy-widget-btn"
              href="/group-buy-widget"
              className="px-4 py-2 bg-rose-50 dark:bg-rose-900/30 hover:bg-rose-100 dark:hover:bg-rose-800/50 text-rose-700 dark:text-rose-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-rose-800/30 shadow-sm"
            >
              Group Buy
            </Link>
            <Link
              id="mystery-discount-btn"
              href="/mystery-discount-generator"
              className="px-4 py-2 bg-pink-50 dark:bg-pink-900/30 hover:bg-pink-100 dark:hover:bg-pink-800/50 text-pink-700 dark:text-pink-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-pink-800/30 shadow-sm"
            >
              Mystery Discount
            </Link>
            <Link
              id="give-get-widget-btn"
              href="/viral-give-get-widget"
              className="px-4 py-2 bg-yellow-50 dark:bg-yellow-900/30 hover:bg-yellow-100 dark:hover:bg-yellow-800/50 text-yellow-700 dark:text-yellow-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-yellow-800/30 shadow-sm"
            >
              Give/Get Widget
            </Link>
            <Link
              id="viral-widget-btn"
              href="/viral-powered-by-ohc-widget"
              className="px-4 py-2 bg-indigo-50 dark:bg-indigo-900/30 hover:bg-indigo-100 dark:hover:bg-indigo-800/50 text-indigo-700 dark:text-indigo-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-indigo-800/30 shadow-sm"
            >
              Viral Widget
            </Link>
            <Link
              id="viral-roi-calculator-btn"
              href="/viral-roi-calculator"
              className="px-4 py-2 bg-teal-50 dark:bg-teal-900/30 hover:bg-teal-100 dark:hover:bg-teal-800/50 text-teal-700 dark:text-teal-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-teal-800/30 shadow-sm"
            >
              ROI Calculator
            </Link>
            <Link
              id="digital-business-card-btn"
              href="/digital-business-card"
              className="px-4 py-2 bg-pink-50 dark:bg-pink-900/30 hover:bg-pink-100 dark:hover:bg-pink-800/50 text-pink-700 dark:text-pink-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-pink-800/30 shadow-sm"
            >
              Digital Business Card
            </Link>
            <Link
              id="review-storefront-btn"
              href="/edge-storefront-setup"
              className="px-4 py-2 bg-[#0071E3] hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors whitespace-nowrap shadow-md shadow-blue-200 dark:shadow-none"
            >
              Review Storefront
            </Link>
            <Link
              id="event-rsvp-btn"
              href="/event-rsvp-builder"
              className="px-4 py-2 bg-emerald-50 dark:bg-emerald-900/30 hover:bg-emerald-100 dark:hover:bg-emerald-800/50 text-emerald-700 dark:text-emerald-300 rounded-lg text-sm font-medium transition-colors whitespace-nowrap border border-transparent dark:border-emerald-800/30 shadow-sm"
            >
              Event RSVP
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
