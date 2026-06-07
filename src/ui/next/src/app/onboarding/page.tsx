"use client";

import React, { useEffect, useState } from 'react';
import { useOnboardingStore } from './store';
type SetupIconName = 'dashboard' | 'eye' | 'launch' | 'next' | 'save';

function SetupIcon({ name }: { name: SetupIconName }) {
  const paths: Record<SetupIconName, string[]> = {
    dashboard: ['M4 5h7v7H4z', 'M13 5h7v4h-7z', 'M13 11h7v8h-7z', 'M4 14h7v5H4z'],
    eye: ['M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z', 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z'],
    launch: ['M13 10V3L4 14h7v7l9-11h-7z'],
    next: ['M5 12h14', 'M13 6l6 6-6 6'],
    save: ['M5 4h12l2 2v16H5z', 'M8 4v7h8V4', 'M8 18h8'],
  };

  return (
    <svg className="h-4 w-4 flex-none" aria-hidden="true" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} viewBox="0 0 24 24">
      {paths[name].map((d) => <path key={d} d={d} />)}
    </svg>
  );
}

function IconLabel({ icon, children }: { icon: SetupIconName; children: React.ReactNode }) {
  return (
    <span className="inline-flex items-center justify-center gap-2">
      <SetupIcon name={icon} />
      <span>{children}</span>
    </span>
  );
}

