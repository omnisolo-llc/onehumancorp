'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function EdgeStorefrontSetupPage() {
  const router = useRouter();
  const [step, setStep] = useState<'initial' | 'setup' | 'success'>('initial');
  const [selectedOption, setSelectedOption] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleStartSetup = () => {
    setStep('setup');
  };

  const handleGenerate = async () => {
    setError('Storefront publishing is unavailable because no edge-publishing API is connected.');
  };

  return (
    <AppShell title="Publish Storefront">
      <div className="max-w-md mx-auto p-6 rounded-[16px] shadow-sm glassmorphism translucent-glass-light dark:translucent-glass-dark">
        {/* Back navigation button required by tests */}
        <button
          aria-label="Go back"
          onClick={() => router.push('/dashboard')}
          className="mb-4 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 glass-control px-3 py-1 rounded-[8px]"
        >
          &larr; Back
        </button>

        {step === 'initial' && (
          <div>
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">
              Publish Storefront
            </h2>
            <p className="text-gray-600 dark:text-gray-300 mb-6">
              Get your business online instantly with our AI Promoter Agent.
            </p>
            <button
              id="start-setup-btn"
              onClick={handleStartSetup}
              className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] min-h-[44px] glass-control"
            >
              Start Setup
            </button>
          </div>
        )}

        {step === 'setup' && (
          <div>
            <h3 className="text-xl font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">
              Promoter Agent
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Choose your main offering type so we can optimize your storefront.
            </p>

            <div className="space-y-3 mb-6">
              <button
                id="select-custom-cakes-btn"
                onClick={() => setSelectedOption('custom-cakes')}
                className={`w-full text-left p-4 rounded-[8px] border glass-control transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] min-h-[44px] ${
                  selectedOption === 'custom-cakes'
                    ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]'
                    : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-gray-400 dark:hover:border-gray-500'
                }`}
              >
                Custom Cakes
              </button>

              <button
                onClick={() => setSelectedOption('ready-to-buy')}
                className={`w-full text-left p-4 rounded-[8px] border glass-control transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] min-h-[44px] ${
                  selectedOption === 'ready-to-buy'
                    ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]'
                    : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-gray-400 dark:hover:border-gray-500'
                }`}
              >
                Ready-to-buy
              </button>
            </div>

            <button
              id="generate-storefront-btn"
              onClick={handleGenerate}
              disabled={!selectedOption}
              className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px] min-h-[44px] glass-control disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Generate & Publish
            </button>
            {error && <p className="mt-3 text-sm text-red-600" role="status">{error}</p>}
          </div>
        )}

        {step === 'success' && (
          <div className="text-center">
            <div className="text-4xl mb-4">🎉</div>
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
              Storefront Live!
            </h2>
            <p className="text-gray-600 dark:text-gray-300 mb-6">
              Your edge-cached storefront is now available.
            </p>

            <div className="p-3 glass-control border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] mb-6 flex items-center justify-between">
              <span className="truncate text-sm text-gray-800 dark:text-gray-200 font-mono">
                https://yourdomain.com/api/v1/builder/edge/store
              </span>
              <button
                id="copy-link-btn"
                className="ml-2 px-3 py-1 glass-control rounded-[8px] min-h-[44px] hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] text-sm"
                onClick={() => {}}
              >
                Copy
              </button>
            </div>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full glass-control rounded-[8px] min-h-[44px] p-4 text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] font-medium border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]"
            >
              Back to Dashboard
            </button>

            <div className="mt-4">
              <a href="/dashboard" className="text-[#0066FF] hover:underline text-sm font-medium">
                Back to Dashboard
              </a>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
