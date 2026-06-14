"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ZeroClickBuilderPage() {
  const router = useRouter();
  const [prompt, setPrompt] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generationStep, setGenerationStep] = useState(0);
  const [generatedStore, setGeneratedStore] = useState<any>(null);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const steps = [
    "Analyzing your business...",
    "Designing storefront layout...",
    "Generating product catalog...",
    "Configuring booking systems...",
    "Finalizing your AI assistant..."
  ];

  const handleGenerate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!prompt.trim()) return;

    setIsGenerating(true);
    setGenerationStep(0);

    // Simulate generation steps for UI feedback
    const interval = setInterval(() => {
      setGenerationStep(prev => {
        if (prev < steps.length - 1) return prev + 1;
        clearInterval(interval);
        return prev;
      });
    }, 1500);

    try {
      const response = await fetch('/api/v1/growth/zero-click-builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt })
      });

      const data = await response.json();

      clearInterval(interval);
      setGenerationStep(steps.length - 1);

      setTimeout(() => {
        setIsGenerating(false);
        setGeneratedStore(data);
      }, 1000);
    } catch (error) {
      console.error("Error generating store:", error);
      setIsGenerating(false);
      clearInterval(interval);
      alert("Something went wrong. Please try again.");
    }
  };

  const handleShare = () => {
    const shareText = `I just built my AI-powered business in 30 seconds using OHC! Start your own for free: https://ohc.app/zero-click-builder?ref=new_store \n\n⚡ Powered by OHC`;
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`;
    window.open(shareUrl, '_blank');
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-outfit selection:bg-indigo-100 selection:text-indigo-900">
      <div className="w-full max-w-2xl">
        <div className="text-center mb-10">
          <div className="inline-flex items-center justify-center p-3 bg-indigo-100 dark:bg-indigo-900/30 rounded-2xl mb-4">
            <span className="text-3xl">✨</span>
          </div>
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white tracking-tight mb-3">
            Zero-Click Business Generator
          </h1>
          <p className="text-lg text-gray-600 dark:text-gray-400 max-w-xl mx-auto">
            Describe your business in one sentence. Our AI will instantly build your storefront, product catalog, and booking system.
          </p>
        </div>

        {!generatedStore ? (
          <div className="glassmorphism p-8 mb-8 relative overflow-hidden">
            {isGenerating && (
              <div className="absolute inset-0 z-10 bg-white/80 dark:bg-black/80 backdrop-blur-sm flex flex-col items-center justify-center">
                <div className="w-16 h-16 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-6"></div>
                <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-2 animate-pulse">
                  {steps[generationStep]}
                </h3>
                <p className="text-sm text-gray-500 font-medium">Please don't close this window.</p>
              </div>
            )}

            <form onSubmit={handleGenerate} className="space-y-6">
              <div>
                <label htmlFor="prompt" className="block text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                  What do you do?
                </label>
                <textarea
                  id="prompt"
                  rows={4}
                  className="w-full rounded-xl border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-4 py-3 text-gray-900 dark:text-white focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-shadow resize-none"
                  placeholder="e.g., I am a home baker in Austin selling custom vegan cakes and cupcakes."
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  disabled={isGenerating}
                  required
                />
              </div>

              <button
                type="submit"
                disabled={isGenerating || !prompt.trim()}
                className="w-full flex items-center justify-center gap-2 bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-4 rounded-xl font-semibold text-lg transition-all active:scale-[0.98] disabled:opacity-50 disabled:pointer-events-none shadow-sm hover:shadow-md"
              >
                <span>🚀</span> Generate My Business
              </button>
            </form>
          </div>
        ) : (
          <div className="glassmorphism p-8 mb-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="text-center mb-8">
              <div className="inline-flex items-center justify-center w-16 h-16 bg-green-100 dark:bg-green-900/30 text-green-600 rounded-full mb-4">
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
                </svg>
              </div>
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                Your business is live!
              </h2>
              <p className="text-gray-600 dark:text-gray-400">
                We've configured everything you need to start selling.
              </p>
            </div>

            <div className="space-y-6">
              <div className="bg-gray-50 dark:bg-gray-800 rounded-xl p-6 border border-gray-200 dark:border-gray-700">
                <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-4">Store Overview</h3>
                <dl className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <div>
                    <dt className="text-sm font-medium text-gray-500">Business Name</dt>
                    <dd className="mt-1 text-sm font-semibold text-gray-900 dark:text-white">{generatedStore.name}</dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-gray-500">Store URL</dt>
                    <dd className="mt-1 text-sm font-semibold text-indigo-600 truncate">{generatedStore.url}</dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-gray-500">Products Generated</dt>
                    <dd className="mt-1 text-sm font-semibold text-gray-900 dark:text-white">{generatedStore.products_count}</dd>
                  </div>
                  <div>
                    <dt className="text-sm font-medium text-gray-500">AI Agents Active</dt>
                    <dd className="mt-1 text-sm font-semibold text-gray-900 dark:text-white">Sales, Ops, Marketing</dd>
                  </div>
                </dl>
              </div>

              <div className="flex flex-col sm:flex-row gap-4 pt-4">
                <button
                  onClick={() => router.push('/dashboard')}
                  className="flex-1 flex items-center justify-center gap-2 bg-gray-900 dark:bg-white text-white dark:text-gray-900 px-6 py-3 rounded-xl font-semibold hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors"
                >
                  Go to Dashboard
                </button>
                <button
                  onClick={handleShare}
                  className="flex-1 flex items-center justify-center gap-2 bg-indigo-50 dark:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400 px-6 py-3 rounded-xl font-semibold hover:bg-indigo-100 dark:hover:bg-indigo-900/50 transition-colors"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z" />
                  </svg>
                  Share to Twitter
                </button>
              </div>
            </div>
          </div>
        )}

        <div className="text-center mt-8">
          <p className="text-sm font-semibold text-gray-500 flex items-center justify-center gap-1">
            ⚡ Powered by OHC
            {!hasPro && (
              <a href="/pricing" className="text-indigo-500 hover:text-indigo-600 hover:underline ml-1">
                (Upgrade to remove)
              </a>
            )}
          </p>
          <div className="flex justify-center mt-2">
            <PoweredByOHC tenantId="ohc" />
          </div>
        </div>
      </div>
    </div>
  );
}
