"use client";

import React, { useState } from 'react';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

export default function OnboardingWizard() {
  const [step, setStep] = useState(1);
  const [businessName, setBusinessName] = useState("");
  const [businessCategory, setBusinessCategory] = useState("");


  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");

  const [startResult, setStartResult] = useState<any>(null);

  const handleNext = () => {
    if (step === 1) {
      if (!businessName.trim()) {
        setError("Please enter your business name.");
        return;
      }
      if (!businessCategory.trim()) {
        setError("Please describe what you sell.");
        return;
      }
    }
    setError("");
    setStep(step + 1);
  };

  const handlePublish = async () => {
    setIsLoading(true);
    setError("");

    try {
      const combinedDescription = `Business Name: ${businessName}\nCategory/Products: ${businessCategory}`;

      const intakeResponse = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: combinedDescription }),
      });

      if (!intakeResponse.ok) {
        throw new Error('Failed to process intake');
      }

      const intakeDataResponse = await intakeResponse.json();

      const startRequest = {
        business_type: intakeDataResponse.business_type || "Retail",
        company_name: intakeDataResponse.business_name || businessName,
        company_description: "Generated from Onboarding",
        selling_categories: intakeDataResponse.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: "modern",
        first_product_name: intakeDataResponse.initial_products?.[0]?.name || "Sample Product",
        first_product_price: intakeDataResponse.initial_products?.[0]?.price || "10.00",
        domain_choice: "auto",
        price_type: "fixed"
      };

      const startResponse = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(startRequest),
      });

      if (!startResponse.ok) {
        throw new Error('Failed to start onboarding');
      }

      const data = await startResponse.json();
      setStartResult(data);
      setStep(3);
    } catch (err: any) {
      setError(err.message || 'An error occurred starting your business.');
    } finally {
      setIsLoading(false);
    }
  };
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter">
      <div className="w-full max-w-[375px] h-screen sm:h-[812px] bg-white shadow-2xl flex flex-col relative sm:border sm:border-gray-200 overflow-hidden"
           style={{
             background: 'rgba(255, 255, 255, 0.65)',
             backdropFilter: 'blur(30px) saturate(210%)',
             border: '1px solid rgba(255, 255, 255, 0.4)'
           }}
      >
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
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Tell us about your business.</h2>
              <p className="text-gray-500 text-sm mb-6">Don't worry, you can change this later.</p>

              <label className="text-sm font-semibold text-gray-700 mb-1">Business Name</label>
              <input
                type="text"
                value={businessName}
                onChange={(e) => setBusinessName(e.target.value)}
                placeholder="e.g. Maya's Cakes"
                className="w-full p-4 rounded-xl border border-gray-200 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] outline-none transition-all text-lg mb-4 bg-white/80"
                autoFocus
              />

              <label className="text-sm font-semibold text-gray-700 mb-1 mt-2">What do you sell?</label>
              <textarea
                value={businessCategory}
                onChange={(e) => setBusinessCategory(e.target.value)}
                placeholder="e.g. I bake custom wedding cakes and cupcakes."
                className="w-full p-4 rounded-xl border border-gray-200 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] outline-none transition-all text-lg mb-4 min-h-[100px] bg-white/80 resize-none"
              />

              <button
                onClick={handleNext}
                className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md hover:bg-blue-700 active:scale-[0.98] transition-all mt-4"
              >
                Next
              </button>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Add some photos.</h2>
              <p className="text-gray-500 text-sm mb-6">Showcase your products or services (or connect your Instagram).</p>

              <div className="flex flex-col items-center justify-center border-2 border-dashed border-gray-300 rounded-xl bg-gray-50 h-40 mb-4 cursor-pointer hover:bg-gray-100 transition-colors">
                 <svg className="w-10 h-10 text-gray-400 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" /></svg>
                 <span className="text-gray-600 font-medium">Tap to Upload</span>
              </div>

              <div className="text-center my-2 text-sm text-gray-400">OR</div>

              <button className="w-full p-4 rounded-xl border border-gray-200 font-bold bg-white text-gray-800 shadow-sm flex items-center justify-center gap-2 hover:bg-gray-50 transition-all mb-6">
                <span>📷</span> Connect Instagram
              </button>

              <div className="flex gap-3 mt-auto">
                <button
                  onClick={() => setStep(1)}
                  className="px-6 py-4 rounded-xl font-bold bg-gray-100 text-gray-600 hover:bg-gray-200 transition-all"
                  disabled={isLoading}
                >
                  Back
                </button>
                <button
                  onClick={handlePublish}
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
