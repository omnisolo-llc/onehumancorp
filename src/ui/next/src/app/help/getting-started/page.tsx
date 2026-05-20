"use client";

import React from "react";

export default function GettingStartedHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Getting Started with OHC</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Welcome to OneHumanCorp! Setting up your new online store is fast and easy. Follow these steps to get your business up and running.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Step 1: Tell us about your business</h2>
          <p>First, use our Store Builder tool to describe what you sell, who your customers are, and the feel of your brand. Our AI agents will use this information to automatically build your entire storefront.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Step 2: Add your products</h2>
          <p>Once your store is built, you can add specific items or services that you sell. Make sure to include clear photos and descriptions so customers know exactly what they are buying.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Step 3: Connect a bank account</h2>
          <p>To get paid when you make a sale, you need to connect your bank account. Go to the Payments section to set this up.</p>

          <div className="mt-8 p-4 bg-blue-50 rounded-xl border border-blue-100">
            <h3 className="font-bold text-blue-900 mb-2">Need more help?</h3>
            <p className="text-sm text-blue-800">You can always ask our AI Support Agent by clicking the "Ask AI" tab in the Help menu.</p>
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
