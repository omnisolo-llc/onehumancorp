"use client";

import React from 'react';
import Link from 'next/link';

export function GrowBusinessCard() {
  return (
    <div className="rounded-[24px] shadow-sm overflow-hidden mt-6 mb-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10">
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
          <div className="flex flex-wrap gap-2">

            <Link
              id="goal-tracker-btn"
              href="/viral-goal-tracker"
              className="px-4 py-2 bg-emerald-50 hover:bg-emerald-100 text-emerald-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Goal Tracker
            </Link>
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
              id="group-buy-widget-btn"
              href="/group-buy-widget"
              className="px-4 py-2 bg-rose-50 hover:bg-rose-100 text-rose-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Group Buy
            </Link>
            <Link
              id="mystery-discount-btn"
              href="/mystery-discount-generator"
              className="px-4 py-2 bg-pink-50 hover:bg-pink-100 text-pink-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Mystery Discount
            </Link>
            <Link
              id="give-get-widget-btn"
              href="/viral-give-get-widget"
              className="px-4 py-2 bg-yellow-50 hover:bg-yellow-100 text-yellow-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Give/Get Widget
            </Link>
            <Link
              id="viral-widget-btn"
              href="/viral-powered-by-ohc-widget"
              className="px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Viral Widget
            </Link>
            <Link
              id="viral-roi-calculator-btn"
              href="/viral-roi-calculator"
              className="px-4 py-2 bg-teal-50 hover:bg-teal-100 text-teal-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              ROI Calculator
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
            <Link
              id="event-rsvp-btn"
              href="/event-rsvp-builder"
              className="px-4 py-2 bg-emerald-50 hover:bg-emerald-100 text-emerald-700 rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
            >
              Event RSVP
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
