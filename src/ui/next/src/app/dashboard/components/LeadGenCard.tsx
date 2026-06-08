"use client";

import React, { useState } from 'react';

export default function LeadGenCard({ tenantId }: { tenantId: string }) {
  const [budget, setBudget] = useState('');
  const [zipCode, setZipCode] = useState('');
  const [status, setStatus] = useState<'idle' | 'loading' | 'success' | 'error'>('idle');
  const [errorMsg, setErrorMsg] = useState('');

  const handleStart = async () => {
    if (!budget || !zipCode) {
      setErrorMsg('Please enter budget and zip code.');
      setStatus('error');
      return;
    }

    const budgetCents = Math.floor(parseFloat(budget) * 100);

    setStatus('loading');
    setErrorMsg('');

    try {
      const res = await fetch('/api/v1/growth/lead-gen', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tenant_id: tenantId,
          zip_code: zipCode,
          weekly_budget_cents: budgetCents,
        }),
      });

      const data = await res.json();
      if (data.success) {
        setStatus('success');
      } else {
        setStatus('error');
        setErrorMsg(data.error || 'Failed to start campaign.');
      }
    } catch (e) {
      console.error(e);
      setStatus('error');
      setErrorMsg('Network error.');
    }
  };

  if (status === 'success') {
    return (
      <div className="block glassmorphism p-6 rounded-[16px] shadow-sm border border-white/40 dark:border-white/10 bg-green-50/50 dark:bg-green-900/20">
        <div className="flex items-start justify-between mb-4">
          <div className="w-12 h-12 rounded-full bg-green-100 dark:bg-green-900/50 flex items-center justify-center text-2xl">✨</div>
        </div>
        <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Campaign Active!</h3>
        <p className="text-sm text-gray-600 dark:text-gray-400">The AI is now finding leads for {zipCode}. Check your inbox soon.</p>
      </div>
    );
  }

  return (
    <div className="block glassmorphism p-6 rounded-[16px] shadow-sm border border-white/40 dark:border-white/10 relative overflow-hidden group">
      <div className="flex items-start justify-between mb-4">
        <div className="w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center text-2xl group-hover:scale-110 transition-transform">📍</div>
        <div className="text-blue-600 dark:text-blue-400 font-semibold text-sm bg-blue-50 dark:bg-blue-900/30 px-3 py-1 rounded-full">Leads</div>
      </div>
      <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Hyperlocal Lead Gen</h3>
      <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">Want more local jobs this week? Set a budget and let AI find customers.</p>

      <div className="space-y-3 mt-4" onClick={(e) => e.preventDefault() /* prevent parent Link if wrapped */}>
        <div>
          <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Weekly Budget ($)</label>
          <input
            type="number"
            inputMode="numeric"
            value={budget}
            onChange={e => setBudget(e.target.value)}
            className="w-full px-3 py-2 bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="e.g. 50"
            data-testid="lead-gen-budget"
          />
        </div>
        <div>
          <label className="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Target Zip Code</label>
          <input
            type="number"
            inputMode="numeric"
            value={zipCode}
            onChange={e => setZipCode(e.target.value)}
            className="w-full px-3 py-2 bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="e.g. 90210"
            data-testid="lead-gen-zip"
          />
        </div>
        {status === 'error' && <p className="text-xs text-red-500">{errorMsg}</p>}
        <button
          onClick={(e) => { e.preventDefault(); handleStart(); }}
          disabled={status === 'loading'}
          className="w-full mt-2 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 rounded-lg transition-colors disabled:opacity-50 text-sm"
          data-testid="lead-gen-submit"
        >
          {status === 'loading' ? 'Starting...' : 'Start Finding Jobs'}
        </button>
      </div>
    </div>
  );
}
