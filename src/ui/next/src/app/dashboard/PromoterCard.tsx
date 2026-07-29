"use client";

import React from 'react';
import Link from 'next/link';

export function PromoterCard() {
  return (
    <div className="rounded-[24px] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] shadow-sm overflow-hidden mt-6 mb-6">
      <div className="bg-gradient-to-r from-indigo-500 to-purple-500 p-1"></div>
      <div className="p-6">
        <div className="flex justify-between items-start">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-indigo-50/50 dark:bg-indigo-900/30 border border-indigo-100/50 dark:border-indigo-800/50 flex items-center justify-center text-2xl">
              📣
            </div>
            <div>
              <h3 className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit">The Promoter Agent</h3>
              <p className="text-sm text-gray-600 dark:text-gray-300 mt-1">Let OHC's AI write engaging social media posts to drive traffic to your storefront.</p>
            </div>
          </div>
          <Link
            href="/promoter"
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-medium transition-colors"
          >
            Create Posts
          </Link>
        </div>
      </div>
    </div>
  );
}
