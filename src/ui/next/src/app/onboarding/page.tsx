"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessDescription, setBusinessDescription,
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
    setStep(2); // Go to loading screen

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      // Step A: Intake
      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: businessDescription })
      });

      if (!intakeRes.ok) {
        throw new Error('Failed to process business details');
      }

      const intakeData = await intakeRes.json();

      // We will map intake data to the start request
      // We need a deterministic but sensible fallback if intakeData doesn't parse well
      const businessType = intakeData.business_type || 'Online Store';
      const companyName = intakeData.business_name || 'My Business';
      const firstProductName = intakeData.initial_products?.[0]?.name || 'First Product';
      const firstProductPrice = intakeData.initial_products?.[0]?.price || '10.00';
      const categories = intakeData.categories || ['physical'];

      // Step B: Start Onboarding
      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: businessType,
          company_name: companyName,
          company_description: businessDescription,
          selling_categories: categories,
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: 'Modern',
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);
      setStep(3); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(1); // Go back on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full max-w-[375px] mx-auto bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative">
        <div className="p-6 flex-1 flex flex-col overflow-y-auto">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-3 rounded-[8px] text-sm">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
              </p>

              <div className="space-y-4 flex-1">
                <textarea
                  value={businessDescription}
                  onChange={(e) => setBusinessDescription(e.target.value)}
                  placeholder="e.g. I bake custom vegan cakes in Portland, OR..."
                  className="w-full p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-white/60 dark:bg-black/30 backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all shadow-inner"
                />
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={!businessDescription.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
                >
                  Generate My Business
                </button>
              </div>
            </div>
          )}

          {step === 2 && (
             <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
               <div className="space-y-2">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
               </div>
             </div>
          )}

          {step === 3 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">You're Live!</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
                {startResult.message || "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                <div className="p-3 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10 flex flex-col items-center mb-6">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-semibold">my-business.ohc.store</span>
                   </div>
                </div>

                <a
                  href="/dashboard"
                  className="block w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-[8px] font-bold shadow-md hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full bg-white/70 dark:bg-white/10 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 p-4 rounded-[8px] font-bold shadow-sm hover:bg-white/90 dark:hover:bg-white/20 active:scale-[0.98] transition-all"
                >
                  Preview Storefront
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
