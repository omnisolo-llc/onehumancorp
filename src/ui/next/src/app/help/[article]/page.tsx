"use client";

import React from 'react';
import { useParams } from 'next/navigation';
import Link from 'next/link';

export default function HelpArticlePage() {
  const { article } = useParams();

  const articles: Record<string, { title: string, content: React.ReactNode }> = {
    "getting-started": {
      title: "Getting Started",
      content: (
        <>
          <p>Welcome to OneHumanCorp! Let's get your store set up.</p>
          <h2 className="text-xl font-bold mt-4">1. Describe Your Business</h2>
          <p>Go to the Builder page and type a short description of what you sell. Our AI will do the rest!</p>
          <h2 className="text-xl font-bold mt-4">2. Launch Your Store</h2>
          <p>Click the "Launch Store" button to make it live for the world to see.</p>
        </>
      )
    },
    "my-store": {
      title: "My Store",
      content: (
        <>
          <p>Manage your products, track inventory, and customize your storefront.</p>
          <h2 className="text-xl font-bold mt-4">Adding Products</h2>
          <p>Navigate to the Products tab to add new items. You can set prices, variants, and upload photos.</p>
          <h2 className="text-xl font-bold mt-4">Inventory Tracking</h2>
          <p>Your AI Operations Agent automatically tracks what is in stock based on your sales.</p>
        </>
      )
    },
    "payments": {
      title: "Getting Paid",
      content: (
        <>
          <p>Set up how you receive money from your customers.</p>
          <h2 className="text-xl font-bold mt-4">Stripe Integration</h2>
          <p>We use Stripe to securely process payments. Go to the Setup page to connect your bank account.</p>
          <h2 className="text-xl font-bold mt-4">Tap to Pay</h2>
          <p>If you sell in person, use our mobile app to accept Tap to Pay directly on your phone.</p>
        </>
      )
    },
    "ai-agents": {
      title: "Your AI Helpers",
      content: (
        <>
          <p>Learn how to use your AI team to do the heavy lifting.</p>
          <h2 className="text-xl font-bold mt-4">The Promoter</h2>
          <p>Handles your marketing and SEO. Ask them to write a social media post!</p>
          <h2 className="text-xl font-bold mt-4">The Ambassador</h2>
          <p>Replies to your customers and asks for reviews automatically.</p>
        </>
      )
    },
    "marketing": {
      title: "Finding Customers",
      content: (
        <>
          <p>Grow your audience and drive sales.</p>
          <h2 className="text-xl font-bold mt-4">Email Campaigns</h2>
          <p>Use the Marketing tab to send beautiful newsletters to your subscribers.</p>
          <h2 className="text-xl font-bold mt-4">SEO Optimization</h2>
          <p>Your website is automatically optimized for Google by our AI agents.</p>
        </>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <>
          <p>Manage your subscription and team members.</p>
          <h2 className="text-xl font-bold mt-4">Your Plan</h2>
          <p>View your current subscription tier and upgrade if you need more features.</p>
          <h2 className="text-xl font-bold mt-4">Team Invites</h2>
          <p>Invite employees to manage your store with you. You can set specific permissions for each member.</p>
        </>
      )
    }
  };

  const articleData = article && typeof article === 'string' && articles[article] ? articles[article] : { title: "Article Not Found", content: <p>We couldn't find the article you're looking for.</p> };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <Link href="/help" className="inline-flex items-center text-blue-600 hover:text-blue-800 mb-6 font-semibold">
          <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          Back to Help Center
        </Link>
        <div className="bg-white p-8 rounded-xl shadow-sm border border-gray-100">
          <h1 className="text-3xl font-bold text-gray-900 mb-6 font-outfit">{articleData.title}</h1>
          <div className="prose prose-blue max-w-none text-gray-700">
            {articleData.content}
          </div>
        </div>
      </div>
    </div>
  );
}
