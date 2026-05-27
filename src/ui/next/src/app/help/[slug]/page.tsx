"use client";

import React from 'react';
import { useParams, useRouter } from 'next/navigation';

export default function HelpArticlePage() {
  const params = useParams();
  const router = useRouter();
  const slug = params.slug as string;

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center py-10 px-4 sm:px-6 lg:px-8">
      <div className="max-w-3xl w-full bg-white rounded-2xl shadow-sm border border-gray-100 p-8">
        <button
          onClick={() => router.back()}
          className="text-blue-600 font-medium hover:underline flex items-center gap-1 mb-6"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back
        </button>
        <h1 className="text-3xl font-bold text-gray-900 mb-6 font-outfit capitalize">
          {slug.replace(/-/g, ' ')}
        </h1>
        <div className="prose prose-blue max-w-none text-gray-700 font-inter leading-relaxed">
          {slug === 'getting-started' && (
            <>
              <p>Welcome to OneHumanCorp! Here is how you can set up your store and accept your first payment.</p>
              <h3>1. Fill out your Business Description</h3>
              <p>Navigate to the storefront builder and tell our AI about your business. It will generate a complete storefront for you.</p>
              <h3>2. Connect Stripe</h3>
              <p>From your dashboard, click on 'Complete Stripe Setup' in the Action Required section to link your bank account.</p>
            </>
          )}
          {slug === 'my-store' && (
            <>
              <p>Manage your products, track inventory, and customize your store's appearance.</p>
              <h3>Adding Products</h3>
              <p>Use the Builder interface to add new products, set prices, and write descriptions.</p>
            </>
          )}
          {slug === 'payments' && (
            <>
              <p>Handling payments is easy with OHC.</p>
              <h3>View Deposits</h3>
              <p>Check your dashboard to see today's sales and pending orders.</p>
            </>
          )}
          {slug === 'ai-agents' && (
            <>
              <p>Your AI helpers are ready to assist you.</p>
              <h3>Hiring Agents</h3>
              <p>Go to the Agents tab to see your AI team and assign tasks.</p>
            </>
          )}
          {slug === 'marketing' && (
            <>
              <p>Grow your business with AI marketing tools.</p>
              <h3>Sending Emails</h3>
              <p>Use the Review Campaigns feature to send automated review requests to recent buyers.</p>
            </>
          )}
          {slug === 'account-billing' && (
            <>
              <p>Manage your OHC subscription.</p>
              <h3>Upgrade to Pro</h3>
              <p>Unlock AI Business Insights and advanced analytics by upgrading to the Pro plan.</p>
            </>
          )}
          {slug === 'analytics' && (
            <>
              <p>Understand your business performance.</p>
              <h3>Dashboard Metrics</h3>
              <p>Your dashboard provides real-time data on sales, active customers, and pending orders.</p>
            </>
          )}
          {!['getting-started', 'my-store', 'payments', 'ai-agents', 'marketing', 'account-billing', 'analytics'].includes(slug) && (
            <p>Welcome to the <strong>{slug.replace(/-/g, ' ')}</strong> article! Detailed help content is coming soon.</p>
          )}
          <div className="mt-8 pt-8 border-t border-gray-100">
            <p className="font-semibold mb-2">Need more help?</p>
            <p>Click the <strong>Ask anything</strong> button in the bottom right to chat with our specialized Help Agent.</p>
          </div>
        </div>
      </div>
    </div>
  );
}
