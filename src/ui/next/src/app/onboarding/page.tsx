"use client";

import React, { useEffect, useRef } from 'react';
import { useOnboardingStore } from './store';
import Step1 from './Step1';
import Step2 from './Step2';
import Step3 from './Step3';
import Step4 from './Step4';
import Step5 from './Step5';

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
    if (step === 1) {
      if (!businessType.trim()) {
        setError("Please describe what you sell.");
        return;
      }
      if (businessType.trim().length < 3) {
        setError("Please enter at least 3 characters.");
        return;
      }
    }
    if (step === 2) {
      if (!businessName.trim()) {
        setError("Please enter your business name.");
        return;
      }
      if (businessName.trim().length < 3) {
        setError("Business name must be at least 3 characters.");
        return;
      }
    }
    if (step === 3) {
      if (!businessCategory.trim()) {
        setError("Please describe your niche.");
        return;
      }
      if (businessCategory.trim().length < 5) {
        setError("Niche description must be at least 5 characters.");
        return;
      }
    }
    setError("");
    setStep(step + 1);
  };

  const handleIntakeSubmit = async () => {
    if (!businessCategory.trim()) {
      setError("Please describe your niche.");
      return;
    }
    if (businessCategory.trim().length < 5) {
      setError("Niche description must be at least 5 characters.");
      return;
    }

    setError("");
    setIsLoading(true);

    const combinedDescription = `Business Type: ${businessType}\nBusiness Name: ${businessName}\nCategory/Products: ${businessCategory}`;

    try {
      const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';
      const response = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ description: combinedDescription }),
      });

      if (!response.ok) {
        throw new Error('Failed to process intake');
      }

      const data = await response.json();
      setIntakeData(data);
      setStep(4); // Go to review step
    } catch (err: any) {
      setError(err.message || 'An error occurred during intake.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setError("");

    try {
      const startRequest = {
        business_type: intakeData.business_type || businessType || "Retail",
        company_name: intakeData.business_name || businessName,
        company_description: "", // Removed preferredStyle
        selling_categories: intakeData.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: template,
        domain: domain,
        first_product_name: firstProductName || intakeData.initial_products?.[0]?.name || "Sample Product",
        first_product_price: firstProductPrice || intakeData.initial_products?.[0]?.price || "10.00",
      };

      const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';
      const response = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify(startRequest),
      });

      if (!response.ok) {
        throw new Error('Failed to start onboarding');
      }

      const data = await response.json();
      setStartResult(data);
      setStep(5); // Go to live step
    } catch (err: any) {
      setError(err.message || 'An error occurred starting your business.');
    } finally {
      setIsLoading(false);
    }
  };

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
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative sm:rounded-[16px] overflow-hidden glass-container backdrop-blur-xl bg-white/20 border border-white/30">
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
            <Step1
              businessType={businessType}
              setBusinessType={setBusinessType}
              handleNext={handleNext}
            />
          )}

          {step === 2 && (
            <Step2
              businessName={businessName}
              setBusinessName={setBusinessName}
              handleNext={handleNext}
              setStep={setStep}
            />
          )}

          {step === 3 && (
            <Step3
              businessCategory={businessCategory}
              setBusinessCategory={setBusinessCategory}
              handleIntakeSubmit={handleIntakeSubmit}
              setStep={setStep}
              isLoading={isLoading}
            />
          )}

          {step === 4 && intakeData && (
            <Step4
              intakeData={intakeData}
              firstProductName={firstProductName}
              setFirstProductName={setFirstProductName}
              firstProductPrice={firstProductPrice}
              setFirstProductPrice={setFirstProductPrice}
              template={template}
              setTemplate={setTemplate}
              domain={domain}
              setDomain={setDomain}
              setStep={setStep}
              handleStartOnboarding={handleStartOnboarding}
              isLoading={isLoading}
            />
          )}

          {step === 5 && startResult && (
            <Step5
              startResult={startResult}
            />
          )}
        </div>
      </div>
    </div>
  );
}
