"use client";

import React, { useEffect, useState, useRef } from 'react';
import { useOnboardingStore } from './store';
type SetupIconName = 'dashboard' | 'eye' | 'launch' | 'next' | 'save' | 'sparkles';

function SetupIcon({ name }: { name: SetupIconName }) {
  const paths: Record<SetupIconName, string[]> = {
    dashboard: ['M4 5h7v7H4z', 'M13 5h7v4h-7z', 'M13 11h7v8h-7z', 'M4 14h7v5H4z'],
    eye: ['M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z', 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z'],
    launch: ['M13 10V3L4 14h7v7l9-11h-7z'],
    next: ['M5 12h14', 'M13 6l6 6-6 6'],
    save: ['M5 4h12l2 2v16H5z', 'M8 4v7h8V4', 'M8 18h8'],
    sparkles: ['M21 12l-3-1 1-3 1 3 3 1-3 1-1 3-1-3zM8 21l-3-4-4-3 4-3 3-4 3 4 4 3-4 3z'],
  };

  return (
    <svg className="h-4 w-4 flex-none" aria-hidden="true" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} viewBox="0 0 24 24">
      {paths[name].map((d) => <path key={d} d={d} />)}
    </svg>
  );
}

function IconLabel({ icon, children }: { icon: SetupIconName; children: React.ReactNode }) {
  return (
    <span className="inline-flex items-center justify-center gap-2 flex-none">
      <span className="flex-none inline-flex items-center justify-center w-4 h-4">
        <SetupIcon name={icon} />
      </span>
      <span className="whitespace-nowrap">{children}</span>
    </span>
  );
}

