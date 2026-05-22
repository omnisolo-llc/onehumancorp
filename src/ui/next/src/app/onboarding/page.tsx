"use client";

import React from 'react';
import { useOnboardingStore } from './store';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export default function OnboardingWizard() {
  const {
    step, setStep,
    bio, setBio,
    isLoading, setIsLoading,
    error, setError,
    intakeData, setIntakeData,
    startResult, setStartResult
  } = useOnboardingStore();

  const handleIntakeSubmit = async () => {
    if (!bio.trim()) {
      setError("Please describe your business.");
      return;
    }

    setError("");
    setIsLoading(true);

    try {
      const response = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: bio }),
      });

      if (!response.ok) {
        throw new Error('Failed to process intake');
      }

      const data = await response.json();
      setIntakeData(data);
      setStep(2); // Go to review step
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
        business_type: intakeData.business_type || "Retail",
        company_name: intakeData.business_name || "",
        company_description: "", // Removed preferredStyle
        selling_categories: intakeData.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: "modern",
        first_product_name: intakeData.initial_products?.[0]?.name || "Sample Product",
        first_product_price: intakeData.initial_products?.[0]?.price || "10.00",
      };

      const response = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(startRequest),
      });

      if (!response.ok) {
        throw new Error('Failed to start onboarding');
      }

      const data = await response.json();
      setStartResult(data);
      setStep(3); // Go to live step
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
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.4);
        }
        @media (prefers-color-scheme: dark) {
          .glass-container {
            background: rgba(22, 22, 26, 0.7);
            border: 1px solid rgba(255, 255, 255, 0.1);
          }
          .glass-container h1, .glass-container h2, .glass-container .text-gray-900 {
            color: #F5F5F7;
          }
          .glass-container p, .glass-container .text-gray-500 {
            color: #A1A1A6;
          }
          .glass-container input, .glass-container textarea, .glass-container .bg-white\\/80 {
            background: rgba(0, 0, 0, 0.3);
            color: #F5F5F7;
            border-color: rgba(255, 255, 255, 0.2);
          }
        }
      `}} />
      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative sm:rounded-[16px] overflow-hidden glass-container">
        {/* Header */}
        <div className="w-full p-6 pb-2 pt-12 flex justify-between items-center z-10">
           <h1 className="text-xl font-bold font-outfit text-gray-900">OHC Setup</h1>
           <div className="text-xs font-semibold px-2 py-1 bg-blue-50 text-[#0066FF] rounded-full">
             Step {Math.min(step, 3)} of 3
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
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Tell us about your business</h2>
              <p className="text-gray-500 text-sm mb-6">Our AI will instantly build your storefront.</p>
              <textarea
                value={bio}
                onChange={(e) => setBio(e.target.value)}
                placeholder="e.g. I am Maya. I bake custom vegan cakes in Seattle. Maya's Cakes. Prices start at $50."
                className="w-full p-4 rounded-xl border border-gray-200 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] outline-none transition-all text-lg mb-4 bg-white/80 min-h-[150px] resize-none"
                autoFocus
              />
              <button
                onClick={handleIntakeSubmit}
                disabled={isLoading}
                className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md hover:bg-blue-700 active:scale-[0.98] transition-all disabled:opacity-70 flex justify-center items-center"
              >
                {isLoading ? (
                  <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                ) : (
                  "Generate Draft"
                )}
              </button>
            </div>
          )}

          {step === 2 && intakeData && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] rounded-full flex items-center justify-center mb-6 mx-auto">
                <span className="text-3xl text-[#0066FF]">✨</span>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2 text-center">Looks Great!</h2>
              <p className="text-gray-500 text-sm mb-6 text-center">Here is what our AI extracted. Ready to publish?</p>

              <div className="bg-white/80 p-5 rounded-xl border border-gray-100 shadow-sm mb-6 space-y-3">
                <div>
                  <span className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Business Name</span>
                  <div className="font-medium text-gray-900">{intakeData.business_name}</div>
                </div>
                <div>
                  <span className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Type</span>
                  <div className="font-medium text-gray-900">{intakeData.business_type}</div>
                </div>
                <div>
                  <span className="text-xs font-semibold text-gray-400 uppercase tracking-wider">Products</span>
                  <ul className="list-disc pl-4 text-sm text-gray-700 mt-1">
                    {intakeData.initial_products?.map((p: any, i: number) => (
                      <li key={i}>{p.name} - ${p.price}</li>
                    ))}
                  </ul>
                </div>
              </div>

              <div className="flex gap-3 mt-auto">
                <button
                  onClick={() => setStep(1)}
                  className="px-6 py-4 rounded-xl font-bold bg-gray-100 text-gray-600 hover:bg-gray-200 transition-all"
                  disabled={isLoading}
                >
                  Edit
                </button>
                <button
                  onClick={handleStartOnboarding}
                  disabled={isLoading}
                  className="flex-1 bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md hover:bg-[#2eb350] active:scale-[0.98] transition-all disabled:opacity-70 flex justify-center items-center"
                >
                  {isLoading ? (
                    <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                  ) : (
                    "Publish Now"
                  )}
                </button>
              </div>
            </div>
          )}

          {step === 3 && startResult && (
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
                <a
                  href="/dashboard"
                  className="block w-full bg-[#1D1D1F] text-white p-4 rounded-xl font-bold shadow-md hover:bg-black active:scale-[0.98] transition-all"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full bg-white text-[#1D1D1F] border border-gray-200 p-4 rounded-xl font-bold shadow-sm hover:bg-gray-50 active:scale-[0.98] transition-all"
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