export default function OnboardingWizard() {
  const {
    step, setStep,
    chatStep, setChatStep,
    businessDescription, setBusinessDescription,
    businessName, setBusinessName,
    whatYouSell, setWhatYouSell,
    location, setLocation,
    targetAudience, setTargetAudience,
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

  const syncStateToBackend = async (overrideState: Partial<any> = {}) => {
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      chatStep,
      businessDescription,
      businessName,
      whatYouSell,
      location,
      targetAudience,
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
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ wizardState })
      });
    } catch (err) {
      console.error('Failed to sync onboarding state', err);
    }
  };
  const [validationError, setValidationError] = useState('');
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  const [saveMessage, setSaveMessage] = useState('');

  const handleSaveDraft = async () => {
    setIsLoading(true);
    setError('');

    try {
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
        domainChoice,
        firstProductName,
        firstProductPrice,
        adminName,
        adminEmail,
        adminPassword,
        aiAgents,
        aiAutoRespond
      };

      const res = await fetch('/api/onboarding/draft', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ wizardState })
      });

      if (!res.ok) {
        throw new Error('Draft endpoint failed');
      }

      setSaveMessage('Draft Saved!');
      setTimeout(() => setSaveMessage(''), 3000);
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred saving draft');
    } finally {
      setIsLoading(false);
    }
  };

  // Read state from server on mount
  useEffect(() => {
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    Promise.all([
      fetch('/api/onboarding/draft', { headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } })
        .then(res => res.ok ? res.json() : null)
        .catch(() => null),
      fetch('/api/onboarding/state', { headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } })
        .then(res => res.ok ? res.json() : null)
        .catch(() => null)
    ])
    .then(([draftData, stateData]) => {
      const data = (draftData && draftData.wizardState) ? draftData : stateData;
      if (data && data.wizardState) {
        if (data.wizardState.step) setStep(data.wizardState.step === 4 ? 3 : data.wizardState.step);
        if (data.wizardState.chatStep) setChatStep(data.wizardState.chatStep);
        if (data.wizardState.businessDescription) setBusinessDescription(data.wizardState.businessDescription);
        if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
        if (data.wizardState.whatYouSell) setWhatYouSell(data.wizardState.whatYouSell);
        if (data.wizardState.location) setLocation(data.wizardState.location);
        if (data.wizardState.targetAudience) setTargetAudience(data.wizardState.targetAudience);
        if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.categories) setCategories(data.wizardState.categories);
        if (data.wizardState.websiteTemplate) setWebsiteTemplate(data.wizardState.websiteTemplate);
        if (data.wizardState.firstProductName) setFirstProductName(data.wizardState.firstProductName);
        if (data.wizardState.firstProductPrice) setFirstProductPrice(data.wizardState.firstProductPrice);
        if (data.wizardState.adminName) setAdminName(data.wizardState.adminName);
        if (data.wizardState.adminEmail) setAdminEmail(data.wizardState.adminEmail);
        if (data.wizardState.adminPassword) setAdminPassword(data.wizardState.adminPassword);
        if (data.wizardState.domainChoice) setDomainChoice(data.wizardState.domainChoice);
        if (data.wizardState.aiAgents) setAiAgents(data.wizardState.aiAgents);
        if (data.wizardState.aiAutoRespond !== undefined) setAiAutoRespond(data.wizardState.aiAutoRespond);
      }
    })
    .catch(err => console.error('Failed to load onboarding state', err))
    .finally(() => setIsLoaded(true));
  }, []);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded) return;

    // Only save if we are past the initial state
    if (step === 1 && !businessName && !whatYouSell && !location && !targetAudience) return;

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      chatStep,
      businessDescription,
      businessName,
      whatYouSell,
      location,
      targetAudience,
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
    targetAudience, businessType, categories, websiteTemplate, domainChoice, firstProductName, firstProductPrice,
    adminName, adminEmail, adminPassword, aiAgents, aiAutoRespond, isLoaded
  ]);

  const handleZeroClickOnboarding = async () => {
    if (!businessDescription.trim()) {
      setValidationError('Please describe your business.');
      return;
    }
    setValidationError('');
    setIsLoading(true);
    setError('');
    setStep(4);
    syncStateToBackend({ step: 4 }); // Go to loading screen



    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      // 1. Intake
      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: businessDescription })
      });

      const intakeData = await intakeRes.json();
      if (!intakeRes.ok) {
        throw new Error(intakeData.error || intakeData.message || 'Failed to process business details');
      }

      // 2. Start Onboarding using extracted data and defaults
      const startRes = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({
          business_type: intakeData.business_type || 'Online Store',
          company_name: intakeData.business_name || 'My Business',
          company_description: businessDescription,
          selling_categories: intakeData.categories || ['physical'],
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: (intakeData.business_name || 'My Business') + ' Admin',
          admin_password: 'password123',
          website_template: 'Modern',
          first_product_name: intakeData.initial_products?.[0]?.name || 'First Product',
          first_product_price: intakeData.initial_products?.[0]?.price || '10.00',
          domain_choice: 'subdomain',
          price_type: 'fixed',
          location: intakeData.location || '',
          target_audience: intakeData.target_audience || ''
        })
      });

      const startResult = await startRes.json().catch(() => ({}));
      if (!startRes.ok) {
        throw new Error(startResult.error || startResult.message || 'Failed to start onboarding');
      }

      setStartResult(startResult);
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_onboarded', 'true');
        if (startResult.organization_id) {
          localStorage.setItem('tenant_id', startResult.organization_id);
          localStorage.setItem('tenant', startResult.organization_id);
        }
      }
      setStep(5);
      syncStateToBackend({ step: 5 }); // Go to "You're Live" screen
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      setStep(1);
      setChatStep(1);
      syncStateToBackend({ step: 1, chatStep: 1 });
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  // Progress percentage calculation
  const getProgress = () => {
    // There are 5 steps, let's make it a more gradual fill
    if (step === 1) {
      if (chatStep === 0) return 10;
      if (chatStep === 1) return 20;
      if (chatStep === 2) return 30;
      if (chatStep === 3) return 40;
      if (chatStep === 4) return 50;
    }
    if (step === 2) return 60;
    if (step === 3) return 80;
    if (step === 4) return 95;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <div className="min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center p-4">
      <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[16px] glassmorphism border border-white/20 shadow-2xl">
        <div className="px-6 pt-5 text-center">
          <h1 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Setup</h1>
          <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Your business, live in minutes.</p>
        </div>
        {/* Progress Bar */}
        <div className="h-1.5 w-full bg-gray-200 overflow-hidden">
          <div
            className="h-full bg-[#0066FF] transition-all duration-700 ease-out shadow-[0_0_10px_rgba(0,102,255,0.5)]"
            style={{ width: `${getProgress()}%` }}
          ></div>
        </div>

        <div className="p-6 flex-1 flex flex-col overflow-y-auto custom-scrollbar">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-4 rounded-[8px] text-sm animate-shake">
              {error}
            </div>
          )}

          {step === 1 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
                <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>

              {chatStep === 0 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in text-center">
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Welcome</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                    Let's get your business online in under 10 minutes.
                  </p>
                  <button
                    role="link"
                    onClick={() => { setChatStep(1); syncStateToBackend({ chatStep: 1 }); }}
                    className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                  >
                    Start Onboarding
                  </button>
                </div>
              )}

              {chatStep > 0 && (
                <>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                    Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
                  </p>
                </>
              )}

              {chatStep === 1 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full max-w-lg mx-auto">
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">Describe your business in one sentence.</h2>
                  <div className="flex items-center justify-between mb-6 w-full text-center">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm w-full">
                      Our AI will instantly generate your storefront, products, and back-office agents.
                    </p>
                  </div>

                  <div className="space-y-4 flex-1 w-full">
                    <div>
                      <textarea
                        autoFocus
                        value={businessDescription}
                        onChange={(e) => setBusinessDescription(e.target.value)}
                        placeholder="e.g. I sell vegan cakes in Austin."
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all shadow-inner"
                      />
                    </div>
                  </div>

                  {validationError && <p className="text-red-500 text-sm font-semibold mb-2">{validationError}</p>}
                  <div className="mt-auto pt-6 w-full">
                    <button
                      onClick={handleZeroClickOnboarding}
                      disabled={!businessDescription.trim() || isLoading}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {isLoading ? (
                        <span className="flex items-center justify-center gap-2">
                          <svg className="animate-spin h-5 w-5 text-white backdrop-filter backdrop-blur-md rounded-full shadow-[0_0_10px_rgba(255,255,255,0.5)]" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                          </svg>
                          Analyzing...
                        </span>
                      ) : <IconLabel icon="launch">Generate My Storefront</IconLabel>}
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {step === 4 && (
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in glassmorphism rounded-[16px] shadow-2xl p-8">
               <div className="w-24 h-24 relative mb-8">
                 <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
                 <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
               </div>
               <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">The Promoter is designing your storefront...</h2>
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
                <div className="p-3 glassmorphism rounded-[8px] flex flex-col items-center mb-6">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-semibold">my-business.ohc.app</span>
                   </div>
                </div>

                <a
                  href="/dashboard"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-md hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="dashboard">Go to Dashboard</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
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
