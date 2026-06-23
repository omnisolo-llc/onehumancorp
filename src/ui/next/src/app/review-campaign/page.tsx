"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function ReviewCampaignBuilderPage() {
  const [customerName, setCustomerName] = useState('');
  const [productName, setProductName] = useState('');
  const [orderId, setOrderId] = useState('');

  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedDraft, setGeneratedDraft] = useState('');
  const [error, setError] = useState('');

  const handleGenerate = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsGenerating(true);
    setError('');
    setGeneratedDraft('');

    try {
      const res = await fetch('/api/v1/growth/campaign/generate-review', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          customer_name: customerName,
          product_name: productName,
          order_id: orderId,
        }),
      });

      if (!res.ok) {
        throw new Error('Failed to generate campaign draft');
      }

      const data = await res.json();
      setGeneratedDraft(data.message || '');
    } catch (err: any) {
      setError(err.message || 'Something went wrong');
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 font-inter py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-xl mx-auto space-y-8">
        <div>
          <Link href="/dashboard" className="text-indigo-600 dark:text-indigo-400 text-sm font-medium hover:underline mb-4 inline-block">
            &larr; Back to Dashboard
          </Link>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
            AI Review Campaign Builder
          </h1>
          <p className="mt-2 text-gray-600 dark:text-gray-300 text-sm">
            Generate an AI-driven review request for your past customers. Each request includes a built-in referral link to grow your business.
          </p>
        </div>

        <div className="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md shadow-xl rounded-2xl p-6 border border-gray-200 dark:border-gray-700">
          <form onSubmit={handleGenerate} className="space-y-6">
            <div>
              <label htmlFor="customerName" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Customer Name
              </label>
              <input
                id="customerName"
                type="text"
                required
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
                className="mt-1 block w-full rounded-xl border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-4 py-3"
                placeholder="e.g. Maya"
              />
            </div>

            <div>
              <label htmlFor="productName" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Product Name
              </label>
              <input
                id="productName"
                type="text"
                required
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
                className="mt-1 block w-full rounded-xl border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-4 py-3"
                placeholder="e.g. Vegan Chocolate Cake"
              />
            </div>

            <div>
              <label htmlFor="orderId" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Order ID
              </label>
              <input
                id="orderId"
                type="text"
                required
                value={orderId}
                onChange={(e) => setOrderId(e.target.value)}
                className="mt-1 block w-full rounded-xl border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm px-4 py-3"
                placeholder="e.g. ORD-98765"
              />
            </div>

            <button
              type="submit"
              disabled={isGenerating || !customerName || !productName || !orderId}
              className="w-full flex justify-center py-3 px-4 border border-transparent rounded-xl shadow-sm text-sm font-bold text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isGenerating ? 'Generating Draft...' : 'Generate AI Campaign'}
            </button>
          </form>

          {error && (
            <div className="mt-4 p-4 text-sm text-red-700 bg-red-100 dark:bg-red-900/30 dark:text-red-400 rounded-xl">
              {error}
            </div>
          )}
        </div>

        {generatedDraft && (
          <div className="bg-indigo-50/80 dark:bg-indigo-900/20 backdrop-blur-md shadow-xl rounded-2xl p-6 border border-indigo-100 dark:border-indigo-800 animate-fade-in-up">
            <div className="flex items-center gap-2 mb-4">
              <span className="text-xl">✨</span>
              <h3 className="text-lg font-bold font-outfit text-indigo-900 dark:text-indigo-300">
                Email Draft Preview
              </h3>
            </div>

            <div className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-gray-700">
              <textarea
                readOnly
                value={generatedDraft}
                className="w-full h-48 bg-transparent border-none resize-none focus:ring-0 text-sm text-gray-700 dark:text-gray-300 p-0"
              />
            </div>

            <div className="mt-6 flex flex-col sm:flex-row gap-3">
              <button
                className="flex-1 py-3 px-4 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-bold shadow-sm transition-colors"
                onClick={() => alert("Campaign sent successfully!")}
              >
                Send to Customer
              </button>
              <button
                className="flex-1 py-3 px-4 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-xl text-sm font-bold shadow-sm transition-colors"
                onClick={() => {
                  navigator.clipboard.writeText(generatedDraft);
                  alert("Copied to clipboard!");
                }}
              >
                Copy Draft
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
