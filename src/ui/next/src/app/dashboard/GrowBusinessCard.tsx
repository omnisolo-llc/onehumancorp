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
              <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Deploy a zero-config edge-cached storefront for instant consumer discovery.</p>
            </div>
          </div>
          <Link
            id="review-storefront-btn"
            href="/edge-storefront-setup"
            className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors whitespace-nowrap"
          >
            Review Storefront
          </Link>
        </div>
      </div>
    </div>
  );
}
