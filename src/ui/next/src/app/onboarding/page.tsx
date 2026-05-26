"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

const SelectionCard = ({ title, icon, selected, onClick }: { title: string, icon: string, selected: boolean, onClick: () => void }) => (
  <button
    onClick={onClick}
    className={`flex flex-col items-center justify-center p-4 rounded-[16px] transition-all duration-200 ${
      selected
        ? 'bg-[#0066FF] text-white shadow-lg scale-[1.02]'
        : 'bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 border border-white/40 dark:border-white/10'
    }`}
  >
    <span className="text-2xl mb-2">{icon}</span>
    <span className="text-xs font-bold font-outfit uppercase tracking-wider">{title}</span>
  </button>
);

const AIProgressBar = () => {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setProgress((prev) => (prev < 95 ? prev + (95 - prev) * 0.1 : prev));
    }, 200);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="w-full space-y-4 animate-in">
      <div className="flex justify-between items-end">
        <div>
          <h2 className="text-2xl font-bold font-outfit">AI is building your business</h2>
          <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Hang tight, this usually takes under 30 seconds.</p>
        </div>
        <span className="font-bold text-[#0066FF]">{Math.round(progress)}%</span>
      </div>
      <div className="h-2 w-full bg-white/20 dark:bg-white/5 rounded-full overflow-hidden border border-white/10">
        <div
          className="h-full bg-gradient-to-r from-[#0066FF] to-[#00C24B] transition-all duration-500 ease-out"
          style={{ width: `${progress}%` }}
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        {[
          { label: 'Analyzing niche', active: progress > 10 },
          { label: 'Generating products', active: progress > 40 },
          { label: 'Designing theme', active: progress > 70 },
          { label: 'Finalizing setup', active: progress > 90 },
        ].map((step, i) => (
          <div key={i} className={`flex items-center gap-2 text-xs font-medium transition-opacity duration-300 ${step.active ? 'opacity-100' : 'opacity-30'}`}>
            <div className={`w-1.5 h-1.5 rounded-full ${step.active ? 'bg-[#00C24B]' : 'bg-gray-400'}`} />
            {step.label}
          </div>
        ))}
      </div>
    </div>
  );
};

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
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    if (isLoaded) return;
    const loadState = async () => {
      try {
        const tenantId = localStorage.getItem('tenant_id') || 'storefront';
        const userId = localStorage.getItem('user_id') || 'test-user';
        const res = await fetch('/api/onboarding/state', {
          headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
        });
        if (res.ok) {
          const data = await res.json();
          if (data && Object.keys(data).length > 0 && (data.step || 0) >= step) {
            setStep(data.step);
            if (data.businessType !== undefined) setBusinessType(data.businessType);
            if (data.businessName !== undefined) setBusinessName(data.businessName);
            if (data.businessCategory !== undefined) setBusinessCategory(data.businessCategory);
            if (data.firstProductName !== undefined) setFirstProductName(data.firstProductName);
            if (data.firstProductPrice !== undefined) setFirstProductPrice(data.firstProductPrice);
            if (data.template !== undefined) setTemplate(data.template);
            if (data.domain !== undefined) setDomain(data.domain);
            if (data.intakeData !== undefined) setIntakeData(data.intakeData);
            if (data.startResult !== undefined) setStartResult(data.startResult);
          }
        }
      } catch (err) {
        console.error("Failed to load onboarding state", err);
      } finally {
        setIsLoaded(true);
      }
    };
    loadState();
  }, [isLoaded, setStep, setBusinessType, setBusinessName, setBusinessCategory, setFirstProductName, setFirstProductPrice, setTemplate, setDomain, setIntakeData, setStartResult, step]);

  useEffect(() => {
    if (isLoaded && !lastSyncState.current) {
      lastSyncState.current = JSON.stringify({ step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, intakeData, startResult });
    }
  }, [isLoaded, step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, intakeData, startResult]);

  useEffect(() => {
    if (!isLoaded) return;
    const currentState = { step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, intakeData, startResult };
    const currentStateStr = JSON.stringify(currentState);
    if (currentStateStr === lastSyncState.current) return;

    const syncState = async () => {
      try {
        const tenantId = localStorage.getItem('tenant_id') || 'storefront';
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

    const timer = setTimeout(syncState, 1000);
    return () => clearTimeout(timer);
  }, [isLoaded, step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, intakeData, startResult]);

  const handleNext = () => {
    if (step === 1 && (!businessType.trim() || businessType.length < 3)) {
      setError(businessType ? "Please enter at least 3 characters." : "Please describe what you sell.");
      return;
    }
    if (step === 2 && (!businessName.trim() || businessName.length < 3)) {
      setError(businessName ? "Business name must be at least 3 characters." : "Please enter your business name.");
      return;
    }
    setError("");
    setStep(step + 1);
  };

  const handleIntakeSubmit = async (overrideCategory?: string) => {
    const categoryToUse = typeof overrideCategory === 'string' ? overrideCategory : businessCategory;
    if (!categoryToUse.trim() || categoryToUse.length < 5) {
      setError(categoryToUse ? "Niche description must be at least 5 characters." : "Please describe your niche.");
      return;
    }
    if (typeof overrideCategory === 'string') setBusinessCategory(overrideCategory);
    setError("");
    setIsLoading(true);

    try {
      const [response] = await Promise.all([
        fetch('/api/onboarding/intake', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ description: `Type: ${businessType}\nName: ${businessName}\nNiche: ${categoryToUse}` }),
        }),
        new Promise(resolve => setTimeout(resolve, 2000)) // Minimum 2s for "Premium" feel
      ]);

      if (!response.ok) throw new Error('Failed to process intake');
      setIntakeData(await response.json());
      setStep(4);
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
      const response = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          business_type: intakeData.business_type || businessType,
          company_name: intakeData.business_name || businessName,
          company_description: "",
          selling_categories: intakeData.categories || [],
          payment_pref: "stripe",
          admin_email: "admin@example.com",
          admin_name: "Admin",
          admin_password: "password123",
          website_template: template,
          domain: domain,
          first_product_name: firstProductName || intakeData.initial_products?.[0]?.name,
          first_product_price: firstProductPrice || intakeData.initial_products?.[0]?.price,
        }),
      });
      if (!response.ok) throw new Error('Failed to start onboarding');
      setStartResult(await response.json());
      setStep(5);
    } catch (err: any) {
      setError(err.message || 'An error occurred starting your business.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div id="setup-screen" className="flex items-center justify-center min-h-screen bg-[#F5F5F7] dark:bg-black p-4">
      <div className="w-full max-w-[375px] h-[700px] glass-card flex flex-col overflow-hidden animate-in">
        <div className="p-8 pb-4 flex justify-between items-center">
          <h1 className="text-xl font-bold font-outfit uppercase tracking-tight">OHC Setup</h1>
          <div className="px-3 py-1 bg-[#0066FF]/10 text-[#0066FF] text-[10px] font-black rounded-full uppercase tracking-widest">
            Step {Math.min(step, 4)} / 4
          </div>
        </div>

        <div className="flex-1 px-8 py-4 overflow-y-auto custom-scrollbar">
          {error && (
            <div className="mb-6 p-4 bg-[#FF3B30]/10 border border-[#FF3B30]/20 text-[#FF3B30] text-xs font-bold rounded-[8px] animate-in">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="space-y-6 animate-in">
              <div>
                <h2 className="text-2xl font-bold font-outfit mb-1">What do you do?</h2>
                <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Tell us what you sell or provide.</p>
              </div>
              <input
                type="text"
                value={businessType}
                onChange={(e) => setBusinessType(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleNext()}
                placeholder="e.g. Handmade Jewellery"
                className="w-full p-4 glass-control outline-none focus:ring-2 focus:ring-[#0066FF]/30 text-lg font-medium"
                autoFocus
              />
              <div className="grid grid-cols-2 gap-3">
                {[
                  { id: 'Store', icon: '🛍️' },
                  { id: 'Service', icon: '🛠️' },
                  { id: 'Food', icon: '🍳' },
                  { id: 'Creative', icon: '🎨' }
                ].map((t) => (
                  <SelectionCard
                    key={t.id}
                    title={t.id}
                    icon={t.icon}
                    selected={businessType === t.id}
                    onClick={() => { setBusinessType(t.id); setStep(2); }}
                  />
                ))}
              </div>
              <button onClick={handleNext} className="w-full p-4 bg-[#0066FF] text-white rounded-[8px] font-black uppercase tracking-widest text-sm shadow-lg hover:bg-[#0052cc] transition-all">
                Continue
              </button>
            </div>
          )}

          {step === 2 && (
            <div className="space-y-6 animate-in">
              <div>
                <h2 className="text-2xl font-bold font-outfit mb-1">Business Name</h2>
                <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Choose a name for your brand.</p>
              </div>
              <input
                type="text"
                value={businessName}
                onChange={(e) => setBusinessName(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleNext()}
                placeholder="e.g. Luna Crafts"
                className="w-full p-4 glass-control outline-none focus:ring-2 focus:ring-[#0066FF]/30 text-lg font-medium"
                autoFocus
              />
              <div className="flex gap-3">
                <button onClick={() => setStep(1)} className="px-6 p-4 glass-control font-bold text-sm">Back</button>
                <button onClick={handleNext} className="flex-1 p-4 bg-[#0066FF] text-white rounded-[8px] font-black uppercase tracking-widest text-sm shadow-lg">Next</button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="space-y-6 animate-in h-full flex flex-col justify-center">
              {isLoading ? (
                <AIProgressBar />
              ) : (
                <>
                  <div>
                    <h2 className="text-2xl font-bold font-outfit mb-1">Your Niche</h2>
                    <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Describe what makes you unique.</p>
                  </div>
                  <textarea
                    value={businessCategory}
                    onChange={(e) => setBusinessCategory(e.target.value)}
                    placeholder="e.g. I create sustainable, hand-poured soy wax candles with essential oils."
                    className="w-full h-32 p-4 glass-control outline-none focus:ring-2 focus:ring-[#0066FF]/30 text-base font-medium resize-none"
                    autoFocus
                  />
                  <div className="flex gap-3">
                    <button onClick={() => setStep(2)} className="px-6 p-4 glass-control font-bold text-sm">Back</button>
                    <button onClick={() => handleIntakeSubmit()} className="flex-1 p-4 bg-[#0066FF] text-white rounded-[8px] font-black uppercase tracking-widest text-sm shadow-lg">Generate</button>
                  </div>
                </>
              )}
            </div>
          )}

          {step === 4 && intakeData && (
            <div className="space-y-6 animate-in pb-8">
              <div className="text-center">
                <div className="w-12 h-12 bg-[#00C24B]/10 rounded-full flex items-center justify-center mx-auto mb-3">
                  <span className="text-xl">✨</span>
                </div>
                <h2 className="text-2xl font-bold font-outfit">Ready to Launch</h2>
                <p className="text-xs text-gray-500 dark:text-[#A1A1A6]">We've drafted your business setup.</p>
              </div>

              <div className="space-y-4">
                <div className="p-5 glass-control space-y-4">
                  <h3 className="text-xs font-black uppercase tracking-widest text-[#0066FF]">Product Draft</h3>
                  <div className="space-y-3">
                    <label className="sr-only" htmlFor="product-name">Product Name</label>
                    <input
                      id="product-name"
                      type="text"
                      value={firstProductName || (intakeData.initial_products?.[0]?.name || '')}
                      onChange={(e) => setFirstProductName(e.target.value)}
                      className="w-full p-3 bg-white/20 dark:bg-black/20 border border-white/20 rounded-[8px] text-sm font-bold"
                      placeholder="Product Name"
                    />
                    <div className="relative">
                      <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 text-xs font-bold">$</span>
                      <label className="sr-only" htmlFor="product-price">Product Price</label>
                      <input
                        id="product-price"
                        type="text"
                        value={firstProductPrice || (intakeData.initial_products?.[0]?.price || '')}
                        onChange={(e) => setFirstProductPrice(e.target.value)}
                        className="w-full p-3 pl-7 bg-white/20 dark:bg-black/20 border border-white/20 rounded-[8px] text-sm font-bold"
                        placeholder="0.00"
                      />
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-3">
                  {['Modern', 'Minimal'].map((t) => (
                    <button
                      key={t}
                      onClick={() => setTemplate(t)}
                      className={`p-4 rounded-[12px] border transition-all ${
                        template === t ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] font-bold' : 'border-white/20 text-gray-500'
                      } text-xs uppercase tracking-widest`}
                    >
                      {t}
                    </button>
                  ))}
                </div>

                <div className="space-y-2">
                  {['free', 'custom'].map((d) => (
                    <button
                      key={d}
                      onClick={() => setDomain(d)}
                      className={`w-full p-4 rounded-[12px] border flex justify-between items-center transition-all ${
                        domain === d ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] font-bold' : 'border-white/20 text-gray-500'
                      } text-xs uppercase tracking-widest`}
                    >
                      {d === 'free' ? 'OHC Domain' : 'Custom Domain'}
                      <div className={`w-2 h-2 rounded-full ${domain === d ? 'bg-[#0066FF]' : 'bg-gray-300'}`} />
                    </button>
                  ))}
                </div>
              </div>

              <div className="flex gap-3">
                <button onClick={() => setStep(3)} className="px-6 p-4 glass-control font-bold text-sm">Edit</button>
                <button onClick={handleStartOnboarding} disabled={isLoading} className="flex-1 p-4 bg-[#00C24B] text-white rounded-[8px] font-black uppercase tracking-widest text-sm shadow-lg">
                  {isLoading ? '...' : 'Publish'}
                </button>
              </div>
            </div>
          )}

          {step === 5 && startResult && (
            <div className="h-full flex flex-col items-center justify-center text-center animate-in space-y-6">
              <div className="w-20 h-20 bg-[#00C24B]/10 rounded-full flex items-center justify-center animate-bounce">
                <svg className="w-10 h-10 text-[#00C24B]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <div>
                <h2 className="text-3xl font-bold font-outfit">You're Live!</h2>
                <p className="text-sm text-gray-500 dark:text-[#A1A1A6] mt-2">Your business is now open to the world.</p>
              </div>
              <div className="w-full space-y-3">
                <a href="/dashboard" className="block w-full p-4 bg-[#1D1D1F] dark:bg-white dark:text-black text-white rounded-[8px] font-black uppercase tracking-widest text-sm">Dashboard</a>
                <a href="/builder" className="block w-full p-4 glass-control font-black uppercase tracking-widest text-sm">Preview Store</a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
