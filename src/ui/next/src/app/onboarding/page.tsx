"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

export default function OnboardingWizard() {
  const {
    step, setStep,
    chatStep, setChatStep,
    businessDescription, setBusinessDescription,
    businessName, setBusinessName,
    whatYouSell, setWhatYouSell,
    location, setLocation,
    businessType, setBusinessType,
    categories, setCategories,
    websiteTemplate, setWebsiteTemplate,
    domainChoice, setDomainChoice,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
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
        if (data.wizardState.chatStep) setChatStep(data.wizardState.chatStep);
        if (data.wizardState.businessDescription) setBusinessDescription(data.wizardState.businessDescription);
        if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
        if (data.wizardState.whatYouSell) setWhatYouSell(data.wizardState.whatYouSell);
        if (data.wizardState.location) setLocation(data.wizardState.location);
        if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.categories) setCategories(data.wizardState.categories);
        if (data.wizardState.websiteTemplate) setWebsiteTemplate(data.wizardState.websiteTemplate);
        if (data.wizardState.firstProductName) setFirstProductName(data.wizardState.firstProductName);
        if (data.wizardState.firstProductPrice) setFirstProductPrice(data.wizardState.firstProductPrice);
        if (data.wizardState.aiAgents) setAiAgents(data.wizardState.aiAgents);
        if (data.wizardState.aiAutoRespond !== undefined) setAiAutoRespond(data.wizardState.aiAutoRespond);
      }
    })
    .catch(err => console.error('Failed to load onboarding state', err));
  }, []);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded) return;

    // Only save if we are past the initial state
    if (step === 1 && chatStep === 1 && !businessName) return;

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      chatStep,
      businessDescription,
      businessName,
      whatYouSell,
      location,
      businessType,
      categories,
      websiteTemplate,
      firstProductName,
      firstProductPrice,
      aiAgents,
      aiAutoRespond
    };

    const timer = setTimeout(() => {
      fetch('/api/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ wizardState })
      }).catch(err => console.error('Failed to sync onboarding state', err));
    }, 1000); // debounce 1s

    return () => clearTimeout(timer);
  }, [
    step, chatStep, businessDescription, businessName, whatYouSell, location,
    businessType, categories, websiteTemplate, firstProductName, firstProductPrice,
    aiAgents, aiAutoRespond, isLoaded
  ]);

  const handleIntake = async () => {
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}`;

      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: combinedDescription })
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

      setStep(2); // Go to review step
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred processing details');
    } finally {
      setIsLoading(false);
    }
  };

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
          company_name: businessName,
          company_description: businessDescription || whatYouSell,
          selling_categories: categories,
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: domainChoice || 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      setStep(5); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(3); // Go back to last input screen on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8">
      <div id="setup-screen" className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-lg overflow-hidden flex flex-col h-[650px] relative">
        <div className="p-6 flex-1 flex flex-col overflow-y-auto">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-3 rounded-[8px] text-sm">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 h-full animate-fade-in">
              <div className="flex-1 overflow-y-auto pr-2 pb-4 space-y-4">
                {/* Intro Message */}
                <div className="flex items-start gap-3">
                  <div className="w-8 h-8 rounded-full bg-[#0066FF]/10 flex items-center justify-center shrink-0">
                    <span className="text-lg">🤖</span>
                  </div>
                  <div className="bg-white/60 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 p-3 rounded-[16px] rounded-tl-none text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">
                    Hi there! I'm your OHC Advisor. Let's get your business up and running in a few minutes.
                  </div>
                </div>

                {/* Step 1: Business Name */}
                <div className="flex items-start gap-3 animate-fade-in">
                  <div className="w-8 h-8 rounded-full bg-[#0066FF]/10 flex items-center justify-center shrink-0">
                    <span className="text-lg">🤖</span>
                  </div>
                  <div className="bg-white/60 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 p-3 rounded-[16px] rounded-tl-none text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">
                    What's the name of your business?
                  </div>
                </div>

                {chatStep > 1 && (
                  <div className="flex items-end justify-end gap-3 animate-fade-in">
                    <div className="bg-[#0066FF] text-white p-3 rounded-[16px] rounded-tr-none text-sm max-w-[80%]">
                      {businessName}
                    </div>
                  </div>
                )}

                {/* Step 2: What do you sell? */}
                {chatStep >= 2 && (
                  <div className="flex items-start gap-3 animate-fade-in">
                    <div className="w-8 h-8 rounded-full bg-[#0066FF]/10 flex items-center justify-center shrink-0">
                      <span className="text-lg">🤖</span>
                    </div>
                    <div className="bg-white/60 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 p-3 rounded-[16px] rounded-tl-none text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">
                      Great name! What do you sell? (Feel free to be descriptive, like "I bake custom vegan cakes for weddings")
                    </div>
                  </div>
                )}

                {chatStep > 2 && (
                  <div className="flex items-end justify-end gap-3 animate-fade-in">
                    <div className="bg-[#0066FF] text-white p-3 rounded-[16px] rounded-tr-none text-sm max-w-[80%]">
                      {whatYouSell}
                    </div>
                  </div>
                )}

                {/* Step 3: Location */}
                {chatStep >= 3 && (
                  <div className="flex items-start gap-3 animate-fade-in">
                    <div className="w-8 h-8 rounded-full bg-[#0066FF]/10 flex items-center justify-center shrink-0">
                      <span className="text-lg">🤖</span>
                    </div>
                    <div className="bg-white/60 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 p-3 rounded-[16px] rounded-tl-none text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">
                      Awesome. Lastly, where are you located? (This helps with tax and shipping settings)
                    </div>
                  </div>
                )}

                {chatStep > 3 && (
                  <div className="flex items-end justify-end gap-3 animate-fade-in">
                    <div className="bg-[#0066FF] text-white p-3 rounded-[16px] rounded-tr-none text-sm max-w-[80%]">
                      {location}
                    </div>
                  </div>
                )}

                {isLoading && (
                  <div className="flex items-start gap-3 animate-fade-in">
                    <div className="w-8 h-8 rounded-full bg-[#0066FF]/10 flex items-center justify-center shrink-0">
                      <span className="text-lg">🤖</span>
                    </div>
                    <div className="bg-white/60 dark:bg-black/30 backdrop-blur-md border border-white/50 dark:border-white/10 p-3 rounded-[16px] rounded-tl-none text-sm text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center gap-2">
                       <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                       <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                       <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }}></div>
                    </div>
                  </div>
                )}
              </div>

              {/* Input Area */}
              <div className="mt-auto pt-4 border-t border-white/50 dark:border-white/10 flex gap-2">
                {chatStep === 1 && (
                  <>
                    <input
                      type="text"
                      autoFocus
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' && businessName.trim()) setChatStep(2);
                      }}
                      placeholder="e.g. Maya's Custom Cakes"
                      className="flex-1 p-3 rounded-[12px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-sm transition-all"
                    />
                    <button
                      onClick={() => setChatStep(2)}
                      disabled={!businessName.trim()}
                      className="bg-[#0066FF] text-white p-3 rounded-[12px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed w-[50px] flex items-center justify-center"
                    >
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" /></svg>
                    </button>
                  </>
                )}
                {chatStep === 2 && (
                  <>
                    <input
                      type="text"
                      autoFocus
                      value={whatYouSell}
                      onChange={(e) => setWhatYouSell(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' && whatYouSell.trim()) setChatStep(3);
                      }}
                      placeholder="e.g. I bake custom vegan cakes..."
                      className="flex-1 p-3 rounded-[12px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-sm transition-all"
                    />
                    <button
                      onClick={() => setChatStep(3)}
                      disabled={!whatYouSell.trim()}
                      className="bg-[#0066FF] text-white p-3 rounded-[12px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed w-[50px] flex items-center justify-center"
                    >
                       <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" /></svg>
                    </button>
                  </>
                )}
                {chatStep === 3 && (
                  <>
                    <input
                      type="text"
                      autoFocus
                      value={location}
                      onChange={(e) => setLocation(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' && location.trim() && !isLoading) handleIntake();
                      }}
                      placeholder="e.g. Portland, OR"
                      className="flex-1 p-3 rounded-[12px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-sm transition-all"
                    />
                    <button
                      onClick={handleIntake}
                      disabled={!location.trim() || isLoading}
                      className="bg-[#0066FF] text-white p-3 rounded-[12px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50 disabled:cursor-not-allowed w-auto px-4 flex items-center justify-center"
                    >
                      {isLoading ? '...' : 'Generate'}
                    </button>
                  </>
                )}
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
                    autoFocus
                    value={businessName}
                    onChange={(e) => setBusinessName(e.target.value)}
                    className="w-full p-3 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Type</label>
                  <input
                    type="text"
                    value={businessType}
                    onChange={(e) => setBusinessType(e.target.value)}
                    className="w-full p-3 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Categories (Comma separated)</label>
                  <input
                    type="text"
                    value={categories.join(', ')}
                    onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                    className="w-full p-3 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">First Product</label>
                      <input
                        type="text"
                        value={firstProductName}
                        onChange={(e) => setFirstProductName(e.target.value)}
                        className="w-full p-3 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                   </div>
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Price</label>
                      <input
                        type="text"
                        inputMode="decimal"
                        value={firstProductPrice}
                        onChange={(e) => setFirstProductPrice(e.target.value)}
                        className="w-full p-3 rounded-[8px] border border-white/50 dark:border-white/10 focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                   </div>
                </div>
              </div>

              {validationError && <p className="text-red-500 text-sm font-semibold mb-2">{validationError}</p>}
              <div className="mt-auto pt-6">
                <button
                  onClick={() => {
                    if (businessName.trim().length < 3) {
                      setValidationError('Business Name must be at least 3 characters.');
                      return;
                    }
                    setValidationError('');
                    setStep(3);
                  }}
                  disabled={!businessName.trim() || !businessType.trim() || categories.length === 0 || !firstProductName.trim() || !firstProductPrice.trim()}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
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

              <div className="space-y-4 flex-1 overflow-y-auto pr-2 hide-scrollbar">
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Website Template</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
                      <div
                        key={template}
                        onClick={() => setWebsiteTemplate(template)}
                        className={`p-3 rounded-[8px] border cursor-pointer transition-all ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
                      >
                        <div className="font-semibold text-sm">{template}</div>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-2 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Select AI Team</label>
                  <div className="space-y-2">
                    {['Sales Agent', 'Support Agent', 'Marketing Agent'].map(agent => {
                       const isSelected = aiAgents.includes(agent);
                       return (
                         <div
                           key={agent}
                           onClick={() => {
                             if (isSelected) {
                               setAiAgents(aiAgents.filter(a => a !== agent));
                             } else {
                               setAiAgents([...aiAgents, agent]);
                             }
                           }}
                           className={`p-3 rounded-[8px] border cursor-pointer flex items-center justify-between transition-all ${isSelected ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 text-[#1D1D1F] dark:text-white'}`}
                         >
                           <span className="font-semibold text-sm">{agent}</span>
                           <div className={`w-4 h-4 rounded-full border flex items-center justify-center ${isSelected ? 'border-[#0066FF] bg-[#0066FF]' : 'border-gray-400'}`}>
                              {isSelected && <svg className="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                           </div>
                         </div>
                       );
                    })}
                  </div>
                </div>

                <div className="pt-2">
                  <label className="flex items-center justify-between cursor-pointer p-3 rounded-[8px] border border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 text-[#1D1D1F] dark:text-white">
                    <span className="font-semibold text-sm">Allow AI to Auto-Respond</span>
                    <input
                      type="checkbox"
                      className="sr-only"
                      checked={aiAutoRespond}
                      onChange={(e) => setAiAutoRespond(e.target.checked)}
                    />
                    <div className={`w-10 h-6 rounded-full transition-colors ${aiAutoRespond ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-600'} relative`}>
                       <div className={`w-4 h-4 rounded-full bg-white absolute top-1 transition-transform ${aiAutoRespond ? 'translate-x-5' : 'translate-x-1'}`}></div>
                    </div>
                  </label>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
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
