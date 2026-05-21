"use client";

import React from "react";
import Link from "next/link";

export default function ChangelogPage() {
  return (
    <div className="min-h-screen bg-gray-50 font-inter p-6 md:p-12">
      <div className="max-w-3xl mx-auto">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">What's New in OHC</h1>
          <Link href="/dashboard" className="text-sm font-semibold text-blue-600 hover:underline">
            ← Back to Dashboard
          </Link>
        </div>

        <div className="space-y-12">
          {/* Release 1 */}
          <section className="bg-white p-6 md:p-8 rounded-2xl shadow-sm border border-gray-100">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">New AI Store Builder</h2>
              <span className="text-sm font-semibold text-blue-600 bg-blue-50 px-3 py-1 rounded-full">v0.4.8</span>
            </div>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              We've completely revamped how you create your storefront. You can now simply describe what your business does, and our AI agents will build a complete, ready-to-launch store for you. It writes your copy, picks images, and sets up your payment processing automatically.
            </p>
            <div className="w-full aspect-video bg-gray-100 rounded-xl mb-6 overflow-hidden flex items-center justify-center border border-gray-200">
              <span className="text-gray-400 text-sm font-semibold">Screenshot: Store Builder Interface</span>
            </div>
          </section>

          {/* Release 2 */}
          <section className="bg-white p-6 md:p-8 rounded-2xl shadow-sm border border-gray-100">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">Viral Storefront Referrals</h2>
              <span className="text-sm font-semibold text-blue-600 bg-blue-50 px-3 py-1 rounded-full">v0.4.7</span>
            </div>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Growing your business just got easier. You can now share a unique invite link with friends. When they launch their storefront, you both get a $50 credit toward premium tools. Track your rewards directly from the dashboard!
            </p>
            <div className="w-full aspect-video bg-gray-100 rounded-xl mb-6 overflow-hidden flex items-center justify-center border border-gray-200">
              <span className="text-gray-400 text-sm font-semibold">Screenshot: Referral Dashboard</span>
            </div>
          </section>

          {/* Release 3 */}
          <section className="bg-white p-6 md:p-8 rounded-2xl shadow-sm border border-gray-100">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-bold text-gray-900">In-App Help & Walkthroughs</h2>
              <span className="text-sm font-semibold text-blue-600 bg-blue-50 px-3 py-1 rounded-full">v0.4.6</span>
            </div>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Never get stuck again. We've added a comprehensive Help Center accessible from the bottom right of any screen. Take interactive tours, watch 60-second video tutorials, or ask our new AI Support Agent for immediate help with any feature.
            </p>
            <div className="w-full aspect-video bg-gray-100 rounded-xl mb-6 overflow-hidden flex items-center justify-center border border-gray-200">
              <span className="text-gray-400 text-sm font-semibold">Screenshot: Help Center Widget</span>
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}
