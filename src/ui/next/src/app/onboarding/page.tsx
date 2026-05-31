"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessType, setBusinessType,
    uploadedPhotos, setUploadedPhotos,
    businessName, setBusinessName,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const [validationError, setValidationError] = useState('');

  // Read state from server on mount
  useEffect(() => {
    setIsLoaded(true);
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    fetch('/api/onboarding/state', {
      headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
    })
    .then(res => res.json())
    .then(data => {
      if (data && data.wizardState) {
        if (data.wizardState.step) setStep(data.wizardState.step);
        if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.uploadedPhotos) setUploadedPhotos(data.wizardState.uploadedPhotos);
        if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
      }
    })
    .catch(err => console.error('Failed to load onboarding state', err));
  }, []);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded) return;
    if (step === 1 && !businessType) return;

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      businessType,
      uploadedPhotos,
      businessName
    };

    const timer = setTimeout(() => {
      fetch('/api/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ wizardState })
      }).catch(err => console.error('Failed to sync onboarding state', err));
    }, 1000);

    return () => clearTimeout(timer);
  }, [step, businessType, uploadedPhotos, businessName, isLoaded]);

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
           business_name: businessName,
           photos: uploadedPhotos
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start storefront generation');
      }

      const startData = await startRes.json();
      setStartResult(startData);

      // Simulate a real-time progress screen
      setTimeout(() => {
        setStep(5);
      }, 2000);
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during launch');
      setStep(3); // Go back to last input screen on error
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center font-inter p-4" style={{ backgroundColor: '#F5F5F7' }}>
      <div className="w-full max-w-md w-full ohc-hybrid-panel relative overflow-hidden flex flex-col">
        {/* Progress Bar */}
        <div className="absolute top-0 left-0 right-0 h-1 bg-gray-200 dark:bg-gray-800">
           <div
             className="h-full bg-[#0066FF] transition-all duration-500 ease-out"
             style={{ width: `${(step / 3) * 100}%` }}
           ></div>
        </div>

        <div className="p-8 flex flex-col min-h-[500px]">
          {error && (
            <div className="bg-red-50 dark:bg-red-900/30 text-red-600 dark:text-red-400 p-3 rounded-[8px] mb-6 text-sm font-semibold border border-red-200 dark:border-red-800 animate-fade-in">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What do you sell?</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Select a business type to get started.
              </p>

              <div className="grid grid-cols-2 gap-4 flex-1">
                {['Food & Beverage', 'Services', 'Retail', 'Creative', 'Digital', 'Other'].map(type => (
                  <button
                    key={type}
                    onClick={() => setBusinessType(type)}
                    className={`p-4 rounded-xl border-2 text-left transition-all ${businessType === type ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-transparent bg-white/50 hover:bg-white/80 dark:bg-black/20 dark:hover:bg-black/40 text-[#1D1D1F] dark:text-white'}`}
                  >
                    <div className="text-2xl mb-2">
                       {type === 'Food & Beverage' && '🍔'}
                       {type === 'Services' && '🔧'}
                       {type === 'Retail' && '🛍️'}
                       {type === 'Creative' && '🎨'}
                       {type === 'Digital' && '💻'}
                       {type === 'Other' && '📦'}
                    </div>
                    <div className="font-semibold">{type}</div>
                  </button>
                ))}
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(2)}
                  disabled={!businessType}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Next
                </button>
              </div>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>

              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Add some photos</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Upload a few photos of your work, or connect Instagram.
              </p>

              <div className="flex-1 flex flex-col items-center justify-center border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl bg-white/50 dark:bg-black/20 p-6">
                 {uploadedPhotos.length > 0 ? (
                    <div className="grid grid-cols-2 gap-2 w-full">
                       {uploadedPhotos.map((photo, i) => (
                          <div key={i} className="aspect-square bg-gray-200 dark:bg-gray-800 rounded-lg flex items-center justify-center text-3xl overflow-hidden">
                             <span role="img" aria-label="uploaded photo">📸</span>
                          </div>
                       ))}
                    </div>
                 ) : (
                    <div className="text-center">
                      <div className="text-4xl mb-4">📸</div>
                      <p className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-4">Tap to upload photos</p>
                      <button
                        onClick={() => setUploadedPhotos([...uploadedPhotos, 'photo1.jpg'])}
                        className="px-4 py-2 bg-gray-100 dark:bg-gray-800 rounded-full text-sm font-semibold hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                      >
                        Browse Files
                      </button>
                    </div>
                 )}
              </div>

              <div className="mt-auto pt-6 flex gap-3">
                <button
                  onClick={() => setStep(3)}
                  className="flex-1 bg-gray-200 dark:bg-gray-800 text-[#1D1D1F] dark:text-white p-4 rounded-[8px] font-bold shadow-sm hover:bg-gray-300 dark:hover:bg-gray-700 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  Skip
                </button>
                <button
                  onClick={() => setStep(3)}
                  disabled={uploadedPhotos.length === 0}
                  className="flex-1 bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
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

              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What's the name of your business?</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Don't worry, you can change this later.
              </p>

              <div className="flex-1">
                <input
                  type="text"
                  placeholder="e.g. Maya's Custom Cakes"
                  value={businessName}
                  onChange={(e) => {
                    setBusinessName(e.target.value);
                    setValidationError('');
                  }}
                  className="w-full bg-white/70 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 rounded-[8px] p-4 text-lg font-medium text-[#1D1D1F] dark:text-[#F5F5F7] placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 transition-all shadow-sm"
                  autoFocus
                />
                {validationError && <p className="text-red-500 text-sm font-semibold mt-2">{validationError}</p>}
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => {
                    if (businessName.trim().length < 3) {
                      setValidationError('Business Name must be at least 3 characters.');
                      return;
                    }
                    setValidationError('');
                    handleStartOnboarding();
                  }}
                  disabled={!businessName.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Generate My Business
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
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Our AI is building your store...</h2>
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
                  className="block w-full bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-[8px] font-bold shadow-md hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full bg-white/70 dark:bg-white/10 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 p-4 rounded-[8px] font-bold shadow-sm hover:bg-white/90 dark:hover:bg-white/20 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
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
