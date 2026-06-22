"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function PreOrderWidgetPage() {
  const [productName, setProductName] = useState('');
  const [offerText, setOfferText] = useState('');
  const [theme, setTheme] = useState('light');
  const [showEmbed, setShowEmbed] = useState(false);

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] p-8 font-sans">
      <div className="max-w-4xl mx-auto">
        <Link href="/dashboard" className="text-blue-600 hover:underline mb-8 inline-block">
          &larr; Back to Dashboard
        </Link>

        <h1 className="text-4xl font-bold mb-2 text-gray-900 dark:text-white">Pre-Order Waitlist Engine</h1>
        <p className="text-gray-600 dark:text-gray-400 mb-8">
          Configure your viral waitlist widget. Capture emails and allow customers to reserve their spot.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
          {/* Configuration Form */}
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Product Name
              </label>
              <input
                type="text"
                placeholder="e.g. The Vegan Chocolate Cake"
                className="w-full px-4 py-2 border rounded-xl dark:bg-black/20 dark:border-white/10 dark:text-white"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Special Offer (Optional)
              </label>
              <input
                type="text"
                placeholder="e.g. Get 10% off your pre-order!"
                className="w-full px-4 py-2 border rounded-xl dark:bg-black/20 dark:border-white/10 dark:text-white"
                value={offerText}
                onChange={(e) => setOfferText(e.target.value)}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                Theme
              </label>
              <div className="flex space-x-4">
                <button
                  onClick={() => setTheme('light')}
                  className={`px-4 py-2 rounded-lg border ${theme === 'light' ? 'bg-blue-50 border-blue-200 text-blue-700' : 'bg-white dark:bg-black/20 dark:border-white/10'}`}
                >
                  Light
                </button>
                <button
                  onClick={() => setTheme('dark')}
                  className={`px-4 py-2 rounded-lg border ${theme === 'dark' ? 'bg-blue-900 border-blue-700 text-blue-100' : 'bg-white dark:bg-black/20 dark:border-white/10 text-black dark:text-white'}`}
                >
                  Dark
                </button>
              </div>
            </div>

            <button
              onClick={() => setShowEmbed(true)}
              className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-medium transition-colors"
            >
              Get Widget Embed Code
            </button>
          </div>

          {/* Live Preview */}
          <div className={`p-8 rounded-2xl border ${theme === 'light' ? 'bg-white border-gray-200 text-black' : 'bg-gray-900 border-gray-800 text-white'}`}>
            <h3 className="text-sm font-semibold text-gray-400 mb-6 uppercase tracking-wider">Live Preview</h3>

            <div className="text-center space-y-4">
              <div className="w-16 h-16 bg-blue-100 dark:bg-blue-900/50 rounded-full flex items-center justify-center mx-auto text-2xl mb-4">
                ✨
              </div>
              <h2 className="text-2xl font-bold">
                {productName || 'Your Product Name'}
              </h2>
              {offerText && (
                <p className={`inline-block px-3 py-1 rounded-full text-sm font-medium ${theme === 'light' ? 'bg-green-100 text-green-800' : 'bg-green-900/50 text-green-300'}`}>
                  {offerText}
                </p>
              )}
              <p className={theme === 'light' ? 'text-gray-600' : 'text-gray-400'}>
                Join the waitlist to get notified when we launch. Spots are limited!
              </p>

              <div className="mt-6 flex space-x-2">
                <input
                  type="email"
                  placeholder="Enter your email"
                  className={`flex-1 px-4 py-2 rounded-lg border ${theme === 'light' ? 'bg-white border-gray-300' : 'bg-gray-800 border-gray-700 text-white'}`}
                />
                <button className="px-6 py-2 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700">
                  Join
                </button>
              </div>
              <p className={`text-xs mt-4 ${theme === 'light' ? 'text-gray-500' : 'text-gray-500'}`}>
                Join 1,204 others on the waitlist
              </p>
            </div>
          </div>
        </div>

        {/* Embed Modal */}
        {showEmbed && (
          <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
            <div className="bg-white dark:bg-gray-900 rounded-2xl p-8 max-w-lg w-full">
              <div className="flex justify-between items-center mb-6">
                <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Embed Your Waitlist</h2>
                <button onClick={() => setShowEmbed(false)} className="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 text-2xl">
                  &times;
                </button>
              </div>
              <p className="text-gray-600 dark:text-gray-400 mb-4">
                Copy and paste this code into your website's HTML where you want the waitlist to appear.
              </p>
              <div className="bg-gray-100 dark:bg-black/50 p-4 rounded-xl font-mono text-sm text-gray-800 dark:text-gray-200 overflow-x-auto mb-6">
                {`<div id="ohc-pre-order-widget" data-product="${productName}" data-offer="${offerText}" data-theme="${theme}"></div>`}
                <br/>
                {`<script src="https://assets.onehumancorp.com/widgets/pre-order.js" async></script>`}
              </div>
              <button onClick={() => setShowEmbed(false)} className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-medium transition-colors">
                Done
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
