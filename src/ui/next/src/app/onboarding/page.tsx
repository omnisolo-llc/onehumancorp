"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

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

  useEffect(() => {
    setIsLoaded(true);
    // Load state from backend when component mounts
    const loadState = async () => {
      try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
        const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';
        const res = await fetch('/api/onboarding/state', {
          headers: {
            'X-Tenant-ID': tenantId,
            'X-User-ID': userId,
          }
        });
        if (res.ok) {
          const data = await res.json();
          if (data && data.description) {
            setBusinessDescription(data.description);
          }
        }
      } catch (err) {
        console.error('Failed to load onboarding state', err);
      }
    };
    loadState();
  }, []);

  const handleStartOnboarding = async (intakeData: any) => {
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
          business_type: intakeData?.business_type || 'Online Store',
          company_name: intakeData?.business_name || 'My Business',
          company_description: businessDescription,
          selling_categories: intakeData?.categories || ['physical'],
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: websiteTemplate,
          first_product_name: intakeData?.initial_products?.[0]?.name || 'First Product',
          first_product_price: intakeData?.initial_products?.[0]?.price || '10.00',
          domain_choice: 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);
      setStep(5); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(1); // Go back to step 1 on error
    } finally {
      setIsLoading(false);
    }
  };

  const saveState = async (desc: string) => {
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';
      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: desc })
      });
    } catch (err) {
      console.error('Failed to save onboarding state', err);
    }
  };

  const handleDescriptionChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const value = e.target.value;
    setBusinessDescription(value);
    // Debounce save state would ideally be here, but for simplicity we save on blur or periodically.
    // Given the E2E test, we'll trigger save on change with a small delay or just directly
    // to ensure cross-device persistence is met.
    saveState(value);
  };

  const handleIntake = async () => {
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

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

      setBusinessType(intakeData.business_type || 'Online Store');
      setBusinessName(intakeData.business_name || 'My Business');
      setFirstProductName(intakeData.initial_products?.[0]?.name || 'First Product');
      setFirstProductPrice(intakeData.initial_products?.[0]?.price || '10.00');
      setCategories(intakeData.categories || ['physical']);

      await handleStartOnboarding(intakeData);
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred processing details');
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div className="w-full max-w-[375px] mx-auto bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative">
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
                  onChange={handleDescriptionChange}
                  placeholder="e.g. I bake custom vegan cakes in Portland, OR..."
                  className="w-full p-4 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-inner"
                />
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleIntake}
                  disabled={!businessDescription.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
                >
                  {isLoading ? 'Analyzing...' : 'Generate My Business'}
                </button>
              </div>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Review Details</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Here's what our AI figured out. Feel free to tweak these.
              </p>

              <div className="space-y-4 flex-1 overflow-y-auto pr-2">
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Name</label>
                  <input
                    type="text"
                    value={businessName}
                    onChange={(e) => setBusinessName(e.target.value)}
                    className="w-full p-3 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Type</label>
                  <input
                    type="text"
                    value={businessType}
                    onChange={(e) => setBusinessType(e.target.value)}
                    className="w-full p-3 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Categories (Comma separated)</label>
                  <input
                    type="text"
                    value={categories.join(', ')}
                    onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                    className="w-full p-3 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">First Product</label>
                      <input
                        type="text"
                        value={firstProductName}
                        onChange={(e) => setFirstProductName(e.target.value)}
                        className="w-full p-3 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                   </div>
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Price</label>
                      <input
                        type="text"
                        value={firstProductPrice}
                        onChange={(e) => setFirstProductPrice(e.target.value)}
                        className="w-full p-3 rounded-[8px] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                   </div>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(3)}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all"
                >
                  Continue
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Style & Team</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
              </p>

              <div className="space-y-4 flex-1">
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Website Template</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
                      <div
                        key={template}
                        onClick={() => setWebsiteTemplate(template)}
                        className={`p-3 rounded-[8px] border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(0,0,0,0.3)] hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
                      >
                        <div className="font-semibold">{template}</div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
                >
                  Launch Store
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
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
               <div className="space-y-2">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1.5s' }}>Onboarding your AI agents</p>
               </div>
             </div>
          )}

          {step === 5 && startResult && (
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
                  className="block w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-[8px] font-bold shadow-md hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full bg-[rgba(255,255,255,0.7)] dark:bg-[rgba(255,255,255,0.1)] backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-[rgba(255,255,255,0.5)] dark:border-[rgba(255,255,255,0.1)] p-4 rounded-[8px] font-bold shadow-sm hover:bg-[rgba(255,255,255,0.9)] dark:hover:bg-[rgba(255,255,255,0.2)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
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
