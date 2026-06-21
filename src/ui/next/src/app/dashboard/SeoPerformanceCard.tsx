"use client";

import React from 'react';

export function SeoPerformanceCard() {
  return (
    <div className="glassmorphism bg-white rounded-[24px] shadow-sm border border-gray-100 overflow-hidden mt-6 mb-6">
      <div className="bg-gradient-to-r from-blue-500 to-teal-400 p-1"></div>
      <div className="p-6">
        <div className="flex justify-between items-start">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-blue-50 flex items-center justify-center text-2xl">
              ⚡
            </div>
            <div>
              <h3 className="text-lg font-bold text-gray-900 font-outfit">SEO &amp; Performance</h3>
              <div className="mt-2 space-y-1">
                <div className="flex items-center gap-2">
                  <span className="text-sm font-medium text-gray-600">Site Speed:</span>
                  <span className="text-sm font-bold text-green-600 bg-green-50 px-2 py-0.5 rounded-full">Lightning Fast</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-sm font-medium text-gray-600">SEO Status:</span>
                  <span className="text-sm font-bold text-blue-600 bg-blue-50 px-2 py-0.5 rounded-full">Optimized for Google</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
