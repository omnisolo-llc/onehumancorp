"use client";

import React from 'react';
import { useOnboardingStore } from './store';

const AdvancedToggle = ({ isOpen, onToggle }: { isOpen: boolean, onToggle: () => void }) => (
  <button
    onClick={(e) => {
      e.preventDefault();
      onToggle();
    }}
    className="flex items-center text-xs font-semibold text-gray-400 hover:text-zinc-600 dark:hover:text-zinc-300 transition-colors mt-4"
  >
    <svg
      className={`w-3 h-3 mr-1 transition-transform ${isOpen ? 'rotate-90' : ''}`}
      fill="none" stroke="currentColor" viewBox="0 0 24 24"
    >
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M9 5l7 7-7 7" />
    </svg>
    {isOpen ? 'Hide Advanced Settings' : 'Show Advanced Settings'}
  </button>
);

export default function OnboardingWizard() {
  const [showAdvanced, setShowAdvanced] = React.useState(false);
  const {
    step, setStep,
    businessName, setBusinessName,
    businessCategory, setBusinessCategory,
    selectedAgents, toggleAgent,
    isLoading, setIsLoading,
    error, setError,
    intakeData, setIntakeData,
    startResult, setStartResult,
    fetchState, saveState
  } = useOnboardingStore();

  React.useEffect(() => {
    fetchState();
  }, [fetchState]);

  React.useEffect(() => {
    if (step > 1 || businessName || businessCategory) {
      saveState();
    }
  }, [step, businessName, businessCategory, saveState]);

  const handleNext = () => {
    if (step === 1 && businessName.trim().length < 3) {
      setError("Business name must be at least 3 characters.");
      return;
    }
    if (step === 2 && businessCategory.trim().length < 10) {
      setError("Please provide a bit more detail (min 10 chars).");
      return;
    }
    setError("");
    setStep(step + 1);
  };

  const handleIntakeSubmit = async () => {
    if (businessCategory.trim().length < 10) {
      setError("Please provide a bit more detail (min 10 chars).");
      return;
    }

    setError("");
    setIsLoading(true);

    const combinedDescription = `Business Name: ${businessName}\nCategory/Products: ${businessCategory}`;

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
      setStep(3);
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
        company_name: intakeData.business_name || businessName,
        company_description: businessCategory,
        selling_categories: intakeData.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: "modern",
        first_product_name: intakeData.initial_products?.[0]?.name || "Sample Product",
        first_product_price: intakeData.initial_products?.[0]?.price || "10.00",
        selected_agents: selectedAgents,
        domain_choice: "subdomain",
        price_type: "fixed",
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
      setStep(5);
    } catch (err: any) {
      setError(err.message || 'An error occurred starting your business.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-[100dvh] font-inter p-0 sm:p-4 bg-transparent">
      <div className="w-full max-w-[375px] min-h-[100dvh] sm:min-h-[667px] sm:max-h-[812px] shadow-2xl flex flex-col relative overflow-hidden glass-container sm:rounded-[16px] rounded-0">
        {/* Header */}
        <div className="w-full p-6 pb-2 pt-10 flex justify-between items-center z-10">
           <h1 className="text-xl font-bold font-outfit tracking-tight">OHC Setup</h1>
           <div className="text-[10px] font-bold uppercase tracking-wider px-2 py-1 bg-blue-500/10 text-[#0066FF] rounded-full" aria-label={`Step ${step} of 5`}>
             Step {step} of 5
           </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 p-6 z-10 flex flex-col">
          {error && (
            <div role="alert" className="mb-4 p-3 bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-400 text-sm rounded-[8px] animate-in">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col flex-1 justify-center animate-in">
              <h2 className="text-2xl font-bold font-outfit mb-2 leading-tight">What's the name of your business?</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8">This will be your brand's public identity.</p>
              <input
                type="text"
                value={businessName}
                onChange={(e) => {
                  setBusinessName(e.target.value);
                  if (e.target.value.trim().length >= 3) setError("");
                }}
                placeholder="e.g. Maya's Cakes"
                className={`w-full p-4 glass-input outline-none text-lg mb-4 ${error && businessName.trim().length < 3 ? 'border-red-500/50' : ''}`}
                autoFocus
                aria-label="Business Name"
              />
              <button
                onClick={handleNext}
                className="w-full bg-[#0066FF] text-white p-4 glass-button font-bold shadow-lg shadow-blue-500/20 hover:bg-blue-600 active:scale-[0.98] transition-all focus:ring-4 focus:ring-blue-500/20 outline-none"
              >
                Continue
              </button>

              <AdvancedToggle isOpen={showAdvanced} onToggle={() => setShowAdvanced(!showAdvanced)} />
              {showAdvanced && (
                <div className="mt-4 p-4 glass-card animate-in">
                   <label className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest block mb-2">Organization ID</label>
                   <input
                    type="text"
                    disabled
                    placeholder="Auto-assigned"
                    className="w-full p-3 glass-input text-sm opacity-50 cursor-not-allowed"
                   />
                </div>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col flex-1 justify-center animate-in">
              <h2 className="text-2xl font-bold font-outfit mb-2 leading-tight">What's your niche?</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8">Briefly describe what you sell or do.</p>
              <textarea
                value={businessCategory}
                onChange={(e) => {
                  setBusinessCategory(e.target.value);
                  if (e.target.value.trim().length >= 10) setError("");
                }}
                placeholder="e.g. I bake custom sourdough bread and deliver locally"
                className={`w-full p-4 glass-input outline-none text-lg mb-4 h-32 resize-none ${error && businessCategory.trim().length < 10 ? 'border-red-500/50' : ''}`}
                autoFocus
                aria-label="Niche description"
              />
              <div className="flex gap-3">
                <button
                  onClick={() => setStep(1)}
                  className="px-6 py-4 glass-button font-bold bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-700 outline-none focus:ring-4 focus:ring-zinc-500/10"
                >
                  Back
                </button>
                <button
                  onClick={handleIntakeSubmit}
                  disabled={isLoading}
                  className="flex-1 bg-[#0066FF] text-white p-4 glass-button font-bold shadow-lg shadow-blue-500/20 hover:bg-blue-600 active:scale-[0.98] disabled:opacity-70 flex justify-center items-center transition-all outline-none focus:ring-4 focus:ring-blue-500/20"
                >
                  {isLoading ? (
                    <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                  ) : (
                    "Next Step"
                  )}
                </button>
              </div>

              <AdvancedToggle isOpen={showAdvanced} onToggle={() => setShowAdvanced(!showAdvanced)} />
              {showAdvanced && (
                <div className="mt-4 p-4 glass-card animate-in">
                   <label className="text-[10px] font-bold text-zinc-400 uppercase tracking-widest block mb-2">Model Temperature</label>
                   <input
                    type="range"
                    disabled
                    className="w-full accent-[#0066FF] opacity-50"
                   />
                </div>
              )}
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col flex-1 animate-in">
              <h2 className="text-2xl font-bold font-outfit mb-2 leading-tight">Assemble your AI Team</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-6">Choose agents to handle your operations.</p>

              <div className="space-y-3 flex-1 overflow-y-auto mb-6 pr-1 custom-scrollbar" role="group" aria-label="AI Team Selection">
                {[
                  { id: 'manager', role: 'The Manager', desc: 'Operations & tasks', icon: '👔' },
                  { id: 'promoter', role: 'The Promoter', desc: 'Brand growth & SEO', icon: '📣' },
                  { id: 'sales', role: 'The Salesperson', desc: 'Leads & revenue', icon: '💰' },
                  { id: 'ambassador', role: 'The Ambassador', desc: 'Customer delight', icon: '🤝' },
                ].map((agent) => {
                  const isSelected = selectedAgents.includes(agent.id);
                  return (
                    <button
                      key={agent.id}
                      onClick={() => toggleAgent(agent.id)}
                      aria-pressed={isSelected}
                      className={`w-full glass-card p-4 flex items-center gap-4 cursor-pointer transition-all border-2 text-left outline-none focus:ring-4 focus:ring-blue-500/20 ${
                        isSelected
                          ? 'border-[#0066FF] bg-blue-500/5'
                          : 'border-transparent hover:bg-white/40 dark:hover:bg-white/5'
                      }`}
                    >
                      <div className={`w-12 h-12 rounded-xl flex items-center justify-center text-2xl transition-colors ${
                        isSelected ? 'bg-blue-500/20' : 'bg-zinc-100 dark:bg-zinc-800'
                      }`}>
                        {agent.icon}
                      </div>
                      <div className="flex-1">
                        <div className="font-bold text-sm text-zinc-900 dark:text-zinc-100">{agent.role}</div>
                        <div className="text-xs text-zinc-500">{agent.desc}</div>
                      </div>
                      <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all ${
                        isSelected ? 'bg-[#0066FF] border-[#0066FF]' : 'border-zinc-300 dark:border-zinc-700'
                      }`}>
                        {isSelected && (
                          <svg className="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                          </svg>
                        )}
                      </div>
                    </button>
                  );
                })}
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => setStep(2)}
                  className="px-6 py-4 glass-button font-bold bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-700 outline-none focus:ring-4 focus:ring-zinc-500/10"
                >
                  Back
                </button>
                <button
                  onClick={() => {
                    if (selectedAgents.length === 0) {
                      setError("Please select at least one agent.");
                      return;
                    }
                    setError("");
                    setStep(4);
                  }}
                  className="flex-1 bg-[#0066FF] text-white p-4 glass-button font-bold shadow-lg shadow-blue-500/20 hover:bg-blue-600 active:scale-[0.98] transition-all outline-none focus:ring-4 focus:ring-blue-500/20"
                >
                  Confirm Team
                </button>
              </div>
            </div>
          )}

          {step === 4 && intakeData && (
            <div className="flex flex-col flex-1 animate-in">
              <div className="w-16 h-16 bg-blue-500/10 rounded-full flex items-center justify-center mb-6 mx-auto">
                <span className="text-3xl text-[#0066FF]" role="img" aria-label="Sparkles">✨</span>
              </div>
              <h2 className="text-2xl font-bold font-outfit mb-2 text-center">Ready to launch?</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-8 text-center px-4">Review your business profile before we go live.</p>

              <div className="glass-card p-5 shadow-sm mb-8 space-y-4">
                <div className="flex justify-between items-start">
                  <div>
                    <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-wider">Business Name</span>
                    <div className="font-semibold text-lg">{intakeData.business_name}</div>
                  </div>
                  <div className="text-right">
                    <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-wider">Type</span>
                    <div className="text-xs font-medium bg-zinc-100 dark:bg-zinc-800 px-2 py-1 rounded-md">{intakeData.business_type}</div>
                  </div>
                </div>
                <div>
                  <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-wider">Launch Inventory</span>
                  <div className="mt-2 space-y-2">
                    {intakeData.initial_products?.slice(0, 3).map((p: any, i: number) => (
                      <div key={i} className="flex justify-between items-center text-sm p-2 bg-black/5 dark:bg-white/5 rounded-lg">
                        <span>{p.name}</span>
                        <span className="font-bold text-[#34C759]">${p.price}</span>
                      </div>
                    ))}
                  </div>
                </div>
                <div>
                  <span className="text-[10px] font-bold text-zinc-400 uppercase tracking-wider">Selected AI Team</span>
                  <div className="flex gap-2 mt-2">
                    {selectedAgents.map(id => (
                      <div key={id} className="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center text-sm" title={id}>
                        {id === 'manager' ? '👔' : id === 'promoter' ? '📣' : id === 'sales' ? '💰' : '🤝'}
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              <div className="flex gap-3 mt-auto">
                <button
                  onClick={() => setStep(3)}
                  className="px-6 py-4 glass-button font-bold bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-700 outline-none focus:ring-4 focus:ring-zinc-500/10"
                  disabled={isLoading}
                >
                  Edit
                </button>
                <button
                  onClick={handleStartOnboarding}
                  disabled={isLoading}
                  className="flex-1 bg-[#34C759] text-white p-4 glass-button font-bold shadow-lg shadow-green-500/20 hover:bg-[#2eb350] active:scale-[0.98] disabled:opacity-70 flex justify-center items-center transition-all outline-none focus:ring-4 focus:ring-green-500/20"
                >
                  {isLoading ? (
                    <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                  ) : (
                    "Launch Business"
                  )}
                </button>
              </div>
            </div>
          )}

          {step === 5 && startResult && (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-in">
              <div className="w-20 h-20 bg-green-500/10 rounded-full flex items-center justify-center mb-6">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-3xl font-bold font-outfit mb-2 tracking-tight">You're Live!</h2>
              <p className="text-zinc-500 dark:text-zinc-400 text-sm mb-12 px-4 leading-relaxed">
                {startResult.message || "Your business has been successfully launched and is ready to accept customers."}
              </p>

              <div className="w-full space-y-3 mt-auto">
                <a
                  href="/dashboard"
                  className="block w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#1D1D1F] p-4 glass-button font-bold shadow-lg active:scale-[0.98] transition-all outline-none focus:ring-4 focus:ring-zinc-500/20 text-center"
                >
                  Open Dashboard
                </a>
                <button
                  onClick={() => window.location.reload()}
                  className="block w-full bg-transparent text-zinc-500 dark:text-zinc-400 p-3 text-sm font-medium hover:text-zinc-800 dark:hover:text-zinc-100 transition-colors outline-none"
                >
                  Start Another Business
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
      <style jsx>{`
        .custom-scrollbar::-webkit-scrollbar {
          width: 4px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
          background: transparent;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: rgba(0, 0, 0, 0.1);
          border-radius: 10px;
        }
        @media (prefers-color-scheme: dark) {
          .custom-scrollbar::-webkit-scrollbar-thumb {
            background: rgba(255, 255, 255, 0.1);
          }
        }
      `}</style>
    </div>
  );
}
