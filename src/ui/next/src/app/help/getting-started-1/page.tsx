"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

export default function GettingStartedArticle() {
  const router = useRouter();

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="app-card backdrop-blur-[30px] saturate-[210%] bg-white dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)]">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-6 tracking-tight">
            Getting Started with Your Store
          </h1>
          <p className="text-gray-700 text-lg leading-relaxed mb-8">
            Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.
          </p>

          <div className="space-y-6 mb-10">
            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">1. Set up your basic info</h3>
              <p className="text-blue-800/80">Add your store name and a short bio so customers know who you are.</p>
            </div>

            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">2. Add your first product</h3>
              <p className="text-blue-800/80">Upload a photo, set a price, and describe what you're selling.</p>
            </div>

            <div className="p-6 bg-blue-50/50 rounded-2xl border border-blue-100/50">
              <h3 className="font-bold font-outfit text-blue-900 text-xl mb-2">3. Start accepting payments</h3>
              <p className="text-blue-800/80">Connect your bank securely to get paid directly when customers buy.</p>
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
