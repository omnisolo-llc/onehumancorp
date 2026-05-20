"use client";

import React from "react";

export default function MarketingHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Growing Your Business</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Having a great store is only the first step. Now you need people to visit it.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Sharing on Social Media</h2>
          <p>The easiest way to get your first customers is to share your store link on Facebook, Instagram, Twitter, or WhatsApp. Tell your friends and family what you are building!</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Emailing Your Customers</h2>
          <p>When people buy from you, collect their email addresses. You can send them special offers or let them know when you add new items. Our built-in marketing tools make sending emails very easy.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Using AI for Marketing</h2>
          <p>If you don't know what to post on social media, ask your AI Marketing Agent to write some posts for you. It can read your product descriptions and turn them into catchy advertisements.</p>
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
