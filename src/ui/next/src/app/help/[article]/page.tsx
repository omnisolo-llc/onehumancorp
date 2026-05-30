"use client";

import React from 'react';
import Link from 'next/link';
import { useParams, useRouter } from 'next/navigation';

export default function HelpArticlePage() {
  const { article } = useParams();
  const router = useRouter();

  const articles: Record<string, { title: string, content: React.ReactNode }> = {
    "getting-started": {
      title: "Getting Started",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">Welcome to OneHumanCorp! We're excited to help you launch your business. Here is how to get started in under 10 minutes.</p>

          <section className="bg-blue-50/50 p-6 rounded-2xl border border-blue-100">
            <h2 className="text-xl font-bold text-blue-900 mb-2 font-outfit">1. Describe Your Business</h2>
            <p className="text-gray-700">Go to the <strong>Builder</strong> page. Tell our AI about what you sell and who your customers are. Don't worry about being perfect—our AI will help you refine it!</p>
          </section>

          <section className="bg-purple-50/50 p-6 rounded-2xl border border-purple-100">
            <h2 className="text-xl font-bold text-purple-900 mb-2 font-outfit">2. Choose Your Vibe</h2>
            <p className="text-gray-700">Select a style that matches your brand: Professional, Friendly, Energetic, or Minimalist. This tells our AI how to design your storefront.</p>
          </section>

          <section className="bg-green-50/50 p-6 rounded-2xl border border-green-100">
            <h2 className="text-xl font-bold text-green-900 mb-2 font-outfit">3. Launch Your Store</h2>
            <p className="text-gray-700">Once you're happy with the draft, click <strong>Launch Store</strong>. Your business is now live on the internet for everyone to see!</p>
          </section>
        </div>
      )
    },
    "my-store": {
      title: "Managing Your Store",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">Your store is the heart of your business. Here is how to keep it updated and looking fresh.</p>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <h3 className="font-bold text-gray-900 mb-2">Adding Products</h3>
              <p className="text-sm text-gray-600">Click "Add Item" on your dashboard. You can add photos, prices, and even set how many items you have in stock.</p>
            </div>
            <div className="p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <h3 className="font-bold text-gray-900 mb-2">Changing Design</h3>
              <p className="text-sm text-gray-600">Want a new look? Use the "Change Vibe" button in the Builder to instantly update your store's colors and style.</p>
            </div>
          </div>

          <section className="mt-8">
            <h2 className="text-xl font-bold text-gray-900 mb-4 font-outfit">Inventory Tracking</h2>
            <p className="text-gray-700 leading-relaxed">Our AI helpers track your stock levels automatically. When an item is sold out, we'll hide it from your store and send you a notification so you can restock.</p>
          </section>
        </div>
      )
    },
    "payments": {
      title: "Getting Paid",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">We make it simple and secure to accept payments from customers all over the world.</p>

          <div className="bg-red-50 p-6 rounded-2xl border border-red-100 flex items-start gap-4">
            <div className="text-2xl">💳</div>
            <div>
              <h3 className="font-bold text-red-900 mb-1">Set Up Stripe</h3>
              <p className="text-sm text-red-800 leading-relaxed">To accept your first payment, you must connect a Stripe account. Click "Complete Stripe Setup" on your dashboard. It only takes 2 minutes!</p>
            </div>
          </div>

          <div className="space-y-4">
            <h3 className="text-lg font-bold text-gray-900 font-outfit">What we support:</h3>
            <ul className="grid grid-cols-2 gap-3">
              <li className="flex items-center gap-2 text-gray-700 bg-gray-50 p-3 rounded-xl border border-gray-100">
                <span className="text-green-500 font-bold">✓</span> Credit & Debit Cards
              </li>
              <li className="flex items-center gap-2 text-gray-700 bg-gray-50 p-3 rounded-xl border border-gray-100">
                <span className="text-green-500 font-bold">✓</span> Apple Pay & Google Pay
              </li>
              <li className="flex items-center gap-2 text-gray-700 bg-gray-50 p-3 rounded-xl border border-gray-100">
                <span className="text-green-500 font-bold">✓</span> Bank Transfers
              </li>
              <li className="flex items-center gap-2 text-gray-700 bg-gray-50 p-3 rounded-xl border border-gray-100">
                <span className="text-green-500 font-bold">✓</span> In-person Tap to Pay
              </li>
            </ul>
          </div>

          <p className="text-sm text-gray-500 italic mt-4">Note: Payouts are usually deposited into your bank account within 2 business days.</p>
        </div>
      )
    },
    "ai-agents": {
      title: "Your AI Team",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">You're not alone! OHC gives you a team of AI agents that work for you 24/7.</p>

          <div className="space-y-4">
            <div className="flex items-center gap-4 p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <div className="text-3xl">🤝</div>
              <div>
                <h4 className="font-bold text-gray-900">The Ambassador</h4>
                <p className="text-sm text-gray-600 leading-relaxed">Handles customer questions and drafts replies so you can sleep while your customers get support.</p>
              </div>
            </div>
            <div className="flex items-center gap-4 p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <div className="text-3xl">⚙️</div>
              <div>
                <h4 className="font-bold text-gray-900">The Manager</h4>
                <p className="text-sm text-gray-600 leading-relaxed">Tracks your orders and inventory. It alerts you when you need to ship something or restock.</p>
              </div>
            </div>
            <div className="flex items-center gap-4 p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <div className="text-3xl">📣</div>
              <div>
                <h4 className="font-bold text-gray-900">The Promoter</h4>
                <p className="text-sm text-gray-600 leading-relaxed">Designs your website and auto-posts updates to social media to help you find new customers.</p>
              </div>
            </div>
          </div>
        </div>
      )
    },
    "marketing": {
      title: "Finding Customers",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">Learn how to get the word out and bring people to your new storefront.</p>

          <div className="bg-indigo-600 text-white p-8 rounded-3xl shadow-xl relative overflow-hidden">
            <div className="relative z-10">
              <h3 className="text-2xl font-bold mb-2 font-outfit">The Referral Loop</h3>
              <p className="opacity-90 mb-6 leading-relaxed">Our most powerful feature. Give your friends a discount link. When they sign up, you earn credits for premium tools.</p>
              <button onClick={() => router.push('/dashboard')} className="bg-white text-indigo-600 px-6 py-3 rounded-xl font-bold shadow-md hover:scale-105 transition-all">Get My Link</button>
            </div>
            <div className="absolute -bottom-10 -right-10 text-9xl opacity-20 rotate-12">🎁</div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-8">
            <div className="p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <h4 className="font-bold text-gray-900 mb-2">Social Sharing</h4>
              <p className="text-sm text-gray-600">Use our "Social Cards" to share beautiful, pre-designed posts on Instagram or Twitter with one tap.</p>
            </div>
            <div className="p-5 bg-white border border-gray-100 rounded-2xl shadow-sm">
              <h4 className="font-bold text-gray-900 mb-2">Google Search</h4>
              <p className="text-sm text-gray-600">Our AI automatically optimizes your site so people can find you when they search for businesses like yours.</p>
            </div>
          </div>
        </div>
      )
    },
    "account-billing": {
      title: "Account & Billing",
      content: (
        <div className="space-y-6">
          <p className="text-lg text-gray-600">Everything you need to know about your plan and how to manage your team.</p>

          <div className="bg-white border border-gray-200 rounded-2xl overflow-hidden">
            <div className="p-6 border-b border-gray-100">
              <h3 className="font-bold text-gray-900 mb-1">Your Plan</h3>
              <p className="text-sm text-gray-500">You are currently on the Free plan.</p>
            </div>
            <div className="p-6 bg-gray-50 flex items-center justify-between">
              <div>
                <p className="text-sm font-bold text-gray-900">OHC Pro</p>
                <p className="text-xs text-gray-500">Unlock custom domains and unlimited agents</p>
              </div>
              <button onClick={() => router.push('/pricing')} className="bg-gray-900 text-white px-4 py-2 rounded-lg text-sm font-bold shadow-md hover:bg-black transition-all">Upgrade</button>
            </div>
          </div>

          <div className="space-y-4">
             <h3 className="font-bold text-gray-900 font-outfit">Common Questions</h3>
             <details className="group bg-white border border-gray-100 rounded-2xl p-4 cursor-pointer">
                <summary className="flex justify-between items-center font-bold text-sm text-gray-800">
                  How do I cancel my plan?
                  <span className="transition-transform group-open:rotate-180">↓</span>
                </summary>
                <p className="text-xs text-gray-600 mt-3 pt-3 border-t border-gray-50">You can cancel any time from the 'Plan' page. You will keep your features until the end of your billing month.</p>
             </details>
             <details className="group bg-white border border-gray-100 rounded-2xl p-4 cursor-pointer">
                <summary className="flex justify-between items-center font-bold text-sm text-gray-800">
                  Can I invite my partner?
                  <span className="transition-transform group-open:rotate-180">↓</span>
                </summary>
                <p className="text-xs text-gray-600 mt-3 pt-3 border-t border-gray-50">Yes! Go to Team settings to invite others. Pro plans allow for unlimited team members.</p>
             </details>
          </div>
        </div>
      )
    }
  };

  const articleData = article && typeof article === 'string' && articles[article]
    ? articles[article]
    : { title: "Article Not Found", content: <div className="text-center py-12"><p className="text-lg text-gray-500">We couldn't find the article you're looking for.</p><button onClick={() => router.push('/help')} className="mt-4 text-blue-600 font-bold hover:underline">Return to Help Center</button></div> };

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <button
          onClick={() => router.push('/help')}
          className="mb-8 flex items-center gap-2 text-[#86868B] font-bold hover:text-[#1D1D1F] transition-colors group"
        >
          <svg className="w-5 h-5 transform group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
          </svg>
          Back to Help Center
        </button>

        <div className="bg-white/80 backdrop-blur-[30px] saturate-[180%] p-8 md:p-12 rounded-[40px] shadow-xl border border-white/50">
          <h1 className="text-4xl font-extrabold text-[#1D1D1F] mb-8 font-outfit tracking-tight">{articleData.title}</h1>
          <div className="prose prose-blue max-w-none text-gray-700 font-medium leading-relaxed">
            {articleData.content}
          </div>

          <div className="mt-12 pt-12 border-t border-gray-100 flex flex-col sm:flex-row items-center justify-between gap-6 text-center sm:text-left">
            <div>
              <p className="text-sm font-bold text-[#1D1D1F] mb-1">Was this article helpful?</p>
              <div className="flex gap-4 justify-center sm:justify-start mt-2">
                 <button className="px-6 py-2 bg-gray-50 hover:bg-green-50 hover:text-green-600 rounded-xl font-bold transition-all border border-gray-100">Yes</button>
                 <button className="px-6 py-2 bg-gray-50 hover:bg-red-50 hover:text-red-600 rounded-xl font-bold transition-all border border-gray-100">No</button>
              </div>
            </div>
            <p className="text-xs text-[#86868B] font-medium">Still need help? <Link href="/help" className="text-blue-600 hover:underline">Ask our AI Agent</Link></p>
          </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
