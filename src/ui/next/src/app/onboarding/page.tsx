"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

export default function OnboardingWizard() {
  const router = useRouter();

  // 0: Welcome, 1: Bio Input, 2: Magic Generation, 3: Activation
  const [step, setStep] = useState(0);

  const [businessBio, setBusinessBio] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [startResult, setStartResult] = useState<any>(null);

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setIsLoaded(true);
  }, []);

  const handleStartGeneration = async () => {
    if (!businessBio.trim()) return;

    setIsLoading(true);
    setError('');
    setStep(2); // Move to Magic Generation

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      // 1. Parse Intake
      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: businessBio })
      });

      if (!intakeRes.ok) throw new Error('Failed to parse business details');
      const intakeData = await intakeRes.json();

      // 2. Start Onboarding Automatically
      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: intakeData.business_type || 'Online Store',
          company_name: intakeData.business_name || 'My Business',
          company_description: businessBio,
          selling_categories: intakeData.categories || ['physical'],
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: 'Modern',
          first_product_name: intakeData.initial_products?.[0]?.name || 'First Product',
          first_product_price: intakeData.initial_products?.[0]?.price || '10.00',
          domain_choice: 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) throw new Error('Failed to build your store');
      const result = await startRes.json();

      setStartResult(result);
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_onboarded', 'true');
        localStorage.setItem('tenant', result.organization_id || tenantId);
        localStorage.setItem('business_name', intakeData.business_name || 'My Business');
      }

      setStep(3); // Activation Screen
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during generation');
      setStep(1); // Go back to input on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center items-center px-4 py-8 sm:px-6 lg:px-8">
      <div id="setup-screen" className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative bg-white/70 dark:bg-black/40 backdrop-blur-xl border border-white/40 dark:border-white/10">
        <div className="p-6 flex-1 flex flex-col">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-3 rounded-[8px] text-sm shrink-0">
              {error}
            </div>
          )}

          {/* Step 0: Welcome Screen */}
          {step === 0 && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-gradient-to-tr from-blue-500 to-purple-500 rounded-3xl flex items-center justify-center mb-8 shadow-lg">
                <svg className="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">OneHumanCorp</h1>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-base mb-10 leading-relaxed px-2">
                The simplest way to run your business. Zero technical knowledge required.
              </p>
              <div className="mt-auto w-full">
                <button
                  onClick={() => setStep(1)}
                  className="w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-xl font-bold text-lg shadow-md hover:opacity-90 active:scale-[0.98] transition-all duration-200"
                >
                  Launch your business in 5 minutes
                </button>
              </div>
            </div>
          )}

          {/* Step 1: Bio Input */}
          {step === 1 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(0)} className="self-start text-[#0066FF] text-sm font-semibold mb-6 flex items-center gap-1 hover:opacity-80 transition-opacity">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Describe what you do</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Tell us about your business, products, or services. Our AI will handle the rest.
              </p>

              <div className="flex-1 flex flex-col mb-6">
                <textarea
                  autoFocus
                  value={businessBio}
                  onChange={(e) => setBusinessBio(e.target.value)}
                  placeholder="e.g., I bake custom vegan cakes for weddings and parties..."
                  className="w-full flex-1 p-4 rounded-xl border border-gray-200 dark:border-white/10 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/20 outline-none bg-white/50 dark:bg-black/20 text-[#1D1D1F] dark:text-[#F5F5F7] text-base resize-none transition-all shadow-inner"
                />
              </div>

              <div className="mt-auto pb-2 w-full">
                <button
                  onClick={handleStartGeneration}
                  disabled={!businessBio.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold text-lg shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                >
                  {isLoading ? 'Processing...' : 'Generate Store'}
                  {!isLoading && <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>}
                </button>
              </div>
            </div>
          )}

          {/* Step 2: Magic Generation */}
          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-24 h-24 relative mb-8">
                <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6">Building your store...</h2>
              <div className="space-y-4 w-full px-4">
                <div className="flex items-center gap-3 opacity-100 transition-opacity">
                  <div className="w-5 h-5 rounded-full bg-green-100 flex items-center justify-center text-green-600">✓</div>
                  <p className="text-sm text-gray-700 dark:text-gray-300 font-medium">Analyzing description</p>
                </div>
                <div className="flex items-center gap-3 animate-pulse">
                  <div className="w-5 h-5 rounded-full border-2 border-[#0066FF] border-t-transparent animate-spin"></div>
                  <p className="text-sm text-gray-700 dark:text-gray-300 font-medium">Generating catalog</p>
                </div>
                <div className="flex items-center gap-3 opacity-50">
                  <div className="w-5 h-5 rounded-full border-2 border-gray-300"></div>
                  <p className="text-sm text-gray-700 dark:text-gray-300 font-medium">Hiring AI agents</p>
                </div>
              </div>
            </div>
          )}

          {/* Step 3: Activation Screen */}
          {step === 3 && (
            <div className="flex flex-col flex-1 items-center text-center animate-fade-in py-4">
              <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6 mt-4">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">You're Live!</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-2">
                Your business has been successfully launched and is ready for customers.
              </p>

              <div className="w-full bg-white dark:bg-black/30 rounded-xl border border-gray-100 dark:border-white/10 p-4 mb-auto shadow-sm">
                 <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                 <div className="flex items-center justify-between bg-gray-50 dark:bg-black/50 rounded-lg p-3 border border-gray-200 dark:border-white/5">
                    <span className="text-[#0066FF] font-semibold text-sm truncate">my-business.ohc.store</span>
                    <button
                      onClick={() => navigator.clipboard.writeText('https://my-business.ohc.store')}
                      className="ml-2 text-gray-500 hover:text-[#0066FF] transition-colors"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                    </button>
                 </div>
              </div>

              <div className="w-full mt-auto pb-2">
                <button
                  onClick={() => router.push('/dashboard')}
                  className="w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-xl font-bold text-lg shadow-md hover:opacity-90 active:scale-[0.98] transition-all duration-200"
                >
                  Go to Dashboard
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
