"use client";

import React, { useEffect, useState } from 'react';
import { useOnboardingStore } from './store';
import { useRouter } from 'next/navigation';

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessDescription, setBusinessDescription,
    businessName, setBusinessName,
    businessType, setBusinessType,
    categories, setCategories,
    websiteTemplate, setWebsiteTemplate,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const router = useRouter();

  useEffect(() => {
    setIsLoaded(true);
    setStep(1);
    setBusinessDescription('');
    setError('');
  }, []);

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setError('');
    setStep(3); // Go to loading screen

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      // First step: intake
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

      const bType = intakeData.business_type || 'Online Store';
      const bName = intakeData.business_name || 'My Business';
      const fName = intakeData.initial_products?.[0]?.name || 'First Product';
      const fPrice = intakeData.initial_products?.[0]?.price || '10.00';
      const bCats = intakeData.categories || ['physical'];

      setBusinessType(bType);
      setBusinessName(bName);
      setFirstProductName(fName);
      setFirstProductPrice(fPrice);
      setCategories(bCats);

      // Second step: start
      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: bType,
          company_name: bName,
          company_description: businessDescription,
          selling_categories: bCats,
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: websiteTemplate || 'Modern',
          first_product_name: fName,
          first_product_price: fPrice,
          domain_choice: 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);

      // Update local storage to reflect activation
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('business_name', bName);
      }

      router.push('/dashboard');

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(2); // Go back to last input screen on error
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
            <div className="flex flex-col flex-1 justify-center animate-fade-in text-center">
              <div className="w-20 h-20 bg-[#0066FF]/10 rounded-full flex items-center justify-center mx-auto mb-8">
                <svg className="w-10 h-10 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h2 className="text-4xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Let's build your business.</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-base mb-12 px-4">
                Launch your business in 10 minutes from your phone. Our AI handles the heavy lifting.
              </p>

              <div className="mt-auto">
                <button
                  onClick={() => setStep(2)}
                  className="w-full bg-[#0066FF] text-white py-4 rounded-[8px] font-bold text-lg shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all min-h-[44px]"
                >
                  Start Now
                </button>
              </div>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-6 flex items-center gap-1 min-h-[44px] min-w-[44px]">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Tell me what you sell in one sentence.</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Be as descriptive as you like. We'll use this to build your custom storefront.
              </p>

              <div className="space-y-4 flex-1">
                <textarea
                  value={businessDescription}
                  onChange={(e) => setBusinessDescription(e.target.value)}
                  placeholder="e.g. I bake custom vegan cakes in Portland, OR..."
                  className="w-full p-4 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-white/60 dark:bg-black/30 backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] h-40 resize-none transition-all shadow-inner text-base"
                />
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={!businessDescription.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white py-4 rounded-[8px] font-bold text-lg shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50 min-h-[44px]"
                >
                  Generate My Business
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
             <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
               <div className="space-y-3">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog...</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings...</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront...</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1.5s' }}>Onboarding your AI agents...</p>
               </div>
             </div>
          )}
        </div>
      </div>
    </div>
  );
}
