"use client";

import React, { useState } from "react";
import Link from "next/link";

export default function HelpCenterPage() {
  const [search, setSearch] = useState("");

  const articles = [
    { category: "Getting Started", title: "Set up your store in 5 minutes", description: "Follow our simple guide to add your first product and go live." },
    { category: "My Store", title: "How to add products", description: "Learn how to list new items, add photos, and set prices." },
    { category: "Payments & Billing", title: "How to accept Apple Pay", description: "Enable Apple Pay with one click in your payment settings." },
    { category: "AI Helpers", title: "What can the Customer Success Helper do?", description: "Your helper can reply to customer emails and Instagram DMs automatically." },
    { category: "Marketing", title: "How to run a promotion", description: "Learn how to create discount codes and share them on social media." },
    { category: "Account & Billing", title: "How to change your subscription", description: "Find out how to upgrade or downgrade your plan and view past invoices." }
  ];

  const filtered = articles.filter(a =>
    a.title.toLowerCase().includes(search.toLowerCase()) ||
    a.description.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="min-h-screen bg-[#f8fafc] p-6 font-inter">
      <div className="max-w-2xl mx-auto">
        <div className="mb-8">
          <Link href="/dashboard" className="text-sm text-blue-600 hover:underline font-semibold mb-4 inline-block">
            ← Back to Dashboard
          </Link>
          <h1 className="text-3xl font-bold text-[#0f172a] mb-4">How can we help?</h1>
          <input
            type="text"
            placeholder="Search help articles..."
            className="w-full p-4 rounded-xl border border-gray-200 shadow-sm focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-200"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <div className="space-y-4">
          {filtered.map((article, idx) => (
            <div key={idx} className="bg-white p-6 rounded-xl border border-gray-200 shadow-sm hover:border-blue-300 transition-colors cursor-pointer">
              <span className="text-xs font-bold text-blue-600 uppercase tracking-wider mb-2 block">{article.category}</span>
              <h2 className="text-xl font-bold text-[#0f172a] mb-2">{article.title}</h2>
              <p className="text-[#64748b] text-sm leading-relaxed">{article.description}</p>
            </div>
          ))}
          {filtered.length === 0 && (
             <div className="text-center py-12 text-gray-500">
                No articles found matching "{search}".
             </div>
          )}
        </div>
      </div>
    </div>
  );
}
