"use client";

import React from "react";

export default function MyStoreHelp() {
  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-2xl mx-auto bg-white p-8 rounded-2xl shadow-sm border border-gray-100">
        <a href="/dashboard" className="text-blue-600 text-sm font-bold mb-6 inline-block hover:underline">← Back to Dashboard</a>
        <h1 className="text-3xl font-extrabold text-gray-900 font-outfit mb-6">Managing Your Store</h1>
        <div className="prose prose-blue text-gray-600">
          <p>Your store is where customers see your business. Here is how to keep it looking great and up-to-date.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Adding New Products</h2>
          <p>You can add a new product at any time. You will need a name, price, and a good photo. Try to describe your product simply so customers know why they should buy it.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Changing Your Store's Look</h2>
          <p>If you want to change the colors or layout of your store, you can use our built-in themes. You don't need any design skills. Just pick a theme that matches your brand.</p>

          <h2 className="text-xl font-bold text-gray-800 mt-8 mb-4">Organizing Your Items</h2>
          <p>If you sell many different things, group them into categories (like "Shirts", "Pants", or "Accessories"). This makes it easier for customers to find what they want.</p>
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
