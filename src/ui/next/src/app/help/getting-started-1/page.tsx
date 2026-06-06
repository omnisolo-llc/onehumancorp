"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

export default function GettingStartedArticle() {
  const router = useRouter();

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="bg-white/60 backdrop-blur-[20px] saturate-200 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/50">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-6 tracking-tight">
            Getting Started with Your Store
          </h1>
          <p className="text-gray-700 text-lg leading-relaxed mb-8">
            Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.
          </p>

          <div className="space-y-8 mb-10">
            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">1. Tell us about your business</h3>
              <p className="text-blue-800/80">Start by telling us what you sell and who your customers are. Maya, our home baker persona, might say: "I sell custom vegan cakes for birthday parties in Brooklyn."</p>
            </div>

            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">2. Let AI build your store</h3>
              <p className="text-blue-800/80">Once you describe your business, click "Generate". Our AI agents will pick a beautiful design, write your first product descriptions, and organize your layout.</p>
            </div>

            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">3. Launch to the world</h3>
              <p className="text-blue-800/80">When you're happy with the preview, click "Launch". Your store is now live at your own unique URL, ready for Maya's first cake order!</p>
            </div>
          </div>

          <div className="pt-6 border-t border-gray-200/50">
            <button
              onClick={() => router.push('/help')}
              className="inline-flex items-center px-6 py-3 bg-white hover:bg-gray-50 text-gray-900 font-bold rounded-xl border border-gray-200 shadow-sm transition-all active:scale-95"
            >
              <svg className="w-5 h-5 mr-2 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              Back to Help Center
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
