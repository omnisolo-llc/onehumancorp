'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function ZeroClickOnboarding() {
  const router = useRouter();
  const [prompt, setPrompt] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      setError('Please describe your business.');
      return;
    }

    setIsLoading(true);
    setError('');

    try {
      const res = await fetch('/api/onboarding/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt }),
      });

      if (!res.ok) {
        throw new Error('Failed to generate business. Please try again.');
      }

      const data = await res.json();
      if (data.tenant_id) {
        localStorage.setItem('tenant_id', data.tenant_id);
        localStorage.setItem('user_id', 'usr-zero-click');
      }

      // Poll for completion
      let attempts = 0;
      const interval = setInterval(async () => {
        attempts++;
        const stateRes = await fetch('/api/onboarding/state', {
          headers: { 'X-Tenant-ID': data.tenant_id, 'X-User-ID': 'usr-zero-click' }
        });
        if (stateRes.ok) {
          const stateData = await stateRes.json();
          if (stateData.status === 'launched') {
            clearInterval(interval);
            router.push('/dashboard?zero_click_success=true');
          }
        }
        if (attempts > 60) {
          clearInterval(interval);
          setError('Generation is taking longer than expected. Please check your dashboard later.');
          setIsLoading(false);
        }
      }, 2000);

    } catch (err: any) {
      setError(err.message || 'An unexpected error occurred.');
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50/50 p-6">
        <div className="relative p-8 rounded-3xl bg-white/60 backdrop-blur-xl border border-white/80 shadow-2xl overflow-hidden max-w-sm w-full text-center">
          <div className="absolute inset-0 bg-gradient-to-r from-blue-100/30 to-purple-100/30 animate-pulse"></div>
          <div className="relative z-10 flex flex-col items-center space-y-6">
            <div className="w-16 h-16 rounded-full border-4 border-blue-500 border-t-transparent animate-spin"></div>
            <h2 className="text-xl font-semibold text-gray-900 tracking-tight">Generating your business...</h2>
            <p className="text-sm text-gray-500">The Operations Manager is stocking your shelves and the System Architect is building your layout.</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50 p-6">
      <div className="w-full max-w-md bg-white rounded-3xl shadow-xl p-8 space-y-8 border border-gray-100">
        <div className="space-y-2 text-center">
          <h1 className="text-3xl font-bold tracking-tight text-gray-900">Zero-Click Setup</h1>
          <p className="text-gray-500 text-sm">Tell us about your business in a single sentence, and our AI will build everything you need.</p>
        </div>

        {error && (
          <div className="bg-red-50 text-red-600 p-4 rounded-xl text-sm border border-red-100">
            {error}
          </div>
        )}

        <div className="space-y-4">
          <textarea
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="e.g. I sell custom vegan cakes in Austin."
            rows={4}
            className="w-full p-4 text-gray-900 bg-gray-50 border border-gray-200 rounded-2xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all resize-none text-lg"
          />
          <button
            onClick={handleGenerate}
            className="w-full py-4 bg-black text-white rounded-2xl font-semibold text-lg hover:bg-gray-800 transition-colors shadow-md active:scale-[0.98]"
          >
            Generate Business
          </button>
        </div>
      </div>
    </div>
  );
}
