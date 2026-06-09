"use client";

import React from 'react';
import Link from 'next/link';

export function PromoterCard() {
  return (
    <div className="glassmorphism rounded-[24px] shadow-sm border border-gray-100 overflow-hidden mt-6 mb-6">
      <div className="bg-gradient-to-r from-indigo-500 to-purple-500 p-1"></div>
      <div className="p-6">
        <div className="flex justify-between items-start">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-indigo-50 flex items-center justify-center text-2xl">
              📣
            </div>
            <div>
              <h3 className="text-lg font-bold text-gray-900 font-outfit">The Promoter Agent</h3>
              <p className="text-sm text-gray-600 mt-1">Let OHC's AI write engaging social media posts to drive traffic to your storefront.</p>
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
