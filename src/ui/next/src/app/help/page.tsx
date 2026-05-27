"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function HelpCenterIndex() {
  const [searchQuery, setSearchQuery] = useState("");

  const topics = [
    { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started" },
    { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" },
    { title: "Getting Paid", desc: "Set up how you get paid, view deposits, and handle simple taxes.", link: "/help/payments" },
    { title: "Your AI Helpers", desc: "Learn how to hire AI helpers and give them tasks to do.", link: "/help/ai-agents" },
    { title: "Finding Customers", desc: "Send emails to customers and grow your business easily.", link: "/help/marketing" },
    { title: "Account & Billing", desc: "View your bills, manage your plan, and invite team members.", link: "/help/account-billing" }
  ];

  const filteredTopics = topics.filter(t =>
    t.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    t.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="space-y-8">
      <div className="text-center">
        <h1 className="text-3xl sm:text-4xl font-extrabold text-gray-900 font-outfit mb-4">How can we help?</h1>
        <p className="text-lg text-gray-600 max-w-2xl mx-auto">Browse topics below or use the AI Help widget in the bottom right for instant answers.</p>
      </div>

      <div className="max-w-xl mx-auto mt-6 relative">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg className="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
            <path fillRule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clipRule="evenodd" />
          </svg>
        </div>
        <input
          type="text"
          className="block w-full pl-10 pr-3 py-3 border border-gray-300 rounded-xl leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 sm:text-sm shadow-sm"
          placeholder="Search for help..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
        {filteredTopics.length > 0 ? filteredTopics.map((topic, idx) => (
          <Link href={topic.link} key={idx} className="block group">
            <div className="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm hover:shadow-md hover:border-blue-200 transition-all h-full">
              <h3 className="text-xl font-bold text-gray-900 mb-2 font-outfit group-hover:text-blue-600 transition-colors">{topic.title}</h3>
              <p className="text-gray-600">{topic.desc}</p>
            </div>
          </Link>
        )) : (
          <div className="col-span-2 text-center py-8">
            <p className="text-gray-500">No help articles found for "{searchQuery}". Try a different term or ask our AI!</p>
          </div>
        )}
      </div>
    </div>
  );
}
