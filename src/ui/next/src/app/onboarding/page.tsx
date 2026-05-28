"use client";

import React, { useEffect, useState } from 'react';
import { useOnboardingStore } from './store';

const STEPS = [
  { id: 1, name: 'Intake' },
  { id: 2, name: 'Review' },
  { id: 3, name: 'Brand' },
  { id: 4, name: 'Team' },
  { id: 5, name: 'Launch' }
];

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
    brandTone, setBrandTone,
    selectedAgents, setSelectedAgents,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult,
    isValidBusinessName, isValidProductName, isValidPrice
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    setIsLoaded(true);
    trackStep(step);
  }, []);

  const trackStep = async (stepNumber: number) => {
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      await fetch('/api/onboarding/track', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId },
        body: JSON.stringify({ step: `step_${stepNumber}_reached` })
      });
    } catch (e) {
      // Silent telemetry failure
    }
  };

  useEffect(() => {
    if (isLoaded) {
      trackStep(step);
    }
  }, [step]);

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
      setBusinessName(intakeData.business_name || businessName);
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
    const prevStep = step;
    setStep(5); // Go to loading screen

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
          company_description: businessDescription,
          selling_categories: categories,
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: 'subdomain',
          price_type: 'fixed',
          selected_agents: selectedAgents,
          brand_tone: brandTone
        })
      });

      if (!startRes.ok) {
        throw new Error('Failed to start onboarding');
      }

      const result = await startRes.json();
      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      setStep(6); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(prevStep); // Go back to last input screen on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-2 py-4 sm:px-6 lg:px-8 transition-colors duration-500">
      <div className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[16px] shadow-2xl overflow-hidden flex flex-col h-[680px] relative animate-fade-in">

        {/* Progress Bar */}
        {step < 6 && (
          <div className="px-6 pt-6 flex items-center justify-between gap-1">
            {STEPS.map((s) => (
              <div key={s.id} className="flex-1 flex flex-col gap-1.5">
                <div className={`h-1 rounded-full transition-all duration-500 ${step >= s.id ? 'bg-[#0066FF]' : 'bg-gray-200 dark:bg-white/10'}`} />
                <span className={`text-[10px] font-bold uppercase tracking-wider text-center transition-colors ${step === s.id ? 'text-[#0066FF]' : 'text-gray-400'}`}>
                  {s.name}
                </span>
              </div>
            ))}
          </div>
        )}

        <div className="p-6 flex-1 flex flex-col overflow-y-auto">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-3 rounded-[8px] text-sm animate-fade-in">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              {chatStep === 1 && (
                <div className="flex flex-col flex-1 animate-fade-in">
                  <div className="w-12 h-12 bg-[#0066FF]/10 rounded-[12px] flex items-center justify-center mb-6">
                    <svg className="w-6 h-6 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                  </div>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">What's your business name?</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                    Our AI will instantly generate your storefront, products, and back-office agents.
                  </p>

                  <div className="space-y-4 flex-1">
                    <input
                      id="business-name-input"
                      type="text"
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                      placeholder="e.g. Maya's Custom Cakes"
                      className="ohc-input text-lg"
                      autoFocus
                    />
                  </div>

                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => setChatStep(2)}
                      disabled={!isValidBusinessName()}
                      className="w-full ohc-button ohc-button-primary p-4"
                    >
                      Next
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 2 && (
                <div className="flex flex-col flex-1 animate-fade-in">
                  <button onClick={() => setChatStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 hover:opacity-70 transition-opacity">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">What do you sell?</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                    Tell us a bit about your products or services.
                  </p>

                  <div className="space-y-4 flex-1">
                    <textarea
                      id="what-you-sell-input"
                      value={whatYouSell}
                      onChange={(e) => setWhatYouSell(e.target.value)}
                      placeholder="e.g. I bake custom vegan cakes for weddings and parties..."
                      className="ohc-input h-32 resize-none"
                    />
                  </div>

                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => setChatStep(3)}
                      disabled={!whatYouSell.trim()}
                      className="w-full ohc-button ohc-button-primary p-4"
                    >
                      Next
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 3 && (
                <div className="flex flex-col flex-1 animate-fade-in">
                  <button onClick={() => setChatStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 hover:opacity-70 transition-opacity">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">Where are you located?</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                    This helps us set up your shipping and tax settings.
                  </p>

                  <div className="space-y-4 flex-1">
                    <input
                      id="location-input"
                      type="text"
                      value={location}
                      onChange={(e) => setLocation(e.target.value)}
                      placeholder="e.g. Portland, OR"
                      className="ohc-input text-lg"
                    />
                  </div>

                  <div className="mt-auto pt-6">
                    <button
                      onClick={handleIntake}
                      disabled={!location.trim() || isLoading}
                      className="w-full ohc-button ohc-button-primary p-4"
                    >
                      {isLoading ? (
                        <div className="flex items-center justify-center gap-2">
                           <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                           <span>Analyzing...</span>
                        </div>
                      ) : 'Generate My Business'}
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 hover:opacity-70 transition-opacity">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">Review Details</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Here's what our AI figured out. Feel free to tweak these.
              </p>

              <div className="space-y-4 flex-1 overflow-y-auto pr-1">
                <div>
                  <label htmlFor="review-business-name" className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-1.5 ml-1">Business Name</label>
                  <input
                    id="review-business-name"
                    type="text"
                    value={businessName}
                    onChange={(e) => setBusinessName(e.target.value)}
                    className="ohc-input py-3"
                  />
                </div>
                <div>
                  <label htmlFor="review-business-type" className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-1.5 ml-1">Business Type</label>
                  <input
                    id="review-business-type"
                    type="text"
                    value={businessType}
                    onChange={(e) => setBusinessType(e.target.value)}
                    className="ohc-input py-3"
                  />
                </div>
                <div>
                  <label htmlFor="review-categories" className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-1.5 ml-1">Categories</label>
                  <input
                    id="review-categories"
                    type="text"
                    value={categories.join(', ')}
                    onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                    className="ohc-input py-3"
                  />
                </div>
                <div className="grid grid-cols-2 gap-3">
                   <div>
                      <label htmlFor="review-product-name" className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-1.5 ml-1">First Product</label>
                      <input
                        id="review-product-name"
                        type="text"
                        value={firstProductName}
                        onChange={(e) => setFirstProductName(e.target.value)}
                        className="ohc-input py-3"
                      />
                   </div>
                   <div>
                      <label htmlFor="review-product-price" className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-1.5 ml-1">Price</label>
                      <input
                        id="review-product-price"
                        type="text"
                        value={firstProductPrice}
                        onChange={(e) => setFirstProductPrice(e.target.value)}
                        className="ohc-input py-3"
                      />
                   </div>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(3)}
                  disabled={!isValidBusinessName() || !isValidProductName() || !isValidPrice() || !businessType.trim()}
                  className="w-full ohc-button ohc-button-primary p-4"
                >
                  Continue
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 animate-fade-in">
              <button onClick={() => setStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 hover:opacity-70 transition-opacity">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">Brand Style</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                Pick your storefront vibe and brand personality.
              </p>

              <div className="space-y-6 flex-1 overflow-y-auto pr-1">
                <div>
                  <label className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-3 ml-1">Website Template</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
                      <div
                        key={template}
                        onClick={() => setWebsiteTemplate(template)}
                        className={`p-4 rounded-[12px] border cursor-pointer transition-all duration-200 ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-inner' : 'border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 hover:border-gray-300 dark:hover:border-gray-600 text-[#1D1D1F] dark:text-white'}`}
                      >
                        <div className="font-bold text-sm">{template}</div>
                      </div>
                    ))}
                  </div>
                </div>

                <div>
                  <label className="block text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-widest mb-3 ml-1">Brand Tone</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Professional', 'Friendly', 'Luxury', 'Playful'].map(tone => (
                      <div
                        key={tone}
                        onClick={() => setBrandTone(tone)}
                        className={`p-4 rounded-[12px] border cursor-pointer transition-all duration-200 ${brandTone === tone ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-inner' : 'border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 hover:border-gray-300 dark:hover:border-gray-600 text-[#1D1D1F] dark:text-white'}`}
                      >
                        <div className="font-bold text-sm">{tone}</div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => setStep(4)}
                  className="w-full ohc-button ohc-button-primary p-4"
                >
                  Continue
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
             <div className="flex flex-col flex-1 animate-fade-in">
                <button onClick={() => setStep(3)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 hover:opacity-70 transition-opacity">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" /></svg> Back
                </button>
                <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 leading-tight">Your AI Team</h2>
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-6">
                  Select the AI agents who will help you run your business 24/7.
                </p>

                <div className="space-y-3 flex-1 overflow-y-auto pr-1">
                   {[
                     { id: 'The Manager', desc: 'Handles scheduling & operations', icon: '📋' },
                     { id: 'The Promoter', desc: 'Social media & marketing', icon: '📢' },
                     { id: 'The Ambassador', desc: '24/7 customer support', icon: '🤝' },
                     { id: 'The Accountant', desc: 'Invoicing & bookkeeping', icon: '💰' },
                     { id: 'The Scout', desc: 'SEO & market discovery', icon: '🔍' }
                   ].map(agent => (
                     <div
                        key={agent.id}
                        onClick={() => {
                          if (selectedAgents.includes(agent.id)) {
                            setSelectedAgents(selectedAgents.filter(id => id !== agent.id));
                          } else {
                            setSelectedAgents([...selectedAgents, agent.id]);
                          }
                        }}
                        className={`p-4 rounded-[16px] border cursor-pointer transition-all duration-200 flex items-center gap-4 ${selectedAgents.includes(agent.id) ? 'border-[#0066FF] bg-[#0066FF]/10 shadow-inner' : 'border-white/50 dark:border-white/10 bg-white/60 dark:bg-black/30 hover:bg-white/80 dark:hover:bg-black/50'}`}
                     >
                        <div className="text-2xl">{agent.icon}</div>
                        <div className="flex-1">
                           <div className={`font-bold text-sm ${selectedAgents.includes(agent.id) ? 'text-[#0066FF]' : 'text-[#1D1D1F] dark:text-[#F5F5F7]'}`}>{agent.id}</div>
                           <div className="text-xs text-gray-500 dark:text-gray-400">{agent.desc}</div>
                        </div>
                        <div className={`w-5 h-5 rounded-full border-2 flex items-center justify-center transition-all ${selectedAgents.includes(agent.id) ? 'bg-[#0066FF] border-[#0066FF]' : 'border-gray-300 dark:border-white/20'}`}>
                           {selectedAgents.includes(agent.id) && <svg className="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={4} d="M5 13l4 4L19 7" /></svg>}
                        </div>
                     </div>
                   ))}
                </div>

                <div className="mt-auto pt-6">
                  <button
                    onClick={handleStartOnboarding}
                    disabled={selectedAgents.length === 0}
                    className="w-full ohc-button ohc-button-primary p-4"
                  >
                    Launch My Business
                  </button>
                </div>
             </div>
          )}

          {step === 5 && (
             <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-[6px] border-[#0066FF]/10 rounded-full" />
                 <div className="absolute inset-0 border-[6px] border-[#0066FF] rounded-full border-t-transparent animate-spin" />
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
               <div className="space-y-3">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Designing your {websiteTemplate} storefront</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Onboarding your AI {brandTone} agents</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Setting up {businessName}</p>
               </div>
             </div>
          )}

          {step === 6 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
              <div className="w-20 h-20 bg-[#34C759]/10 rounded-full flex items-center justify-center mb-6 shadow-sm">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">You're Live!</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
                {startResult.message || "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                <div className="p-4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border border-white/50 dark:border-white/10 flex flex-col items-center mb-6">
                   <p className="text-[10px] text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-widest mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-bold text-lg">{businessName.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.ohc.store</span>
                   </div>
                </div>

                <a
                  href="/dashboard"
                  className="block w-full ohc-button ohc-button-primary p-4 text-center"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full ohc-button ohc-button-secondary p-4 text-center"
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