function generateSubdomain(name: string): string {
  if (!name || name.trim() === '') return 'my-business.ohc.app';
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return cleanName ? `${cleanName}.ohc.app` : 'my-business.ohc.app';
}

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessDescription, setBusinessDescription,
    businessGoal, setBusinessGoal,
    businessName, setBusinessName,
    bio, setBio,
    businessType, setBusinessType,
    categories, setCategories,
    websiteTemplate, setWebsiteTemplate,
    domainChoice, setDomainChoice,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    adminName, setAdminName,
    adminEmail, setAdminEmail,
    adminPassword, setAdminPassword,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const initialStateLoaded = useRef(false);

  const syncStateToBackend = async (overrideState: Partial<any> = {}) => {
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      businessDescription,
      businessName,
      businessType,
      categories,
      websiteTemplate,
      domainChoice,
      firstProductName,
      firstProductPrice,
      adminName,
      adminEmail,
      adminPassword,
      aiAgents,
      aiAutoRespond,
      ...overrideState
    };

    try {
      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          step: wizardState.step,
          wizardState
        })
      });
    } catch (e) {
      console.error('Failed to sync onboarding state', e);
    }
  };

  useEffect(() => {
    if (!initialStateLoaded.current) {
      initialStateLoaded.current = true;
      const fetchState = async () => {
        try {
          const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
          const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

          const res = await fetch('/api/onboarding/state', {
            headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
          });
          if (res.ok) {
            const data = await res.json();
            if (data.wizardState) {
              if (data.wizardState.step) setStep(data.wizardState.step);
              if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
              if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
              if (data.wizardState.categories) setCategories(data.wizardState.categories);
              if (data.wizardState.websiteTemplate) setWebsiteTemplate(data.wizardState.websiteTemplate);
              if (data.wizardState.domainChoice) setDomainChoice(data.wizardState.domainChoice);
              if (data.wizardState.firstProductName) setFirstProductName(data.wizardState.firstProductName);
              if (data.wizardState.firstProductPrice) setFirstProductPrice(data.wizardState.firstProductPrice);
              if (data.wizardState.adminName) setAdminName(data.wizardState.adminName);
              if (data.wizardState.adminEmail) setAdminEmail(data.wizardState.adminEmail);
              if (data.wizardState.adminPassword) setAdminPassword(data.wizardState.adminPassword);
              if (data.wizardState.aiAgents) setAiAgents(data.wizardState.aiAgents);
              if (typeof data.wizardState.aiAutoRespond === 'boolean') setAiAutoRespond(data.wizardState.aiAutoRespond);
            }
          }
        } catch (e) {
          console.error("Failed to load onboarding state", e);
        } finally {
          setIsLoaded(true);
        }
      };
      fetchState();
    } else {
      setIsLoaded(true);
    }
  }, [
    setStep, setBusinessName, setBusinessType, setCategories, setWebsiteTemplate,
    setDomainChoice, setFirstProductName, setFirstProductPrice, setAdminName,
    setAdminEmail, setAdminPassword, setAiAgents, setAiAutoRespond
  ]);

  const [validationError, setValidationError] = useState('');
  const [saveMessage, setSaveMessage] = useState('');

  const handleSaveDraft = async () => {
    await syncStateToBackend();
    setSaveMessage('Draft Saved!');
    setTimeout(() => setSaveMessage(''), 3000);
  };

  const handleStartOnboarding = async () => {
    if (!bio.trim()) return;
    setIsLoading(true);
    setError('');
    setStep(4);
    syncStateToBackend({ step: 4 });

    try {
      const tenantIdStr = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userIdStr = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantIdStr,
          'X-User-ID': userIdStr,
        },
        body: JSON.stringify({ description: bio }),
      });

      const data = await res.json();
      if (!res.ok) {
        throw new Error(data.error || data.message || 'Failed to analyze business details');
      }

      const inferredBusinessName = data.business_name || 'My Business';
      const inferredBusinessType = data.business_type || 'Online Store';
      const inferredProductName = data.initial_products?.[0]?.name || 'First Product';
      const inferredProductPrice = data.initial_products?.[0]?.price || '10.00';
      const inferredLocation = data.location || 'Unknown';

      setBusinessName(inferredBusinessName);
      setBusinessType(inferredBusinessType);
      setFirstProductName(inferredProductName);
      setFirstProductPrice(inferredProductPrice);

      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userIdStr },
        body: JSON.stringify({
          company_name: inferredBusinessName,
          admin_email: adminEmail || 'admin@example.com',
          admin_name: adminName || 'Admin',
          admin_password: adminPassword || 'password123',
          business_type: inferredBusinessType,
          first_product_name: inferredProductName,
          first_product_price: inferredProductPrice,
          price_type: 'physical',
          location: inferredLocation,
          ai_agents: ['Operations', 'Marketing', 'Finance', 'Legal', 'Advisory'],
          auto_respond: true,
          initial_products: data.initial_products || []
        })
      });

      const result = await startRes.json().catch(() => ({}));
      if (!startRes.ok) {
        throw new Error(result.error || result.message || 'Failed to start onboarding');
      }

      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      if (result.organization_id) {
        localStorage.setItem('tenant_id', result.organization_id);
        localStorage.setItem('tenant', result.organization_id);
      }
      setStep(5);
      syncStateToBackend({ step: 5 }); // Go to "You're Live" screen
      fetch('/api/onboarding/launch', { method: 'POST', headers: { 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userIdStr } }).catch(console.error);

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(0); syncStateToBackend({ step: 0 }); // Go back to start screen on error
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center p-4">
      <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[16px] ohc-hybrid-panel border border-white/20 shadow-2xl">
        <div className="px-6 pt-5 text-center">
          <h1 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Setup</h1>
          <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Your business, live in minutes.</p>
        </div>

        <div className="p-6 flex-1 flex flex-col overflow-y-auto custom-scrollbar">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-4 rounded-[8px] text-sm animate-shake">
              {error}
            </div>
          )}

          {step === 0 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What do you do?</h2>
              <div className="flex items-center justify-between mb-6 w-full">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center w-full">
                  Describe your business. Our AI will handle the rest in 30 seconds.
                </p>
              </div>

              <div className="space-y-4 flex-1 w-full relative">
                <textarea
                  value={bio}
                  onChange={(e) => setBio(e.target.value)}
                  className="w-full ohc-hybrid-panel min-h-[120px] p-4 pr-12 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7] shadow-inner rounded-[8px] text-lg"
                  placeholder="I'm a plumber in Miami..."
                  rows={6}
                  autoFocus
                />
                <button
                   className="absolute bottom-4 right-4 p-2 text-gray-400 hover:text-[#0066FF] transition-colors"
                   disabled
                   title="Voice input coming soon"
                >
                   <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                       <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
                   </svg>
                </button>
              </div>

              <div className="mt-auto pt-6 w-full">
                <button
                  onClick={handleStartOnboarding}
                  disabled={!bio.trim() || isLoading}
                  className="w-full bg-[#0066FF] text-white min-h-[44px] min-w-[44px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed group relative overflow-hidden"
                >
                  <div className="absolute inset-0 bg-white/20 group-hover:bg-transparent transition-colors duration-300"></div>
                  {isLoading ? (
                    <span className="flex items-center justify-center gap-2">
                      <svg className="animate-spin h-5 w-5 text-white backdrop-filter backdrop-blur-md rounded-full shadow-[0_0_10px_rgba(255,255,255,0.5)]" fill="none" viewBox="0 0 24 24">
                        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                      </svg>
                      Generating...
                    </span>
                  ) : <span className="animate-pulse-slow">Generate My Business</span>}
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in ohc-hybrid-panel rounded-[16px] shadow-2xl p-8">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Agents are building...</h2>
               <div className="space-y-2">
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Building your service menu</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring booking deposits</p>
                 <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
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
                <div className="p-3 ohc-hybrid-panel rounded-[8px] flex flex-col items-center mb-6">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-semibold">{generateSubdomain(businessName)}</span>
                   </div>
                </div>

                <a
                  href="/assistant"
                  className="flex w-full items-center justify-center bg-[#0066FF] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="sparkles">Publish & Share Link</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center ohc-hybrid-panel text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="eye">Preview Storefront</IconLabel>
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
