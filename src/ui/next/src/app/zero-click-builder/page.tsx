'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

type BuilderState = 'idle' | 'generating' | 'success';

export default function ZeroClickBuilderPage() {
  const router = useRouter();
  const [prompt, setPrompt] = useState('');
  const [builderState, setBuilderState] = useState<BuilderState>('idle');
  const [error, setError] = useState('');

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      setError('Please tell us about your business first.');
      return;
    }

    setError('');
    setBuilderState('generating');

    try {
      const response = await fetch('/api/v1/onboarding/start_zero_click', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt }),
      });

      if (!response.ok) {
        throw new Error('Failed to generate store');
      }

      const data = await response.json();
      console.log('Success:', data);
      setBuilderState('success');
    } catch (err) {
      console.error(err);
      setError('An error occurred while generating your store. Please try again.');
      setBuilderState('idle');
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center p-4" style={{ backgroundColor: '#F5F5F7' }}>
      <style dangerouslySetInnerHTML={{__html: `
        :root { color-scheme: light dark; }
        @media (prefers-color-scheme: dark) {
          .page-bg { background-color: #000000; }
        }
      `}} />
      <div className="w-full max-w-[375px] mx-auto glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10 relative overflow-hidden">

        {builderState === 'idle' && (
          <div className="flex flex-col gap-6 relative z-10">
            <div>
              <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Zero-Click Business Generator
              </h1>
              <p className="text-[#1D1D1F] dark:text-[#F5F5F7] opacity-80 text-sm">
                Describe your business in one sentence, and our AI will build your entire storefront, catalog, and operations backend instantly.
              </p>
            </div>

            <div className="flex flex-col gap-2">
              <label htmlFor="prompt" className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">
                What do you do?
              </label>
              <textarea
                id="prompt"
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
                placeholder="e.g., I am a home baker in Austin selling custom vegan cakes and cupcakes."
                className="w-full min-h-[120px] p-4 rounded-[8px] bg-white/50 dark:bg-black/50 border border-gray-200 dark:border-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-[#0066FF] resize-none"
              />
              {error && <p className="text-[#FF3B30] text-sm mt-1">{error}</p>}
            </div>

            <button
              onClick={handleGenerate}
              className="w-full min-h-[44px] bg-[#0066FF] hover:bg-[#005CE6] text-white font-semibold rounded-[8px] transition-colors flex items-center justify-center mt-2"
            >
              Generate Store
            </button>
          </div>
        )}

        {builderState === 'generating' && (
          <div className="flex flex-col items-center justify-center gap-6 py-12 relative z-10">
            <div className="w-16 h-16 border-4 border-[#0066FF]/30 border-t-[#0066FF] rounded-full animate-spin"></div>
            <div className="text-center">
              <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Analyzing your business...
              </h2>
              <p className="text-[#1D1D1F] dark:text-[#F5F5F7] opacity-80 text-sm animate-pulse">
                Provisioning catalog and operations...
              </p>
            </div>
          </div>
        )}

        {builderState === 'success' && (
          <div className="flex flex-col gap-6 relative z-10">
            <div className="text-center">
              <div className="w-16 h-16 bg-[#34C759]/20 text-[#34C759] rounded-full flex items-center justify-center text-3xl mx-auto mb-4">
                ✓
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
                Your business is live!
              </h2>
              <p className="text-[#1D1D1F] dark:text-[#F5F5F7] opacity-80 text-sm">
                Your mobile storefront is ready for customers.
              </p>
            </div>

            <div className="w-full aspect-[9/16] bg-gray-100 dark:bg-gray-900 rounded-[12px] overflow-hidden border border-gray-200 dark:border-gray-800 shadow-inner relative">
                <div className="absolute inset-0 flex items-center justify-center text-gray-400">Loading Preview...</div>
                <iframe
                    title="Live Storefront Preview"
                    src="/preview-placeholder"
                    className="w-full h-full relative z-10 border-0"
                />
            </div>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full min-h-[44px] bg-[#0066FF] hover:bg-[#005CE6] text-white font-semibold rounded-[8px] transition-colors flex items-center justify-center"
            >
              Launch My Store
            </button>
          </div>
        )}

      </div>
    </div>
  );
}
