'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function EdgeStorefrontSetupPage() {
  const router = useRouter();
  const [step, setStep] = useState<'initial' | 'setup' | 'success'>('initial');
  const [selectedOption, setSelectedOption] = useState<string | null>(null);

  const handleStartSetup = () => {
    setStep('setup');
  };

  const handleGenerate = async () => {
    // Simulate generation process
    setStep('success');
  };

  return (
    <AppShell title="Publish Storefront">
      <div className="max-w-md mx-auto p-6 bg-white dark:bg-gray-800 rounded-xl shadow-sm glassmorphism">
        {/* Back navigation button required by tests */}
        <button
          aria-label="Go back"
          onClick={() => router.push('/dashboard')}
          className="mb-4 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
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
              className="w-full py-2 px-4 bg-[#0071E3] hover:bg-blue-700 text-white glass-control transition-colors font-medium min-h-[44px]"
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
                className={`w-full text-left px-4 py-3 glass-control border min-h-[44px] ${
                  selectedOption === 'custom-cakes'
                    ? 'border-[#0066FF] bg-blue-50 dark:bg-blue-900/30 text-[#0066FF]'
                    : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-750'
                }`}
              >
                Custom Cakes
              </button>

              <button
                onClick={() => setSelectedOption('ready-to-buy')}
                className={`w-full text-left px-4 py-3 glass-control border min-h-[44px] ${
                  selectedOption === 'ready-to-buy'
                    ? 'border-[#0066FF] bg-blue-50 dark:bg-blue-900/30 text-[#0066FF]'
                    : 'border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-750'
                }`}
              >
                Ready-to-buy
              </button>
            </div>

            <button
              id="generate-storefront-btn"
              onClick={handleGenerate}
              disabled={!selectedOption}
              className="w-full py-2 px-4 bg-[#0071E3] hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-white glass-control transition-colors font-medium min-h-[44px]"
            >
              Generate & Publish
            </button>
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

            <div className="p-3 bg-gray-50 dark:bg-gray-900 rounded-lg mb-6 flex items-center justify-between border border-gray-200 dark:border-gray-700">
              <span className="truncate text-sm text-gray-800 dark:text-gray-200 font-mono">
                https://yourdomain.com/api/v1/builder/edge/store
              </span>
              <button
                id="copy-link-btn"
                className="ml-2 px-3 py-1 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 glass-control text-sm hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors min-h-[44px]"
                onClick={() => {}}
              >
                Copy
              </button>
            </div>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full py-2 px-4 bg-gray-100 hover:bg-gray-200 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-900 dark:text-white glass-control transition-colors font-medium min-h-[44px]"
            >
              Back to Dashboard
            </button>

            <div className="mt-4">
              <a href="/dashboard" className="text-[#0071E3] hover:underline text-sm ">
                Back to Dashboard
              </a>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
