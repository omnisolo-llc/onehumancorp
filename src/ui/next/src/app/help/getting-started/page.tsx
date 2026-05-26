"use client";

import React from 'react';
import Link from 'next/link';

export default function GettingStartedHelp() {
  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter p-4">
      <div className="w-full max-w-[600px] bg-white shadow-xl rounded-2xl overflow-hidden flex flex-col">
        <header className="px-6 py-5 border-b border-gray-100 flex items-center gap-4">
          <Link href="/" className="text-blue-600 hover:bg-blue-50 p-2 rounded-full transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold text-gray-900">Getting Started</h1>
        </header>
        <div className="p-6 space-y-6 flex-1 overflow-y-auto">
          <p className="text-gray-600 text-lg">Welcome to OneHumanCorp! Let's get your store set up and ready to accept your first payment.</p>

          <div className="space-y-4">
            <h2 className="text-xl font-bold text-gray-800">1. Set up your store</h2>
            <p className="text-gray-600">Head over to the <Link href="/storefront-builder" className="text-blue-600 font-medium hover:underline">Storefront Builder</Link>. Our AI will help you design your store quickly. Just describe what you sell, and we'll do the rest!</p>
          </div>

          <div className="space-y-4">
            <h2 className="text-xl font-bold text-gray-800">2. Accept your first payment</h2>
            <p className="text-gray-600">Connect your bank account securely. Click on the Setup tab to get started with Stripe. It's safe, fast, and takes only a few minutes.</p>
          </div>

          <div className="space-y-4">
            <h2 className="text-xl font-bold text-gray-800">3. Activate your AI Support Agent</h2>
            <p className="text-gray-600">Want help answering customer questions? Go to the <Link href="/agents" className="text-blue-600 font-medium hover:underline">AI Agents</Link> page and hire your first helper. They work 24/7 so you don't have to!</p>
          </div>
        </div>
      </div>
    </div>
  );
}
