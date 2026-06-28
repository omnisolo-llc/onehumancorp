"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { SetupIcon } from '../components/SetupIcon';
import { useOnboardingStore } from '../store';

const loadingStates = [
  "Analyzing business type...",
  "Generating product catalog...",
  "Setting up booking system...",
  "Provisioning AI Departments...",
  "Finalizing your storefront..."
];

export default function ZeroClickOnboarding() {
  const router = useRouter();
  const [prompt, setPrompt] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [loadingStateIndex, setLoadingStateIndex] = useState(0);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);
  const [organizationId, setOrganizationId] = useState('');

  const { updateState } = useOnboardingStore();

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isLoading) {
      interval = setInterval(() => {
        setLoadingStateIndex((prev) => (prev + 1) % loadingStates.length);
      }, 2000);
    }
    return () => clearInterval(interval);
  }, [isLoading]);

  const handleGenerate = async () => {
    if (!prompt.trim()) {
      setError('Please tell us about your business.');
      return;
    }
    setError('');
    setIsLoading(true);

    try {
      const backendUrl = typeof window !== 'undefined' && window.location.origin.includes('localhost') ? 'http://127.0.0.1:18789' : '';
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch(`${backendUrl}/api/onboarding/start_zero_click`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ prompt }),
      });

      if (!res.ok) {
        throw new Error('Failed to generate storefront');
      }

      const data = await res.json();

      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_onboarded', 'true');
        if (data.organization_id) {
          localStorage.setItem('tenant_id', data.organization_id);
          localStorage.setItem('tenant', data.organization_id);
        }
      }

      setOrganizationId(data.organization_id || '');
      setSuccess(true);

      // Update global state just in case
      updateState({ step: 5, startResult: data });

      // Navigate to success or dashboard
      if (typeof window !== 'undefined' && window.location.href.includes('setup.html')) {
           window.location.href = '/success.html';
      } else {
           setTimeout(() => {
               router.push('/dashboard');
           }, 2000);
      }

    } catch (err: any) {
      setError(err.message || 'An error occurred during generation.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] flex items-center justify-center p-4 font-inter text-[#1D1D1F] dark:text-[#F5F5F7]">
      <div className="w-[375px] max-w-full relative overflow-hidden bg-transparent">

        {!isLoading && !success && (
          <div className="flex flex-col gap-6 animate-fade-in bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] p-6 shadow-lg">
            <h1 className="text-2xl font-bold font-outfit text-center">Tell us about your business</h1>
            <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center">
              Our AI will build your entire storefront, products, and services in 30 seconds.
            </p>

            <textarea
              id="instant-bio"
              data-testid="instant-bio"
              className={`bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] w-full p-4 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] ${error ? "border border-[#FF3B30]" : "focus:border-[#0066FF] focus:ring-4 focus:ring-[#0066FF]/20"}`}
              placeholder="e.g. I run a local bakery that sells custom vegan cakes..."
              rows={6}
              style={{ resize: 'none' }}
              value={prompt}
              onChange={(e) => {
                setPrompt(e.target.value);
                if (error) setError('');
              }}
            />

            {error && (
              <p className="text-[#FF3B30] text-sm mt-1 text-center">{error}</p>
            )}

            <button
              id="generate-storefront-btn"
              data-testid="generate-storefront-btn"
              onClick={handleGenerate}
              disabled={!prompt.trim()}
              className="flex items-center justify-center w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
            >
              <span className="flex items-center gap-2"><SetupIcon name="sparkles" /> Build My Business</span>
            </button>

            <p className="text-xs text-center text-gray-400 dark:text-gray-500 mt-4">Powered by OHC</p>
          </div>
        )}

        {isLoading && (
          <div className="flex flex-col items-center justify-center py-20 animate-fade-in bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] p-6 shadow-lg">
            <div className="mb-8 relative w-20 h-20">
              <div className="absolute inset-0 bg-[#0066FF] rounded-full blur-xl opacity-30 animate-pulse"></div>
              <svg className="animate-spin w-full h-full text-[#0066FF]" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
            </div>
            <h3 className="text-xl font-bold font-outfit mb-2">Building Your Business</h3>
            <p className="text-[#0066FF] text-sm animate-pulse h-6">{loadingStates[loadingStateIndex]}</p>
          </div>
        )}

        {success && (
          <div className="flex flex-col items-center justify-center py-16 animate-fade-in bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] p-6 shadow-lg">
            <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6">
              <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 className="text-2xl font-bold font-outfit mb-2 text-center">Business Created!</h2>
            <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8">
              Your new workspace is ready.
            </p>

            <button
              onClick={() => router.push('/dashboard')}
              className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all rounded-[8px]"
            >
              1-Tap Launch
            </button>
          </div>
        )}

      </div>
    </div>
  );
}
