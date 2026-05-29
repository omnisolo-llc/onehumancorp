"use client";

import React from 'react';
import { useParams } from 'next/navigation';

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
          <p>Manage your products and see what your store looks like.</p>
          <h2 className="text-xl font-bold mt-4">1. Add Products</h2>
          <p>Go to the Products page. Click "Add Product" to upload photos, set a price, and write a description.</p>
          <h2 className="text-xl font-bold mt-4">2. Check Stock</h2>
          <p>See how many items you have left so you never run out unexpectedly.</p>
          <h2 className="text-xl font-bold mt-4">3. Change Design</h2>
          <p>Use the Store Builder to easily change colors, fonts, and images to fit your brand.</p>
        </>
      )
    },
    "payments": {
      title: "Getting Paid",
      content: (
        <>
          <p>Learn how to receive money from your customers.</p>
          <h2 className="text-xl font-bold mt-4">1. Connect Your Bank</h2>
          <p>Go to Setup and link your bank account securely using Stripe. This is how you get your money.</p>
          <h2 className="text-xl font-bold mt-4">2. Track Deposits</h2>
          <p>Check the Dashboard to see when your sales money will arrive in your bank account.</p>
          <h2 className="text-xl font-bold mt-4">3. Taxes Made Simple</h2>
          <p>Our system helps calculate standard taxes automatically during checkout, so you do not have to worry.</p>
        </>
      )
    },
    "ai-agents": {
      title: "Your AI Helpers",
      content: (
        <>
          <p>AI Agents are like virtual employees who help you run your business.</p>
          <h2 className="text-xl font-bold mt-4">1. Hire a Helper</h2>
          <p>Visit the Agents page to choose a helper for tasks like writing emails or sorting products.</p>
          <h2 className="text-xl font-bold mt-4">2. Give Them Tasks</h2>
          <p>Simply type what you need done in plain English. Your helper will get to work right away.</p>
          <h2 className="text-xl font-bold mt-4">3. Check Their Work</h2>
          <p>You can always see what your helpers are doing and approve their work before it goes live.</p>
        </>
      )
    },
    "marketing": {
      title: "Finding Customers",
      content: (
        <>
          <p>Grow your business by reaching more people easily.</p>
          <h2 className="text-xl font-bold mt-4">1. Send Emails</h2>
          <p>Use our tools to write friendly emails to your customers about sales or new items.</p>
          <h2 className="text-xl font-bold mt-4">2. Share on Social Media</h2>
          <p>Create special links and images to post on your social accounts to bring people to your store.</p>
          <h2 className="text-xl font-bold mt-4">3. Offer Discounts</h2>
          <p>Create discount codes to give your customers a reason to buy today.</p>
        </>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <>
          <p>Keep your account details up to date and manage your plan.</p>
          <h2 className="text-xl font-bold mt-4">1. View Bills</h2>
          <p>See your past receipts and current charges on the Billing page.</p>
          <h2 className="text-xl font-bold mt-4">2. Manage Your Plan</h2>
          <p>Upgrade or change your subscription plan anytime to fit your growing needs.</p>
          <h2 className="text-xl font-bold mt-4">3. Invite Your Team</h2>
          <p>Add your staff or partners to the account so they can help manage the store.</p>
        </>
      )
    }
  };

  const articleData = article && typeof article === 'string' && articles[article] ? articles[article] : { title: "Article Not Found", content: <p>We couldn't find the article you're looking for.</p> };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white p-8 rounded-xl shadow-sm border border-gray-100">
        <h1 className="text-3xl font-bold text-gray-900 mb-6">{articleData.title}</h1>
        <div className="prose prose-blue max-w-none text-gray-700">
          {articleData.content}
        </div>
      </div>
    </div>
  );
}
