"use client";

import React, { useState } from 'react';

export default function OnboardingWizard() {
  const [step, setStep] = useState(1);
  const [businessName, setBusinessName] = useState("");
  const [businessCategory, setBusinessCategory] = useState("");
  const [businessType, setBusinessType] = useState("");

  // Specific fields
  const [paymentPref, setPaymentPref] = useState("");
  const [services, setServices] = useState("");

  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");
  const [startResult, setStartResult] = useState<any>(null);

  const handleNext = () => {
    if (step === 1 && (!businessName.trim() || !businessCategory.trim())) {
      setError("Please fill out both fields.");
      return;
    }
    setError("");
    setStep(step + 1);
  };

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setError("");

    try {
      const startRequest = {
        business_type: businessType || "Retail",
        company_name: businessName,
        company_description: businessCategory + (services ? ` Services: ${services}` : ""),
        selling_categories: [businessType.toLowerCase()],
        payment_pref: paymentPref || "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: "modern",
        first_product_name: "Sample Product",
        first_product_price: "10.00",
        price_type: "fixed",
        domain_choice: "auto"
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
      setStep(10); // Success Step
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
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What's the name of your business?</h2>
              <input
                type="text"
                value={businessName}
                onChange={(e) => setBusinessName(e.target.value)}
                placeholder="e.g. Maya's Cakes"
                className="w-full p-4 rounded-xl border border-gray-200 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] outline-none transition-all text-lg mb-4 bg-white/80"
                autoFocus
              />
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2 mt-4">Briefly describe your business:</h2>
              <input
                type="text"
                value={businessCategory}
                onChange={(e) => setBusinessCategory(e.target.value)}
                placeholder="e.g. I bake custom vegan cakes."
                className="w-full p-4 rounded-xl border border-gray-200 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] outline-none transition-all text-lg mb-6 bg-white/80"
              />
              <button
                onClick={handleNext}
                className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md hover:bg-blue-700 active:scale-[0.98] transition-all"
              >
                Next
              </button>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What do you sell?</h2>
              <div className="flex flex-col gap-3">
                <button onClick={() => { setBusinessType('Food'); handleNext(); }} className="w-full p-4 rounded-xl border border-gray-200 text-left font-medium hover:border-[#0066FF]">Food / Custom Cakes</button>
                <button onClick={() => { setBusinessType('Services'); handleNext(); }} className="w-full p-4 rounded-xl border border-gray-200 text-left font-medium hover:border-[#0066FF]">Services / Bookings</button>
                <button onClick={() => { setBusinessType('Physical'); handleNext(); }} className="w-full p-4 rounded-xl border border-gray-200 text-left font-medium hover:border-[#0066FF]">Physical Products</button>
                <button onClick={() => { setBusinessType('Subscriptions'); handleNext(); }} className="w-full p-4 rounded-xl border border-gray-200 text-left font-medium hover:border-[#0066FF]">Services & Subscriptions</button>
                <button onClick={() => { setBusinessType('Food Cart'); handleNext(); }} className="w-full p-4 rounded-xl border border-gray-200 text-left font-medium hover:border-[#0066FF]">Food & Beverage (Cart)</button>
              </div>
            </div>
          )}

          {/* Maya (Food) Path */}
          {step === 3 && businessType === 'Food' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">How do you want to get paid?</h2>
              <input type="text" value={paymentPref} onChange={(e) => setPaymentPref(e.target.value)} placeholder="Deposits via Stripe" className="w-full p-4 rounded-xl border border-gray-200 mb-4" />
              <button onClick={() => setStep(4)} className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md">Next</button>
            </div>
          )}
          {step === 4 && businessType === 'Food' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Connect Instagram?</h2>
              <button onClick={() => setStep(5)} className="w-full bg-pink-500 text-white p-4 rounded-xl font-bold shadow-md mb-3">Connect IG</button>
              <button onClick={() => setStep(5)} className="w-full bg-gray-100 text-gray-600 p-4 rounded-xl font-bold shadow-md">Skip</button>
            </div>
          )}
          {step === 5 && businessType === 'Food' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Set Prices & Deposit Rules</h2>
              <input type="text" placeholder="Prices" className="w-full p-4 rounded-xl border border-gray-200 mb-4" />
              <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md disabled:opacity-70 flex justify-center items-center">
                {isLoading ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Store"}
              </button>
            </div>
          )}

          {/* Carlos (Services) Path */}
          {step === 3 && businessType === 'Services' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What services do you offer?</h2>
              <input type="text" placeholder="Plumbing, Painting" value={services} onChange={(e) => setServices(e.target.value)} className="w-full p-4 rounded-xl border border-gray-200 mb-4" />
              <button onClick={() => setStep(4)} className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md">Next</button>
            </div>
          )}
          {step === 4 && businessType === 'Services' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Set Working Hours & Deposits</h2>
              <input type="text" placeholder="Mon-Fri, 20% Deposit" className="w-full p-4 rounded-xl border border-gray-200 mb-4" />
              <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md disabled:opacity-70 flex justify-center items-center">
                {isLoading ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Store"}
              </button>
            </div>
          )}

          {/* Priya (Physical) Path */}
          {step === 3 && businessType === 'Physical' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Upload Inventory</h2>
              <button onClick={() => setStep(4)} className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md">Upload CSV</button>
            </div>
          )}
          {step === 4 && businessType === 'Physical' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Inventory Uploaded</h2>
              <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md disabled:opacity-70 flex justify-center items-center">
                {isLoading ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Store"}
              </button>
            </div>
          )}

          {/* Leo (Subscriptions) Path */}
          {step === 3 && businessType === 'Subscriptions' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Connect Calendar?</h2>
              <button onClick={() => setStep(4)} className="w-full bg-blue-500 text-white p-4 rounded-xl font-bold shadow-md mb-3">Connect Google Calendar</button>
            </div>
          )}
          {step === 4 && businessType === 'Subscriptions' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Calendar Synced</h2>
              <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md disabled:opacity-70 flex justify-center items-center">
                {isLoading ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Store"}
              </button>
            </div>
          )}

          {/* Fatima (Food Cart) Path */}
          {step === 3 && businessType === 'Food Cart' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Take photos of menu</h2>
              <button onClick={() => setStep(4)} className="w-full bg-[#0066FF] text-white p-4 rounded-xl font-bold shadow-md mb-3">Upload Photos</button>
            </div>
          )}
          {step === 4 && businessType === 'Food Cart' && (
            <div className="flex flex-col flex-1 justify-center animate-fade-in">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Menu Ready</h2>
              <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-4 rounded-xl font-bold shadow-md disabled:opacity-70 flex justify-center items-center">
                {isLoading ? <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Store"}
              </button>
            </div>
          )}

          {/* Success */}
          {step === 10 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-green-50 rounded-full flex items-center justify-center mb-6">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h2>
              <div className="w-full space-y-3 mt-auto">
                <a href="/dashboard" className="block w-full bg-[#1D1D1F] text-white p-4 rounded-xl font-bold shadow-md hover:bg-black transition-all">Go to Dashboard</a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
