"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useOnboardingStore } from './store';

export default function OnboardingWizard() {
  const router = useRouter();
  const {
    step, setStep,
    businessName, setBusinessName,
    businessType, setBusinessType,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setIsLoaded(true);
  }, []);

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setError('');
    setStep(4); // Go to loading screen

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: businessType,
          company_name: businessName,
          company_description: `${businessName} is a ${businessType} business.`,
          selling_categories: [],
          business_theme: 'Modern',
          first_product_name: '',
          first_product_price: '',
          ai_agents: [],
          ai_auto_respond: true
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);

      router.push('/dashboard');
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(3); // Go back to last input screen on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div id="setup-screen" className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative">
        <div className="p-6 flex-1 flex flex-col overflow-y-auto">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-3 rounded-[8px] text-sm">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in text-center">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mx-auto mb-6">
                <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Welcome to OHC</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                Launch your business in under 10 minutes.
              </p>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(2)}
                  className="w-full bg-[#0066FF] text-white p-4 mac-button font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98]"
                >
                  Start a Business
                </button>
              </div>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What's the name of your business?</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Our AI will instantly generate your storefront, products, and back-office agents.
              </p>

              <div className="space-y-4 flex-1">
                <div>
                  <input
                    type="text"
                    value={businessName}
                    onChange={(e) => setBusinessName(e.target.value)}
                    placeholder="e.g. Maya's Custom Cakes"
                    className="w-full p-4 rounded-[12px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
                  />
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(3)}
                  disabled={businessName.trim().length < 3}
                  className="w-full bg-[#0066FF] text-white p-4 mac-button font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Next
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Business Type</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Select the type of business you're running.
              </p>

              <div className="space-y-3 flex-1 overflow-y-auto">
                {['Physical', 'Digital', 'Service', 'Food'].map(type => (
                  <div
                    key={type}
                    onClick={() => setBusinessType(type)}
                    className={`p-4 rounded-[12px] border cursor-pointer flex items-center justify-between transition-all ${businessType === type ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
                  >
                    <span className="font-semibold">{type}</span>
                    <div className={`w-5 h-5 rounded-full border flex items-center justify-center ${businessType === type ? 'border-[#0066FF] bg-[#0066FF]' : 'border-gray-400'}`}>
                       {businessType === type && <svg className="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                    </div>
                  </div>
                ))}
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={!businessType || isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 mac-button font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isLoading ? 'Building...' : 'Launch Store'}
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
             <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">AI is building your storefront...</h2>
               <div className="space-y-2">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1.5s' }}>Onboarding your AI agents</p>
               </div>
             </div>
          )}
        </div>
      </div>
    </div>
  );
}
