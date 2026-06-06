"use client";

import React from 'react';
import Link from 'next/link';
import { WithTooltip } from '../../../components/TooltipRegistry';

export default function MyStoreHelpPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-3xl mx-auto">
        <Link href="/help" className="inline-flex items-center text-blue-600 hover:text-blue-800 mb-8 transition-colors font-medium">
          <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Back to Help Center
        </Link>

        <article className="bg-white/70 backdrop-blur-[20px] saturate-200 p-8 sm:p-12 rounded-3xl shadow-[0_8px_32px_rgba(0,0,0,0.04)] border border-white/60">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 mb-6 tracking-tight">My Store: Adding Products & Managing Inventory</h1>

          <div className="prose prose-blue max-w-none prose-headings:font-outfit prose-headings:font-bold prose-p:text-gray-600 prose-p:leading-relaxed">
            <p className="text-lg text-gray-700 font-medium mb-8">
              Setting up your storefront is easy. You don't need any technical skills to add products, set prices, and manage what's in stock.
            </p>

            <h2 className="text-2xl mt-10 mb-4 text-gray-900 border-b border-gray-100 pb-2">Adding Your First Product</h2>
            <p>
              To add a new item to sell, follow these simple steps:
            </p>
            <ol className="list-decimal pl-5 space-y-3 mb-8">
              <li>Go to your <Link href="/dashboard" className="text-blue-600 hover:underline">Dashboard</Link> and click on <strong>Storefront</strong> in the menu.</li>
              <li>Click the <WithTooltip id="btn-new-product-tooltip" defaultText="Click here to add something new to sell."><span className="bg-blue-50 text-blue-700 px-2 py-1 rounded-md border border-blue-100 cursor-help font-medium">Add Product</span></WithTooltip> button.</li>
              <li>Upload a clear, bright photo of what you're selling.</li>
              <li>Type in a name and a short description. (If you're stuck, ask your AI helper to write the description for you!)</li>
              <li>Set your price and tell us how many you have in stock.</li>
              <li>Click <strong>Save</strong>. Your product is now live on your site!</li>
            </ol>

            <h2 className="text-2xl mt-10 mb-4 text-gray-900 border-b border-gray-100 pb-2">Tracking Your Inventory</h2>
            <p>
              We automatically keep track of how many items you have left. When a customer buys something, the stock number goes down.
            </p>
            <div className="bg-amber-50 border-l-4 border-amber-400 p-4 rounded-r-lg my-6">
              <p className="m-0 text-amber-800 text-sm">
                <strong>Tip:</strong> If you run out of an item, it will automatically show as "Sold Out" on your website. You don't have to change anything manually!
              </p>
            </div>

            <h2 className="text-2xl mt-10 mb-4 text-gray-900 border-b border-gray-100 pb-2">Making Changes</h2>
            <p>
              Want to put an item on sale or change the photo? Just go back to the Storefront page, tap on the product, and make your changes. They will update on your live website instantly.
            </p>
          </div>
        </article>
      </div>
    </div>
  );
}
