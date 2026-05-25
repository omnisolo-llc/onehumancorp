"use client";

import React, { useEffect, useRef } from 'react';
import { useOnboardingStore } from './store';

// OHC Premium Design Standards
const AGENT_TEAM = [
  { id: 'The Manager', name: 'Operations', role: 'The Manager', icon: '📋', desc: 'Handles order management and logistics.' },
  { id: 'The Promoter', name: 'Marketing', role: 'The Promoter', icon: '📢', desc: 'Crafts social posts and advertising.' },
  { id: 'The Salesperson', name: 'Sales', role: 'The Salesperson', icon: '💰', desc: 'Generates leads and closes deals.' },
  { id: 'The Ambassador', name: 'Support', role: 'The Ambassador', icon: '🤝', desc: 'Answers customer questions 24/7.' },
  { id: 'The Accountant', name: 'Finance', role: 'The Accountant', icon: '📊', desc: 'Tracks revenue and expenses.' },
  { id: 'The Protector', name: 'Legal', role: 'The Protector', icon: '⚖️', desc: 'Ensures compliance and safety.' },
  { id: 'The Advisor', name: 'Strategy', role: 'The Advisor', icon: '🧠', desc: 'Provides business growth insights.' },
];

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
    selectedAgents, setSelectedAgents, toggleAgent,
    isLoading, setIsLoading,
    error, setError,
    intakeData, setIntakeData,
    startResult, setStartResult
  } = useOnboardingStore();

  const lastSyncState = useRef("");
  const [isLoaded, setIsLoaded] = React.useState(false);

  // Load state from backend
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
          if (data && Object.keys(data).length > 0) {
            if (data.step && data.step >= step) {
              setStep(data.step);
              if (data.businessType) setBusinessType(data.businessType);
              if (data.businessName) setBusinessName(data.businessName);
              if (data.businessCategory) setBusinessCategory(data.businessCategory);
              if (data.firstProductName) setFirstProductName(data.firstProductName);
              if (data.firstProductPrice) setFirstProductPrice(data.firstProductPrice);
              if (data.template) setTemplate(data.template);
              if (data.domain) setDomain(data.domain);
              if (data.selectedAgents) setSelectedAgents(data.selectedAgents);
              if (data.intakeData) setIntakeData(data.intakeData);
              if (data.startResult) setStartResult(data.startResult);
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
  }, [isLoaded]);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded) return;
    const currentState = { step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, selectedAgents, intakeData, startResult };
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
  }, [isLoaded, step, businessType, businessName, businessCategory, firstProductName, firstProductPrice, template, domain, selectedAgents, intakeData, startResult]);

  const handleNext = () => {
    if (step === 1 && (!businessType.trim() || businessType.trim().length < 3)) {
      setError("Please describe what you sell (at least 3 characters).");
      return;
    }
    if (step === 2 && (!businessName.trim() || businessName.trim().length < 3)) {
      setError("Business name must be at least 3 characters.");
      return;
    }
    setError("");
    setStep(step + 1);
  };

  const handleIntakeSubmit = async () => {
    if (!businessCategory.trim() || businessCategory.trim().length < 5) {
      setError("Niche description must be at least 5 characters.");
      return;
    }
    setError("");
    setIsLoading(true);
    try {
      const response = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: `Type: ${businessType}\nName: ${businessName}\nNiche: ${businessCategory}` }),
      });
      if (!response.ok) throw new Error('Failed to process intake');
      const data = await response.json();
      setIntakeData(data);
      setStep(4);
    } catch (err: any) {
      setError(err.message || 'An error occurred.');
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
          business_type: intakeData?.business_type || businessType,
          company_name: intakeData?.business_name || businessName,
          selling_categories: intakeData?.categories || [],
          admin_email: "admin@example.com",
          admin_name: "Admin",
          admin_password: "password123",
          website_template: template,
          first_product_name: firstProductName || intakeData?.initial_products?.[0]?.name || "Sample Product",
          first_product_price: firstProductPrice || intakeData?.initial_products?.[0]?.price || "10.00",
          selected_agents: selectedAgents
        }),
      });
      if (!response.ok) throw new Error('Failed to start onboarding');
      const data = await response.json();
      setStartResult(data);
      setStep(7);
    } catch (err: any) {
      setError(err.message || 'An error occurred.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#F0F0F2] dark:bg-[#000] font-inter text-[#1D1D1F] dark:text-[#F5F5F7]">
      <style dangerouslySetInnerHTML={{__html: `
        .glass-container {
          background: rgba(255, 255, 255, 0.8);
          backdrop-filter: blur(40px) saturate(210%);
          -webkit-backdrop-filter: blur(40px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.5);
          box-shadow: 0 20px 50px rgba(0, 0, 0, 0.1);
        }
        @media (prefers-color-scheme: dark) {
          .glass-container {
            background: rgba(22, 22, 26, 0.8);
            border: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
          }
        }
        .animate-in { animation: fadeIn 300ms ease-out forwards; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
      `}} />

      <div className="w-full max-w-[375px] mx-auto min-h-[100dvh] sm:min-h-[812px] flex flex-col relative sm:rounded-[32px] overflow-hidden glass-container">
        {/* Header */}
        <div className="w-full px-8 pt-16 pb-6 flex justify-between items-center z-10">
           <h1 className="text-xl font-bold font-outfit text-zinc-900 dark:text-zinc-50">OHC Setup</h1>
           <div className="flex gap-1.5">
             {[1,2,3,4,5,6].map(s => (
               <div key={s} className={`h-1.5 w-4 rounded-full transition-all duration-500 ${s <= Math.min(step, 6) ? 'bg-[#0066FF] shadow-[0_0_8px_rgba(0,102,255,0.5)]' : 'bg-zinc-200 dark:bg-zinc-800'}`} />
             ))}
           </div>
        </div>

        {/* Content */}
        <div className="flex-1 px-8 pb-8 flex flex-col z-10 overflow-y-auto">
          {error && <div className="mb-6 p-4 bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-400 text-xs font-bold rounded-2xl animate-in">{error}</div>}

          {step === 1 && (
            <div className="flex flex-col flex-1 justify-center animate-in">
              <h2 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-zinc-900 dark:text-zinc-50">What do you do?</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8 leading-relaxed">Tell us what you sell or the services you provide in plain language.</p>
              <textarea
                value={businessType}
                onChange={(e) => setBusinessType(e.target.value)}
                placeholder="e.g. I sell organic sourdough bread"
                className="w-full p-5 rounded-2xl bg-zinc-100/50 dark:bg-zinc-900/50 border-2 border-transparent focus:border-[#0066FF] focus:bg-white dark:focus:bg-zinc-900 outline-none transition-all text-lg mb-8 h-48 resize-none shadow-inner text-zinc-900 dark:text-zinc-50"
                autoFocus
              />
              <button onClick={handleNext} className="w-full bg-[#0066FF] text-white p-5 rounded-2xl font-bold shadow-xl shadow-blue-500/25 hover:scale-[1.02] active:scale-[0.98] transition-all text-lg">Next</button>
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-in">
              <h2 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-zinc-900 dark:text-zinc-50">Business Name</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8 leading-relaxed">What's the name of your brand? You can change this later.</p>
              <input
                type="text"
                value={businessName}
                onChange={(e) => setBusinessName(e.target.value)}
                placeholder="e.g. Golden Crust Bakery"
                className="w-full p-5 rounded-2xl bg-zinc-100/50 dark:bg-zinc-900/50 border-2 border-transparent focus:border-[#0066FF] focus:bg-white dark:focus:bg-zinc-900 outline-none transition-all text-lg mb-8 shadow-inner text-zinc-900 dark:text-zinc-50"
                autoFocus
              />
              <div className="flex gap-4">
                <button onClick={() => setStep(1)} className="px-8 py-5 rounded-2xl font-bold bg-zinc-100 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 transition-all">Back</button>
                <button onClick={handleNext} className="flex-1 bg-[#0066FF] text-white p-5 rounded-2xl font-bold shadow-xl shadow-blue-500/25 hover:scale-[1.02] active:scale-[0.98] transition-all text-lg">Next</button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 justify-center animate-in">
              <h2 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-zinc-900 dark:text-zinc-50">Your Niche</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8 leading-relaxed">Who are your customers and what makes you unique?</p>
              <input
                type="text"
                value={businessCategory}
                onChange={(e) => setBusinessCategory(e.target.value)}
                placeholder="e.g. Local foodies looking for healthy bread"
                className="w-full p-5 rounded-2xl bg-zinc-100/50 dark:bg-zinc-900/50 border-2 border-transparent focus:border-[#0066FF] focus:bg-white dark:focus:bg-zinc-900 outline-none transition-all text-lg mb-8 shadow-inner text-zinc-900 dark:text-zinc-50"
                autoFocus
              />
              <div className="flex gap-4">
                <button onClick={() => setStep(2)} className="px-8 py-5 rounded-2xl font-bold bg-zinc-100 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 transition-all">Back</button>
                <button onClick={handleIntakeSubmit} disabled={isLoading} className="flex-1 bg-[#0066FF] text-white p-5 rounded-2xl font-bold shadow-xl shadow-blue-500/25 disabled:opacity-50 flex justify-center items-center text-lg">
                  {isLoading ? <span className="w-6 h-6 border-3 border-white/30 border-t-white rounded-full animate-spin"></span> : "Generate Draft"}
                </button>
              </div>
            </div>
          )}

          {step === 4 && intakeData && (
            <div className="flex flex-col flex-1 animate-in">
              <h2 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-zinc-900 dark:text-zinc-50">Review Setup</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8">AI suggested these details based on your input.</p>

              <div className="space-y-5 mb-10">
                <div className="p-5 rounded-[24px] bg-zinc-50 dark:bg-zinc-900/80 border border-zinc-200 dark:border-zinc-800 shadow-sm">
                  <label className="text-[10px] font-black uppercase tracking-[0.15em] text-zinc-400 mb-3 block">First Product</label>
                  <div className="flex gap-4 items-center">
                    <input value={firstProductName || intakeData.initial_products?.[0]?.name || ""} onChange={(e) => setFirstProductName(e.target.value)} className="flex-1 bg-transparent border-none p-0 focus:ring-0 text-base font-bold outline-none text-zinc-900 dark:text-zinc-50" />
                    <div className="flex items-center gap-1.5 bg-[#0066FF]/10 px-3 py-1.5 rounded-xl">
                        <span className="text-xs font-black text-[#0066FF]">$</span>
                        <input value={firstProductPrice || intakeData.initial_products?.[0]?.price || ""} onChange={(e) => setFirstProductPrice(e.target.value)} className="w-16 text-right bg-transparent border-none p-0 focus:ring-0 text-base font-black outline-none text-[#0066FF]" />
                    </div>
                  </div>
                </div>

                <div className="p-5 rounded-[24px] bg-zinc-50 dark:bg-zinc-900/80 border border-zinc-200 dark:border-zinc-800 shadow-sm">
                  <label className="text-[10px] font-black uppercase tracking-[0.15em] text-zinc-400 mb-4 block">Theme Template</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Modern', 'Minimal', 'Elegant', 'Bold'].map(t => (
                      <button key={t} onClick={() => setTemplate(t)} className={`p-3 rounded-xl text-xs font-black border-2 transition-all ${template === t ? 'bg-[#0066FF] text-white border-[#0066FF] shadow-lg shadow-blue-500/20' : 'bg-white dark:bg-zinc-800 border-transparent text-zinc-500 dark:text-zinc-400 hover:border-zinc-200 dark:hover:border-zinc-700'}`}>{t}</button>
                    ))}
                  </div>
                </div>

                <div className="p-5 rounded-[24px] bg-zinc-50 dark:bg-zinc-900/80 border border-zinc-200 dark:border-zinc-800 shadow-sm">
                  <label className="text-[10px] font-black uppercase tracking-[0.15em] text-zinc-400 mb-4 block">Domain</label>
                  <div className="flex flex-col gap-3">
                    <button onClick={() => setDomain('free')} className={`p-4 rounded-xl text-xs font-black text-left border-2 flex justify-between items-center transition-all ${domain === 'free' ? 'border-[#0066FF] bg-[#0066FF]/5 text-[#0066FF]' : 'border-transparent bg-white dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400'}`}>
                      <span>Free Subdomain</span>
                      <span className="text-[10px] font-bold opacity-60">.ohc.store</span>
                    </button>
                    <button onClick={() => setDomain('custom')} className={`p-4 rounded-xl text-xs font-black text-left border-2 flex justify-between items-center transition-all ${domain === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/5 text-[#0066FF]' : 'border-transparent bg-white dark:bg-zinc-800 text-zinc-500 dark:text-zinc-400 opacity-60'}`}>
                      <span>Custom Domain</span>
                      <span className="text-[10px] font-black uppercase bg-zinc-200 dark:bg-zinc-700 px-2 py-0.5 rounded-md text-zinc-500 dark:text-zinc-400">Soon</span>
                    </button>
                  </div>
                </div>
              </div>

              <div className="mt-auto flex gap-4">
                <button onClick={() => setStep(3)} className="px-8 py-5 rounded-2xl font-bold bg-zinc-100 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 transition-all">Edit</button>
                <button onClick={() => setStep(5)} className="flex-1 bg-[#0066FF] text-white p-5 rounded-2xl font-bold shadow-xl shadow-blue-500/25 hover:scale-[1.02] active:scale-[0.98] transition-all text-lg">Next</button>
              </div>
            </div>
          )}

          {step === 5 && (
            <div className="flex flex-col flex-1 animate-in">
              <h2 className="text-3xl font-bold font-outfit mb-3 tracking-tight text-zinc-900 dark:text-zinc-50">Your AI Team</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-10 leading-relaxed">Select your starting workforce. They handle the hard work for you.</p>

              <div className="space-y-4 mb-10">
                {AGENT_TEAM.map(agent => (
                  <button
                    key={agent.id}
                    onClick={() => toggleAgent(agent.id)}
                    className={`w-full p-5 rounded-[24px] border-2 flex items-start gap-5 transition-all text-left group ${selectedAgents.includes(agent.id) ? 'border-[#0066FF] bg-[#0066FF]/5 dark:bg-[#0066FF]/10' : 'border-transparent bg-zinc-100 dark:bg-zinc-900/60 opacity-80 hover:opacity-100'}`}
                  >
                    <span className="text-3xl pt-0.5">{agent.icon}</span>
                    <div className="flex-1">
                      <div className="flex justify-between items-start mb-1">
                        <div className="flex flex-col">
                            <h4 className="font-bold text-sm text-zinc-900 dark:text-zinc-50">{agent.id}</h4>
                            <span className="text-[10px] font-black text-[#0066FF] uppercase tracking-widest">{agent.name}</span>
                        </div>
                        <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all mt-1 ${selectedAgents.includes(agent.id) ? 'bg-[#0066FF] border-[#0066FF] shadow-md shadow-blue-500/30' : 'border-zinc-300 dark:border-zinc-700'}`}>
                          {selectedAgents.includes(agent.id) && <svg className="w-3.5 h-3.5 text-white" fill="none" stroke="currentColor" strokeWidth="4" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" /></svg>}
                        </div>
                      </div>
                      <p className="text-[11px] text-zinc-500 dark:text-zinc-400 font-medium leading-normal">{agent.desc}</p>
                    </div>
                  </button>
                ))}
              </div>

              <div className="mt-auto flex gap-4">
                <button onClick={() => setStep(4)} className="px-8 py-5 rounded-2xl font-bold bg-zinc-100 dark:bg-zinc-900 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-800 transition-all">Back</button>
                <button onClick={() => setStep(6)} className="flex-1 bg-[#0066FF] text-white p-5 rounded-2xl font-bold shadow-xl shadow-blue-500/25 hover:scale-[1.02] active:scale-[0.98] transition-all text-lg">Next</button>
              </div>
            </div>
          )}

          {step === 6 && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-in">
              <div className="w-28 h-28 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-10 relative">
                <span className="text-6xl">🚀</span>
                <div className="absolute -top-1 -right-1 w-10 h-10 bg-[#34C759] rounded-full border-4 border-white dark:border-zinc-900 flex items-center justify-center shadow-lg">
                    <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" strokeWidth="5" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" /></svg>
                </div>
              </div>
              <h2 className="text-3xl font-bold font-outfit mb-4 tracking-tight text-zinc-900 dark:text-zinc-50">Ready to Launch?</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-12 px-4 leading-relaxed">Your business is fully configured and ready for the world.</p>

              <div className="w-full p-8 rounded-[32px] bg-zinc-100/80 dark:bg-zinc-900/80 border border-zinc-200 dark:border-zinc-800 text-left mb-12 space-y-6 shadow-inner">
                 <div className="flex justify-between items-center border-b border-zinc-200/50 dark:border-zinc-800/50 pb-5">
                    <span className="text-[10px] text-zinc-400 font-black uppercase tracking-[0.2em]">Business</span>
                    <span className="text-sm font-bold text-zinc-900 dark:text-zinc-50">{businessName}</span>
                 </div>
                 <div className="flex justify-between items-center border-b border-zinc-200/50 dark:border-zinc-800/50 pb-5">
                    <span className="text-[10px] text-zinc-400 font-black uppercase tracking-[0.2em]">Workforce</span>
                    <span className="text-sm font-bold text-[#0066FF]">{selectedAgents.length} Agents</span>
                 </div>
                 <div className="flex justify-between items-center">
                    <span className="text-[10px] text-zinc-400 font-black uppercase tracking-[0.2em]">Template</span>
                    <span className="text-sm font-bold text-zinc-900 dark:text-zinc-50">{template}</span>
                 </div>
              </div>

              <div className="w-full flex flex-col gap-4">
                <button onClick={handleStartOnboarding} disabled={isLoading} className="w-full bg-[#34C759] text-white p-5 rounded-2xl font-bold shadow-xl shadow-green-500/25 hover:scale-[1.02] active:scale-[0.98] transition-all flex justify-center items-center h-20 text-xl tracking-tight">
                   {isLoading ? <span className="w-8 h-8 border-4 border-white/30 border-t-white rounded-full animate-spin"></span> : "Publish Now"}
                </button>
                <button onClick={() => setStep(5)} disabled={isLoading} className="w-full bg-transparent text-zinc-500 font-bold p-3 text-sm hover:text-[#0066FF] transition-colors">Back to AI Team</button>
              </div>
            </div>
          )}

          {step === 7 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-in">
              <div className="w-28 h-28 bg-[#34C759]/10 dark:bg-[#34C759]/20 rounded-full flex items-center justify-center mb-10">
                <svg className="w-14 h-14 text-[#34C759]" fill="none" stroke="currentColor" strokeWidth="4" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-4xl font-bold font-outfit mb-5 tracking-tighter text-zinc-900 dark:text-zinc-50">You're Live!</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-base mb-16 px-8 leading-relaxed font-medium">
                {startResult.message || "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-5 px-4">
                <a href="/dashboard" className="block w-full bg-zinc-900 dark:bg-white text-white dark:text-zinc-900 p-6 rounded-[24px] font-black shadow-2xl hover:opacity-90 active:scale-[0.98] transition-all text-lg tracking-tight">Go to Dashboard</a>
                <a href="/builder" className="block w-full bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-50 p-6 rounded-[24px] font-black border-2 border-zinc-100 dark:border-zinc-800 shadow-sm hover:bg-zinc-50 dark:hover:bg-zinc-700 active:scale-[0.98] transition-all text-lg tracking-tight">Preview Storefront</a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
