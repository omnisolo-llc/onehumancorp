"use client";

import React, { useEffect, useRef } from 'react';
import { useOnboardingStore } from './store';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessType, setBusinessType,
    businessName, setBusinessName,
    businessCategory, setBusinessCategory,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    template, setTemplate,
    domain, setDomain,
    isLoading, setIsLoading,
    error, setError,
    intakeData, setIntakeData,
    startResult, setStartResult
  } = useOnboardingStore();

  const lastSyncState = useRef("");

  const [isLoaded, setIsLoaded] = React.useState(false);

  // Load state from backend on initial mount
  useEffect(() => {
    if (isLoaded) return;

    // Zustand's persist middleware automatically loads the state from local storage before this.
    const loadState = async () => {
      try {
        const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
        const userId = localStorage.getItem('user_id') || 'test-user';
        const res = await fetch('/api/onboarding/state', {
          headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
        });
        if (res.ok) {
          const data = await res.json();
          if (data && Object.keys(data).length > 0) {
            // Prefer backend state if it's further along or on the same step but has more data
            if (data.step && data.step >= step) {
              setStep(data.step);
              if (data.businessType) setBusinessType(data.businessType);
              if (data.businessName) setBusinessName(data.businessName);
              if (data.businessCategory) setBusinessCategory(data.businessCategory);
              if (data.firstProductName) setFirstProductName(data.firstProductName);
              if (data.firstProductPrice) setFirstProductPrice(data.firstProductPrice);
              if (data.template) setTemplate(data.template);
              if (data.domain) setDomain(data.domain);
              if (data.intakeData) setIntakeData(data.intakeData);
              if (data.startResult) setStartResult(data.startResult);

              lastSyncState.current = JSON.stringify({
                step: data.step,
                businessType: data.businessType || businessType,
                businessName: data.businessName || businessName,
                businessCategory: data.businessCategory || businessCategory,
                firstProductName: data.firstProductName || firstProductName,
                firstProductPrice: data.firstProductPrice || firstProductPrice,
                template: data.template || template,
                domain: data.domain || domain,
                intakeData: data.intakeData || intakeData,
                startResult: data.startResult || startResult
              });
            }
          }
        }
      } catch (err) {
        console.error("Failed to load onboarding state", err);
      } finally {
        setIsLoaded(true);
      }
    };

    loadState();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isLoaded]); // Only run once to load

  // Sync state to backend when it changes
  useEffect(() => {
    if (!isLoaded) return;

    const currentStateStr = JSON.stringify({
      step,
      businessType,
      businessName,
      businessCategory,
      firstProductName,
      firstProductPrice,
      template,
      domain,
      intakeData,
      startResult
    });

    // Only sync if state actually changed from last sync
    if (currentStateStr === lastSyncState.current) {
      return;
    }

    const syncState = async () => {
      try {
        const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
        const userId = localStorage.getItem('user_id') || 'test-user';
        await fetch('/api/onboarding/state', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
          body: currentStateStr
        });
        lastSyncState.current = currentStateStr;
      } catch (err) {
        console.error("Failed to sync onboarding state", err);
      }
    };

    const timer = setTimeout(syncState, 1000); // Debounce sync with 1s delay
    return () => clearTimeout(timer);
  }, [isLoaded, step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, intakeData, startResult]);

  const handleNext = () => {
    setError("");
    setStep(step + 1);
  };

  const handleIntakeSubmit = async () => {
    if (!businessCategory.trim()) {
      setError("Please describe what you do.");
      return;
    }
    if (businessCategory.trim().length < 5) {
      setError("Description must be at least 5 characters.");
      return;
    }

    setError("");
    setIsLoading(true);

    const combinedDescription = `Description: ${businessCategory}`;

    setStep(3); // Show shimmer effect immediately

    try {
      const response = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: combinedDescription }),
      });

      if (!response.ok) {
        throw new Error('Failed to process intake');
      }

      const data = await response.json();
      setIntakeData(data);

      // Auto-start onboarding with defaults for instant live transition
      const startRequest = {
        business_type: data.business_type || "Custom",
        company_name: data.business_name || "My Store",
        company_description: "",
        selling_categories: data.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: "Modern",
        domain: "free",
        first_product_name: data.initial_products?.[0]?.name || "Custom Service",
        first_product_price: data.initial_products?.[0]?.price || "100.00",
      };

      const startResponse = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(startRequest),
      });

      if (!startResponse.ok) {
        throw new Error('Failed to start onboarding');
      }

      const startData = await startResponse.json();
      setStartResult(startData);
      setStep(4); // Go directly to activation screen
    } catch (err: any) {
      setError(err.message || 'An error occurred during generation.');
      setStep(2); // Go back to intake on error
    } finally {
      setIsLoading(false);
    }
  };

  // handleStartOnboarding is now merged into handleIntakeSubmit auto-start logic

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-[#000] font-inter">
      <style dangerouslySetInnerHTML={{__html: `
        .glass-container {
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.75), rgba(255, 255, 255, 0.5));
          backdrop-filter: blur(40px) saturate(250%);
          -webkit-backdrop-filter: blur(40px) saturate(250%);
          border: 1px solid rgba(255, 255, 255, 0.6);
          box-shadow:
            0 8px 32px 0 rgba(31, 38, 135, 0.1),
            inset 0 0 0 1px rgba(255, 255, 255, 0.3);
        }
        @media (prefers-color-scheme: dark) {
          .glass-container {
            background: linear-gradient(135deg, rgba(35, 35, 40, 0.8), rgba(22, 22, 26, 0.7));
            backdrop-filter: blur(40px) saturate(250%);
            -webkit-backdrop-filter: blur(40px) saturate(250%);
            border: 1px solid rgba(255, 255, 255, 0.15);
            box-shadow:
              0 8px 32px 0 rgba(0, 0, 0, 0.4),
              inset 0 0 0 1px rgba(255, 255, 255, 0.05);
          }
          .glass-container h1, .glass-container h2, .glass-container .text-gray-900 {
            color: #F5F5F7;
          }
          .glass-container p, .glass-container .text-gray-500 {
            color: #A1A1A6;
          }
          .glass-container input, .glass-container textarea, .glass-container .bg-white\\/80 {
            background: rgba(0, 0, 0, 0.4);
            color: #F5F5F7;
            border-color: rgba(255, 255, 255, 0.15);
            box-shadow: inset 0 2px 4px rgba(0,0,0,0.2);
          }
          .glass-container input:focus, .glass-container textarea:focus {
            box-shadow: 0 0 0 2px rgba(0, 102, 255, 0.5), inset 0 2px 4px rgba(0,0,0,0.2);
          }
        }
        .animate-fade-in {
          animation: fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1);
        }
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }
      `}} />
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative sm:rounded-[16px] overflow-hidden glass-container">
        {/* Header */}
        <div className="w-full p-6 pb-2 pt-12 flex justify-between items-center z-10">
           <h1 className="text-xl font-bold font-outfit text-gray-900">OHC Setup</h1>
           <div className="text-xs font-semibold px-2 py-1 bg-blue-50 text-[#0066FF] rounded-full">
             Step {Math.min(step, 4)} of 4
           </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 p-6 overflow-y-auto z-10 flex flex-col">
          {error && (
            <div className="mb-4 p-3 bg-red-50 border border-red-200 text-red-700 text-sm rounded-xl">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-blue-50 rounded-full flex items-center justify-center mb-6">
                <span className="text-4xl">🚀</span>
              </div>
              <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-4">Launch your business in 5 minutes.</h2>
              <p className="text-gray-500 text-base mb-8 px-4">From zero to live with the power of AI. No setup required.</p>

              <button
                onClick={() => setStep(2)}
                className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[12px] font-bold text-lg shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all"
              >
                Get Started
              </button>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Describe what you do</h2>
              <p className="text-gray-500 text-sm mb-6">Be as brief or detailed as you like.</p>
              <textarea
                value={businessCategory}
                onChange={(e) => setBusinessCategory(e.target.value)}
                placeholder="e.g. I run a custom cake bakery in downtown Austin..."
                className="w-full p-4 rounded-[12px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none transition-all text-lg mb-4 bg-white/40 backdrop-blur-md shadow-sm min-h-[150px] resize-none"
                autoFocus
              />
              <div className="flex gap-3 mt-auto">
                <button
                  onClick={() => setStep(1)}
                  className="px-6 py-4 rounded-[12px] font-bold bg-white/50 backdrop-blur-sm text-gray-700 hover:bg-white/70 shadow-sm border border-white/40 transition-all"
                >
                  Back
                </button>
                <button
                  onClick={() => {
                    setBusinessType("Custom");
                    setBusinessName("My Business");
                    handleIntakeSubmit();
                  }}
                  disabled={isLoading}
                  className="flex-1 bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all disabled:opacity-70 flex justify-center items-center"
                >
                  Next
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-24 h-24 rounded-full flex items-center justify-center mb-6 relative">
                 <div className="absolute inset-0 bg-[#0066FF] rounded-full animate-ping opacity-20"></div>
                 <div className="w-16 h-16 bg-[#eef2ff] rounded-full flex items-center justify-center z-10 shadow-sm border border-[#0066FF]/20">
                   <span className="text-3xl text-[#0066FF] animate-pulse">✨</span>
                 </div>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Building your store...</h2>
              <p className="text-gray-500 text-sm mb-6 max-w-[250px]">Our Marketing Agent is generating your initial site copy and selecting themes based on your description.</p>

              <div className="w-full max-w-[200px] h-2 bg-gray-100 rounded-full overflow-hidden mt-4">
                 <div className="h-full bg-[#0066FF] rounded-full animate-[progress_2s_ease-in-out_infinite] w-1/2"></div>
              </div>
              <style dangerouslySetInnerHTML={{__html: `
                @keyframes progress {
                  0% { transform: translateX(-100%); }
                  100% { transform: translateX(200%); }
                }
              `}} />
            </div>
          )}

          {step === 4 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-green-50 rounded-full flex items-center justify-center mb-6">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h2>
              <p className="text-gray-500 text-sm mb-8 px-4">
                {startResult.message || "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                <button
                  onClick={() => {
                    const domainStr = domain === 'custom' ? 'www.myshop.com' : 'myshop.ohc.store';
                    navigator.clipboard.writeText(`https://${domainStr}`);
                    alert("Link copied to clipboard!");
                  }}
                  className="w-full flex items-center justify-center gap-2 bg-white/70 backdrop-blur-md text-[#1D1D1F] border border-white/50 p-4 rounded-[12px] font-bold shadow-sm hover:bg-white/90 active:scale-[0.98] transition-all"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" /></svg>
                  Share Link
                </button>
                <a
                  href="/dashboard"
                  className="block w-full text-center bg-gradient-to-r from-[#34C759] to-[#2eb350] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all"
                >
                  Go to Dashboard
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
