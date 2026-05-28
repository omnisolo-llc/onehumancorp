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
          <p>Your store is where you manage everything your customers see.</p>
          <h2 className="text-xl font-bold mt-4">Adding Products</h2>
          <p>You can add new items to sell anytime. Just click "Add Product" and enter a name, price, and picture.</p>
          <h2 className="text-xl font-bold mt-4">Changing How It Looks</h2>
          <p>Want a different color or logo? Use the "Theme" button to update your store's style in seconds.</p>
        </>
      )
    },
    "payments": {
      title: "Payments",
      content: (
        <>
          <p>Getting paid is the best part of running a business.</p>
          <h2 className="text-xl font-bold mt-4">How to Get Paid</h2>
          <p>We use Stripe to safely send money to your bank account. Go to Settings and click "Connect Bank" to set it up.</p>
          <h2 className="text-xl font-bold mt-4">When Will I See My Money?</h2>
          <p>Money usually shows up in your bank account in 2 to 3 days after a customer buys something.</p>
        </>
      )
    },
    "ai-agents": {
      title: "AI Agents",
      content: (
        <>
          <p>Think of AI Agents as your digital helpers. They work for you 24/7.</p>
          <h2 className="text-xl font-bold mt-4">What Can They Do?</h2>
          <p>They can answer customer questions, write emails, and build web pages for you.</p>
          <h2 className="text-xl font-bold mt-4">How to Use Them</h2>
          <p>Go to the Agents page to see who is working for you. You can give them new tasks by typing what you need them to do.</p>
        </>
      )
    },
    "marketing": {
      title: "Marketing",
      content: (
        <>
          <p>Marketing helps new people find your business.</p>
          <h2 className="text-xl font-bold mt-4">Sending Emails</h2>
          <p>You can send messages to people who have bought from you before. Tell them about new items or sales.</p>
          <h2 className="text-xl font-bold mt-4">Getting Help</h2>
          <p>If you don't know what to write, ask your AI Marketing Agent to write an email for you!</p>
        </>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <>
          <p>Manage your personal details and plan here.</p>
          <h2 className="text-xl font-bold mt-4">Your Bill</h2>
          <p>You can see how much your plan costs and view past receipts anytime.</p>
          <h2 className="text-xl font-bold mt-4">Inviting Helpers</h2>
          <p>Have a business partner or employee? You can invite them to help run the store by adding their email.</p>
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
