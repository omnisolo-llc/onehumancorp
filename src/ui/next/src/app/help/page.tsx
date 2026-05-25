"use client";

import React from 'react';
import Link from 'next/link';
import { WithTooltip } from '../../components/TooltipRegistry';
import { useState } from 'react';

const topics = [
  {
    title: 'Getting Started',
    slug: 'getting-started',
    desc: 'Learn how to easily set up your store and accept your first payment.',
    icon: '🚀'
  },
  {
    title: 'My Store',
    slug: 'my-store',
    desc: "Add products, track what's in stock, and change how your store looks.",
    icon: '🏪'
  },
  {
    title: 'Getting Paid',
    slug: 'payments',
    desc: 'Set up how you get paid, view deposits, and handle simple taxes.',
    icon: '💳'
  },
  {
    title: 'Your AI Helpers',
    slug: 'ai-agents',
    desc: 'Learn how to hire AI helpers and give them tasks to do.',
    icon: '🤖'
  },
  {
    title: 'Finding Customers',
    slug: 'marketing',
    desc: 'Send emails to customers and grow your business easily.',
    icon: '📢'
  },
  {
    title: 'Account & Billing',
    slug: 'account-billing',
    desc: 'View your bills, manage your plan, and invite team members.',
    icon: '⚙️'
  }
];

export default function HelpCenterPage() {
  const [searchQuery, setSearchQuery] = useState('');

  const filteredTopics = topics.filter(t =>
    t.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    t.desc.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold text-gray-900 mb-4 font-outfit">Help Center</h1>
          <p className="text-lg text-gray-600 mb-6">How can we help you grow your business today?</p>

          <div className="max-w-xl mx-auto relative">
            <input
              type="text"
              placeholder="Search for articles..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full px-6 py-4 rounded-full border-2 border-gray-200 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 outline-none text-lg shadow-sm transition-all"
            />
            <div className="absolute right-4 top-4 text-gray-400">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {filteredTopics.length > 0 ? filteredTopics.map((topic) => (
            <Link key={topic.slug} href={`/help/${topic.slug}`}>
              <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 hover:border-blue-300 hover:shadow-md transition-all cursor-pointer h-full flex items-start gap-4">
                <div className="text-3xl">{topic.icon}</div>
                <div>
                  <h2 className="text-xl font-bold text-gray-900 mb-2 font-outfit">{topic.title}</h2>
                  <p className="text-gray-600 text-sm leading-relaxed">{topic.desc}</p>
                </div>
              </div>
            </Link>
          )) : (
            <div className="col-span-1 md:col-span-2 text-center py-12 bg-white rounded-2xl shadow-sm border border-gray-100">
              <p className="text-gray-500 text-lg">No results found for "{searchQuery}". Try asking our AI Support Agent!</p>
            </div>
          )}
        </div>

        <div className="mt-12 bg-blue-50 rounded-2xl p-8 border border-blue-100 text-center">
          <h3 className="text-2xl font-bold text-blue-900 mb-2 font-outfit">Still need help?</h3>
          <p className="text-blue-800 mb-6">Our AI Support Agent is available 24/7 to answer your questions.</p>
          <WithTooltip id="help-chat-prompt" defaultText="Click the floating button in the bottom right corner to start a chat.">
              <span className="inline-block bg-blue-600 text-white font-bold py-3 px-6 rounded-xl shadow-sm hover:bg-blue-700 transition-colors">
                  Ask AI Support
              </span>
          </WithTooltip>
        </div>
      </div>
    </div>
  );
}