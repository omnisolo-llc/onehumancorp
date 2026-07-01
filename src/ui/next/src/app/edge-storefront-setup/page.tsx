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
      <div className="max-w-md mx-auto p-6 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] shadow-sm">
        {/* Back navigation button required by tests */}
        <button
          aria-label="Go back"
          onClick={() => router.push('/dashboard')}
          className="mb-4 text-sm text-[#1D1D1F] hover:text-[#0071E3] dark:text-[#F5F5F7] dark:hover:text-[#0071E3] transition-colors"
        >
          &larr; Back
        </button>

        {step === 'initial' && (
          <div>
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">
              Publish Storefront
            </h2>
            <p className="text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-6">
              Get your business online instantly. We will set up your public page.
            </p>
            <button
              id="start-setup-btn"
              onClick={handleStartSetup}
              className="w-full py-3 px-4 bg-[#0066FF] hover:bg-[#0071E3] text-white rounded-[8px] transition-colors font-medium min-h-[44px]"
            >
              Start Setup
            </button>
          </div>
        )}

        {step === 'setup' && (
          <div>
            <h3 className="text-xl font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">
              Store Setup
            </h3>
            <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4">
              What do you primarily sell?
            </p>

            <div className="space-y-3 mb-6">
              <button
                id="select-custom-cakes-btn"
                onClick={() => setSelectedOption('custom-cakes')}
                className={`w-full text-left px-4 py-3 rounded-[8px] border min-h-[44px] transition-colors ${
                  selectedOption === 'custom-cakes'
                    ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] dark:text-[#0071E3]'
                    : 'border-black/10 dark:border-white/10 text-[#1D1D1F] dark:text-[#F5F5F7] hover:bg-black/5 dark:hover:bg-white/5'
                }`}
              >
                Custom Orders (e.g. Cakes)
              </button>

              <button
                onClick={() => setSelectedOption('ready-to-buy')}
                className={`w-full text-left px-4 py-3 rounded-[8px] border min-h-[44px] transition-colors ${
                  selectedOption === 'ready-to-buy'
                    ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] dark:text-[#0071E3]'
                    : 'border-black/10 dark:border-white/10 text-[#1D1D1F] dark:text-[#F5F5F7] hover:bg-black/5 dark:hover:bg-white/5'
                }`}
              >
                Ready-to-buy Items
              </button>
            </div>

            <button
              id="generate-storefront-btn"
              onClick={handleGenerate}
              disabled={!selectedOption}
              className="w-full py-3 px-4 bg-[#0066FF] hover:bg-[#0071E3] disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-[8px] transition-colors font-medium min-h-[44px]"
            >
              Publish My Store
            </button>
          </div>
        )}

        {step === 'success' && (
          <div className="text-center">
            <div className="text-4xl mb-4">🎉</div>
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
              Storefront Live!
            </h2>
            <p className="text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-6">
              Your store is now ready for customers.
            </p>

            <div className="p-3 bg-black/5 dark:bg-white/5 rounded-[8px] mb-6 flex items-center justify-between border border-black/10 dark:border-white/10">
              <span className="truncate text-sm text-[#1D1D1F] dark:text-[#F5F5F7] font-mono select-all">
                https://yourdomain.com/store
              </span>
              <button
                id="copy-link-btn"
                className="ml-2 px-3 py-2 bg-white dark:bg-black/20 border border-black/10 dark:border-white/10 rounded-[8px] text-sm text-[#1D1D1F] dark:text-[#F5F5F7] hover:bg-black/5 dark:hover:bg-white/10 transition-colors min-h-[44px]"
                onClick={() => {}}
              >
                Copy
              </button>
            </div>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full py-3 px-4 bg-black/5 hover:bg-black/10 dark:bg-white/10 dark:hover:bg-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] transition-colors font-medium min-h-[44px]"
            >
              Back to Dashboard
            </button>
          </div>
        )}
      </div>
    </AppShell>
  );
}
