"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function CampaignsPage() {
  const router = useRouter();
  const [name, setName] = useState('');
  const [maxCapacity, setMaxCapacity] = useState('50');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const [campaign, setCampaign] = useState<any>(null);
  const [errorMessage, setErrorMessage] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setErrorMessage('');

    try {
      const response = await fetch('/api/v1/campaigns', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name,
          maxCapacity,
          tenant_id: 'tenant_demo'
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to create waitlist campaign');
      }

      const data = await response.json();
      setCampaign(data.campaign);
      setIsSuccess(true);
    } catch (error: any) {
      console.error(error);
      setErrorMessage(error.message || 'An error occurred.');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Campaign Monitoring</h1>
        <button
          onClick={() => router.push('/')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back Home
        </button>
      </header>

      <main className="flex-1 flex flex-col p-6 md:p-12 w-full max-w-2xl mx-auto">
        {isSuccess && campaign ? (
          <div className="w-full bg-white/65 backdrop-blur-md rounded-2xl shadow-sm border border-white/40 p-8 flex flex-col mb-8">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Active Drop</h2>
            <div className="flex items-center justify-between mt-4">
              <span className="text-lg font-medium">{campaign.name}</span>
              <span className="bg-indigo-100 text-indigo-800 text-sm font-semibold px-3 py-1 rounded-full">
                {campaign.secured} / {campaign.maxCapacity} Secured
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2.5 mt-4">
              <div className="bg-indigo-600 h-2.5 rounded-full" style={{ width: `${(campaign.secured / campaign.maxCapacity) * 100}%` }}></div>
            </div>
          </div>
        ) : null}

        <div className="w-full bg-white/65 backdrop-blur-md p-6 rounded-2xl shadow-sm border border-white/40">
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Launch New Drop</h2>
          <form onSubmit={handleSubmit} className="flex flex-col gap-4">
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">Campaign Name</label>
              <input
                type="text"
                id="name"
                required
                placeholder="e.g. Thanksgiving Pies"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all shadow-inner"
              />
            </div>
            <div>
              <label htmlFor="capacity" className="block text-sm font-medium text-gray-700 mb-1">Max Capacity</label>
              <input
                type="number"
                id="capacity"
                required
                min="1"
                value={maxCapacity}
                onChange={(e) => setMaxCapacity(e.target.value)}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all shadow-inner"
              />
            </div>
            <button
              type="submit"
              disabled={isSubmitting || !name}
              className={`w-full py-3 px-4 font-semibold text-white rounded-xl shadow-md transition-all ${
                isSubmitting || !name
                  ? 'bg-indigo-400 cursor-not-allowed'
                  : 'bg-indigo-600 hover:bg-indigo-700 hover:-translate-y-0.5'
              }`}
            >
              {isSubmitting ? 'Launching...' : 'Publish 1-Tap Drop'}
            </button>
            {errorMessage && <p className="text-red-500 text-sm mt-2">{errorMessage}</p>}
          </form>
        </div>
      </main>
    </div>
  );
}
