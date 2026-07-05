'use client';

import React, { useState } from 'react';
import Link from 'next/link';

export default function LeadGenCampaignPage() {
  const [budget, setBudget] = useState('');
  const [radius, setRadius] = useState('10');
  const [zipCode, setZipCode] = useState('');
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState(false);
  const [error, setError] = useState('');

  const handleStartCampaign = async () => {
    if (!budget || !radius || !zipCode) {
      setError('Please fill out all fields');
      return;
    }

    setLoading(true);
    setError('');

    try {
      // In a real app, we would make a gRPC/REST call here
      // We will create the API route next
      const res = await fetch('/api/v1/growth/campaign/lead-gen', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          budget: parseFloat(budget),
          radius_miles: parseInt(radius, 10),
          zip_code: zipCode,
        }),
      });

      if (!res.ok) {
        throw new Error('Failed to start campaign');
      }

      setSuccess(true);
    } catch (err: any) {
      setError(err.message || 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-50 p-4 sm:p-6 lg:p-8 flex items-center justify-center font-inter">
      <div className="w-full max-w-md mx-auto glassmorphism p-8 rounded-2xl border border-white/50 shadow-xl">
        <div className="mb-6">
          <Link href="/dashboard" className="text-sm font-semibold text-[#0071E3] hover:text-blue-800 transition-colors">
            &larr; Back to Dashboard
          </Link>
        </div>

        <div className="text-center mb-8">
          <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center text-3xl mx-auto mb-4">
            🎯
          </div>
          <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
            Local Lead Generator
          </h1>
          <p className="text-sm text-gray-600">
            Let our AI agents find customers in your area. Set your budget and location, and we'll handle the rest invisibly.
          </p>
        </div>

        {success ? (
          <div className="bg-green-50 border border-green-200 text-green-800 rounded-xl p-6 text-center">
            <h3 className="text-lg font-bold mb-2">Campaign Started! 🚀</h3>
            <p className="text-sm mb-4">
              Our Marketing & Advertising agent is now actively seeking leads within {radius} miles of {zipCode}. We'll notify you when a booking is made.
            </p>
            <Link href="/dashboard" className="inline-block px-6 py-2 bg-green-600 text-white font-semibold rounded-lg hover:bg-green-700 transition-colors">
              Return to Dashboard
            </Link>
          </div>
        ) : (
          <div className="space-y-6">
            <div>
              <label htmlFor="budget" className="block text-sm font-semibold text-gray-700 mb-2">
                Weekly Budget ($)
              </label>
              <div className="relative">
                <span className="absolute left-4 top-1/2 -translate-y-1/2 text-gray-500 font-semibold">$</span>
                <input
                  type="number"
                  id="budget"
                  inputMode="numeric"
                  placeholder="50"
                  value={budget}
                  onChange={(e) => setBudget(e.target.value)}
                  className="w-full pl-8 pr-4 py-3 bg-white border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all"
                />
              </div>
            </div>

            <div>
              <label htmlFor="zipCode" className="block text-sm font-semibold text-gray-700 mb-2">
                Target Zip Code
              </label>
              <input
                type="text"
                id="zipCode"
                inputMode="numeric"
                placeholder="90210"
                value={zipCode}
                onChange={(e) => setZipCode(e.target.value)}
                className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all"
              />
            </div>

            <div>
              <label htmlFor="radius" className="block text-sm font-semibold text-gray-700 mb-2">
                Search Radius (miles)
              </label>
              <div className="flex items-center gap-4">
                <input
                  type="range"
                  id="radius"
                  min="1"
                  max="50"
                  value={radius}
                  onChange={(e) => setRadius(e.target.value)}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
                />
                <span className="text-sm font-bold text-[#0071E3] min-w-[3rem] text-right">
                  {radius} mi
                </span>
              </div>
            </div>

            {error && (
              <div className="text-sm text-red-600 bg-red-50 p-3 rounded-lg border border-red-100">
                {error}
              </div>
            )}

            <button
              onClick={handleStartCampaign}
              disabled={loading}
              className="w-full py-4 bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold rounded-xl shadow-lg hover:shadow-xl hover:from-blue-700 hover:to-indigo-700 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? 'Launching Agents...' : 'Start Finding Jobs'}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
