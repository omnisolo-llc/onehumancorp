import React from 'react';
import Link from 'next/link';
import { notFound } from 'next/navigation';

const ARTICLES: Record<string, { title: string, content: React.ReactNode }> = {
  "getting-started": {
    title: "Getting Started",
    content: (
      <>
        <p className="mb-4">Welcome to OneHumanCorp! Setting up your store is designed to be as easy as possible.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">1. Describe Your Business</h3>
        <p className="mb-4">Just tell our AI what you sell, and it will generate a complete storefront for you.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">2. Connect Payments</h3>
        <p className="mb-4">Go to the Setup page and connect your bank account using Stripe so you can start getting paid immediately.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">3. Publish</h3>
        <p className="mb-4">Click "Launch" to make your store live on the internet.</p>
      </>
    )
  },
  "my-store": {
    title: "My Store",
    content: (
      <>
        <p className="mb-4">Your store is your online home. Here is how to manage it.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Adding Products</h3>
        <p className="mb-4">Navigate to the Dashboard and click "Add Product". You can upload photos, set a price, and write a simple description.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Changing the Look</h3>
        <p className="mb-4">Want a new color or logo? Use the Storefront Builder to ask your AI Architect to give your store a fresh new look.</p>
      </>
    )
  },
  "payments": {
    title: "Getting Paid",
    content: (
      <>
        <p className="mb-4">Getting paid is easy and secure with Stripe.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Setting Up</h3>
        <p className="mb-4">You only need to connect your bank account once. We handle all the security and compliance.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Deposits</h3>
        <p className="mb-4">Funds from your sales are typically deposited into your bank account within 2-3 business days.</p>
      </>
    )
  },
  "ai-agents": {
    title: "Your AI Helpers",
    content: (
      <>
        <p className="mb-4">You don't have to run your business alone. Your AI helpers are here 24/7.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Giving Tasks</h3>
        <p className="mb-4">Go to the Agents tab and type what you need done. E.g., "Write a marketing email for my new spring collection."</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Approvals</h3>
        <p className="mb-4">Your AI helpers will always ask for your approval before spending money or sending emails to your customers.</p>
      </>
    )
  },
  "marketing": {
    title: "Finding Customers",
    content: (
      <>
        <p className="mb-4">Growing your business is about finding the right people.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Email Campaigns</h3>
        <p className="mb-4">Ask your AI Marketer to write an email campaign. They will design it and send it to your subscriber list.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Promotions</h3>
        <p className="mb-4">You can easily create discount codes like "SPRING20" to share on your social media.</p>
      </>
    )
  },
  "account-billing": {
    title: "Account & Billing",
    content: (
      <>
        <p className="mb-4">Manage your subscription and team members here.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Your Plan</h3>
        <p className="mb-4">You can upgrade or downgrade your plan at any time based on your business needs.</p>
        <h3 className="text-xl font-bold mt-6 mb-2">Inviting Your Team</h3>
        <p className="mb-4">Have an accountant or a business partner? You can invite them to view your dashboard from the Team page.</p>
      </>
    )
  }
};

export default function HelpArticlePage({ params }: { params: { slug: string } }) {
  const article = ARTICLES[params.slug];

  if (!article) {
    notFound();
  }

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-2xl w-full bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <Link href="/" className="text-blue-600 hover:text-blue-800 font-medium text-sm mb-6 inline-flex items-center">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to App
        </Link>
        <h1 className="text-3xl font-extrabold text-gray-900 mb-6 font-outfit">{article.title}</h1>
        <div className="prose prose-blue max-w-none text-gray-600 leading-relaxed">
          {article.content}
        </div>
        <div className="mt-12 pt-6 border-t border-gray-100 text-center">
           <p className="text-sm text-gray-500 mb-4">Did this answer your question?</p>
           <div className="flex justify-center gap-4">
             <button className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">Yes, it did</button>
             <button className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg text-sm font-medium transition-colors">No, I need more help</button>
           </div>
        </div>
      </div>
    </div>
  );
}
