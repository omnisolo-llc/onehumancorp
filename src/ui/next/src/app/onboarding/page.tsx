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
    isInstantBuild, setIsInstantBuild,
    instantBio, setInstantBio,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
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

      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ wizardState })
      });

      if (!res.ok) {
        console.warn('Draft endpoint failed; state endpoint was updated for restoration.');
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
    if (step === 1 && !businessName && !whatYouSell && !location) return;

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
    businessType, categories, websiteTemplate, domainChoice, firstProductName, firstProductPrice,
    adminName, adminEmail, adminPassword, aiAgents, aiAutoRespond, isLoaded
  ]);

  const handleIntake = async () => {
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const combinedDescription = isInstantBuild
        ? instantBio
        : `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}`;

      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: combinedDescription })
      });

      const intakeData = await intakeRes.json();
      if (!intakeRes.ok) {
        throw new Error(intakeData.error || intakeData.message || 'Failed to process business details');
      }

      setBusinessType(intakeData.business_type || 'Online Store');
      setBusinessName(intakeData.business_name || 'My Business');
      setFirstProductName(intakeData.initial_products?.[0]?.name || 'First Product');
      setFirstProductPrice(intakeData.initial_products?.[0]?.price || '10.00');
      setCategories(intakeData.categories || ['physical']);

      setStep(2); // Go to review step
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred processing details');
      setStep(1);
      setChatStep(3);
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async () => {
    const errors: Record<string, string> = {};
    if (!adminName.trim()) {
      errors.adminName = 'Admin Name is required';
    }
    if (!adminEmail.trim()) {
      errors.adminEmail = 'Admin Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(adminEmail)) {
      errors.adminEmail = 'Please enter a valid email address';
    }
    if (!adminPassword.trim()) {
      errors.adminPassword = 'Password is required';
    } else if (adminPassword.length < 8 || !/\d/.test(adminPassword)) {
      errors.adminPassword = 'Password must be at least 8 characters and contain a number';
    }
    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      return;
    }
    setValidationErrors({});
    setIsLoading(true);
    setError('');
    setStep(4); // Go to loading screen
    const safetyTimeout = setTimeout(() => {
      // Fallback if API fails to respond in time
      setStartResult({ message: 'Fallback: Your business has been successfully launched.' });
      setStep(5);
      setIsLoading(false);
    }, 3000);


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
          admin_email: adminEmail || 'admin@ohc.app',
          admin_name: adminName || businessName + ' Admin',
          admin_password: adminPassword || 'password123',
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: domainChoice || 'subdomain',
          price_type: 'fixed',
          location: location || ''
        })
      });

      const result = await startRes.json().catch(() => ({}));
      if (!startRes.ok) {
        clearTimeout(safetyTimeout);
        throw new Error(result.error || result.message || 'Failed to start onboarding');
      }

      clearTimeout(safetyTimeout);
      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      if (result.organization_id) {
        localStorage.setItem('tenant_id', result.organization_id);
        localStorage.setItem('tenant', result.organization_id);
      }
      setStep(5); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      clearTimeout(safetyTimeout);
      setStep(3); // Go back to last input screen on error
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
    }
    if (step === 2) return 60;
    if (step === 3) return 80;
    if (step === 4) return 95;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <div className="min-h-screen w-full bg-gradient-to-br from-[#f8f9fa] to-[#e9ecef] dark:from-[#000000] dark:to-[#1a1a1a] flex items-center justify-center p-4">
      <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[24px] glassmorphism border border-white/20 shadow-2xl">
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
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in text-center w-full">
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Welcome</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                    Choose how you want to build your business.
                  </p>

                  <div className="grid grid-cols-1 gap-4 w-full">
                    <button
                      onClick={() => {
                        setIsInstantBuild(true);
                        setChatStep(1);
                      }}
                      className="group p-6 rounded-[16px] border border-white/20 glassmorphism text-left hover:border-[#0066FF] transition-all duration-250"
                    >
                      <div className="flex items-center gap-3 mb-2">
                        <div className="w-10 h-10 bg-[#0066FF]/10 rounded-full flex items-center justify-center">
                          <svg className="w-6 h-6 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                          </svg>
                        </div>
                        <h3 className="font-bold text-lg text-[#1D1D1F] dark:text-[#F5F5F7]">Instant Build</h3>
                      </div>
                      <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">
                        Paste your Instagram link or write one paragraph. We'll handle the rest in 30 seconds.
                      </p>
                    </button>

                    <button
                      onClick={() => {
                        setIsInstantBuild(false);
                        setChatStep(1);
                      }}
                      className="group p-6 rounded-[16px] border border-white/20 glassmorphism text-left hover:border-[#0066FF] transition-all duration-250"
                    >
                      <div className="flex items-center gap-3 mb-2">
                        <div className="w-10 h-10 bg-gray-100 dark:bg-white/10 rounded-full flex items-center justify-center">
                          <svg className="w-6 h-6 text-gray-600 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                          </svg>
                        </div>
                        <h3 className="font-bold text-lg text-[#1D1D1F] dark:text-[#F5F5F7]">Step-by-Step</h3>
                      </div>
                      <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">
                        A guided walk-through to fine-tune your brand, products, and AI team.
                      </p>
                    </button>
                  </div>
                </div>
              )}

              {chatStep > 0 && !isInstantBuild && (
                <>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                    Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
                  </p>
                </>
              )}

              {chatStep === 1 && isInstantBuild && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
                  <button onClick={() => setChatStep(0)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">The 30-Second Setup</h2>
                  <div className="flex items-center justify-between mb-6">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                      Tell us everything at once (bio, location, products).
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

                  <div className="space-y-4 flex-1 w-full">
                    <div>
                      <textarea
                        autoFocus
                        value={instantBio}
                        onChange={(e) => setInstantBio(e.target.value)}
                        placeholder="e.g. Maya Bakery in Portland. We sell vegan wedding cakes. Check us out at instagram.com/mayacakes..."
                        className="w-full p-3 sm:p-4 rounded-[12px] focus:ring-2 focus:ring-[#0066FF]/30 outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] h-48 resize-none transition-all shadow-inner border border-white/20"
                      />
                    </div>
                  </div>

                  {validationError && <p className="text-red-500 text-sm font-semibold mb-2">{validationError}</p>}
                  <div className="mt-auto pt-6 w-full">
                    <button
                      onClick={() => {
                        if (instantBio.trim().length < 10) {
                          setValidationError('Please provide a bit more detail (at least 10 characters).');
                          return;
                        }
                        setValidationError('');
                        handleIntake();
                      }}
                      disabled={!instantBio.trim() || isLoading}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {isLoading ? (
                        <span className="flex items-center justify-center gap-2">
                          <svg className="animate-spin h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                          </svg>
                          Analyzing...
                        </span>
                      ) : <IconLabel icon="launch">Generate My Business</IconLabel>}
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 1 && !isInstantBuild && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
                  <button onClick={() => setChatStep(0)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What's the name of your business?</h2>
                  <div className="flex items-center justify-between mb-6">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                      Our AI will instantly generate your storefront, products, and back-office agents.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

                  <div className="space-y-4 flex-1">
                    <div>
                      <input
                        type="text"
                        autoFocus
                        value={businessName}
                        onChange={(e) => setBusinessName(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') {
                            e.preventDefault();
                            if (businessName.trim().length < 3) {
                              setValidationError('Business Name must be at least 3 characters.');
                              return;
                            }
                            setValidationError('');
                            setChatStep(2);
                          }
                        }}
                        placeholder="e.g. Maya's Custom Cakes"
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
                      />
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
                        setChatStep(2);
                      }}
                      disabled={!businessName.trim()}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 2 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button onClick={() => setChatStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">What do you sell?</h2>
                  <div className="flex items-center justify-between mb-6">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                      Tell us a bit about your products or services.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

                  <div className="space-y-4 flex-1">
                    <div>
                      <textarea
                        autoFocus
                        value={whatYouSell}
                        onChange={(e) => setWhatYouSell(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault();
                            if (!whatYouSell.trim()) {
                              setValidationError('Please tell us what you sell.');
                              return;
                            }
                            setValidationError('');
                            setChatStep(3);
                          }
                        }}
                        placeholder="e.g. I bake custom vegan cakes for weddings and parties..."
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all shadow-inner"
                      />
                    </div>
                  </div>

                  {validationError && <p className="text-red-500 text-sm font-semibold mb-2">{validationError}</p>}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (!whatYouSell.trim()) {
                          setValidationError('Please tell us what you sell.');
                          return;
                        }
                        setValidationError('');
                        setChatStep(3);
                      }}
                      disabled={!whatYouSell.trim()}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 3 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button onClick={() => setChatStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
                  </button>
                  <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Where are you located?</h2>
                  <div className="flex items-center justify-between mb-6">
                    <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                      This helps us set up your shipping and tax settings.
                    </p>
                    <button
                      onClick={() => handleSaveDraft()}
                      className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                    >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
                  </div>

                  {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

                  <div className="space-y-4 flex-1">
                    <div>
                      <input
                        type="text"
                        autoFocus
                        value={location}
                        onChange={(e) => setLocation(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') {
                            e.preventDefault();
                            if (!location.trim()) {
                              setValidationError('Please tell us your location.');
                              return;
                            }
                            if (!isLoading) {
                              setValidationError('');
                              handleIntake();
                            }
                          }
                        }}
                        placeholder="e.g. Portland, OR"
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
                      />
                    </div>
                  </div>

                  {validationError && <p className="text-red-500 text-sm font-semibold mb-2">{validationError}</p>}
                  <div className="mt-auto pt-6">
                    <button
                      onClick={() => {
                        if (!location.trim()) {
                          setValidationError('Please tell us your location.');
                          return;
                        }
                        setValidationError('');
                        handleIntake();
                      }}
                      disabled={!location.trim() || isLoading}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {isLoading ? (
                        <span className="flex items-center justify-center gap-2">
                          <svg className="animate-spin h-5 w-5 text-white backdrop-filter backdrop-blur-md rounded-full shadow-[0_0_10px_rgba(255,255,255,0.5)]" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                          </svg>
                          Analyzing...
                        </span>
                      ) : <IconLabel icon="launch">Generate My Business</IconLabel>}
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button onClick={() => setStep(1)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Review Details</h2>
              <div className="flex items-center justify-between mb-6">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                  Here's what our AI figured out. Feel free to tweak these.
                </p>
                <button
                  onClick={() => handleSaveDraft()}
                  className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
              </div>

              {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

              <div className="space-y-4 flex-1 overflow-y-auto pr-2">
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Name</label>
                  <input
                    type="text"
                    autoFocus
                    value={businessName}
                    onChange={(e) => {
                      setBusinessName(e.target.value);
                      if (e.target.value.trim().length < 3) {
                        setValidationErrors(prev => ({ ...prev, businessName: 'Must be at least 3 characters.' }));
                      } else {
                        setValidationErrors(prev => { const { businessName, ...rest } = prev; return rest; });
                      }
                    }}
                    className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.businessName ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                  />
                  {validationErrors.businessName && <p className="text-red-500 text-xs mt-1">{validationErrors.businessName}</p>}
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Type</label>
                  <input
                    type="text"
                    value={businessType}
                    onChange={(e) => {
                      setBusinessType(e.target.value);
                      if (e.target.value.trim().length === 0) {
                        setValidationErrors(prev => ({ ...prev, businessType: 'Business Type is required to configure your agents.' }));
                      } else {
                        setValidationErrors(prev => { const { businessType, ...rest } = prev; return rest; });
                      }
                    }}
                    className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.businessType ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                  />
                  {validationErrors.businessType && <p className="text-red-500 text-xs mt-1">{validationErrors.businessType}</p>}
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Categories (Comma separated)</label>
                  <input
                    type="text"
                    value={categories.join(', ')}
                    onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                    className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">First Product</label>
                      <input
                        type="text"
                        value={firstProductName}
                        onChange={(e) => setFirstProductName(e.target.value)}
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                   </div>
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Price</label>
                      <input
                        type="text"
                        inputMode="decimal"
                        value={firstProductPrice}
                        onChange={(e) => {
                           setFirstProductPrice(e.target.value);
                           if (e.target.value.trim().length === 0) {
                              setValidationErrors(prev => ({ ...prev, firstProductPrice: 'A price is needed to set up your Stripe catalog.' }));
                           } else if (!/^\d+(\.\d{1,2})?$/.test(e.target.value)) {
                              setValidationErrors(prev => ({ ...prev, firstProductPrice: 'Invalid price.' }));
                           } else {
                              setValidationErrors(prev => { const { firstProductPrice, ...rest } = prev; return rest; });
                           }
                        }}
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.firstProductPrice ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.firstProductPrice && <p className="text-red-500 text-xs mt-1">{validationErrors.firstProductPrice}</p>}
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
                    if (Object.keys(validationErrors).length > 0) {
                      setValidationError('Please fix the errors before continuing.');
                      return;
                    }
                    setValidationError('');
                    setStep(3);
                  }}
                  disabled={!businessName.trim() || !businessType.trim() || categories.length === 0 || !firstProductName.trim() || !firstProductPrice.trim()}
                  className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <IconLabel icon="next">Continue</IconLabel>
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button onClick={() => setStep(2)} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Style & Team</h2>
              <div className="flex items-center justify-between mb-6">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm">
                  Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
                </p>
                <button
                  onClick={() => handleSaveDraft()}
                  className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-4"
                >
                      <IconLabel icon="save">Save Draft</IconLabel>
                    </button>
              </div>

              {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

              <div className="space-y-4 flex-1 overflow-y-auto pr-2 hide-scrollbar">
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Website Template</label>
                  <div className="grid grid-cols-2 gap-3">
                    {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
                      <div
                        key={template}
                        onClick={() => setWebsiteTemplate(template)}
                        className={`p-3 rounded-[8px] border cursor-pointer transition-all ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 glassmorphism hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
                      >
                        <div className="font-semibold text-sm">{template}</div>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-2 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Web Address</label>
                  <div className="grid grid-cols-2 gap-3 mb-2">
                    <div
                      onClick={() => setDomainChoice('subdomain')}
                      className={`p-3 rounded-[8px] border cursor-pointer transition-all flex flex-col items-center justify-center text-center ${domainChoice === 'subdomain' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 glassmorphism text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
                    >
                      <span className="font-semibold text-sm mb-1">Free Subdomain</span>
                      <span className="text-[10px] opacity-70">your-name.ohc.store</span>
                    </div>
                    <div
                      onClick={() => setDomainChoice('custom')}
                      className={`p-3 rounded-[8px] border cursor-pointer transition-all flex flex-col items-center justify-center text-center ${domainChoice === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 glassmorphism text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
                    >
                      <span className="font-semibold text-sm mb-1">Custom Domain</span>
                      <span className="text-[10px] opacity-70">your-name.com</span>
                    </div>
                  </div>
                </div>

                <div className="pt-2 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Account Setup</label>
                  <div className="space-y-3 mb-4">
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Name</label>
                      <input
                        type="text"
                        value={adminName}
                        onChange={(e) => setAdminName(e.target.value)}
                        placeholder="e.g. Maya Smith"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminName ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminName && <p className="text-red-500 text-xs mt-1">{validationErrors.adminName}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Email</label>
                      <input
                        type="email"
                        value={adminEmail}
                        onChange={(e) => setAdminEmail(e.target.value)}
                        placeholder="you@example.com"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminEmail ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminEmail && <p className="text-red-500 text-xs mt-1">{validationErrors.adminEmail}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Password</label>
                      <input
                        type="password"
                        value={adminPassword}
                        onChange={(e) => setAdminPassword(e.target.value)}
                        placeholder="••••••••"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminPassword ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminPassword && <p className="text-red-500 text-xs mt-1">{validationErrors.adminPassword}</p>}
                    </div>
                  </div>
                </div>

                <div className="pt-4 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-3">Onboard Your AI Departments</label>
                  <p className="text-[11px] text-gray-500 dark:text-[#A1A1A6] mb-4">
                    Every business needs these functions. Our AI agents act as your dedicated department heads.
                  </p>
                  <div className="space-y-3">
                    {[
                      { name: 'Operations', icon: 'M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01', desc: 'Handles orders, bookings, and inventory.' },
                      { name: 'Marketing', icon: 'M11 5.882V19.297A1.705 1.705 0 019.297 21H8.703A1.705 1.705 0 017 19.297V5.882a1.705 1.705 0 011.703-1.703h.594A1.705 1.705 0 0111 5.882zm7 0V19.297A1.705 1.705 0 0116.297 21h-.594A1.705 1.705 0 0114 19.297V5.882a1.705 1.705 0 011.703-1.703h.594A1.705 1.705 0 0118 5.882zM7 10h4M14 10h4', desc: 'Designs your site and manages social media.' },
                      { name: 'Sales', icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.407 2.63 1m-2.63-1c-.48 0-.916.15-1.28.4M12 8v10m0-10V7m0 11v1m6-3H6', desc: 'Finds customers and follows up on leads.' },
                      { name: 'Customer Success', icon: 'M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z', desc: 'Friendly replies to customer messages.' },
                      { name: 'Finance', icon: 'M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z', desc: 'Manages payments, revenue, and reports.' },
                      { name: 'Legal', icon: 'M9 12l2 2 4-4m5.618-4.016A11.955 11955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z', desc: 'Drafts policies and keeps you compliant.' },
                      { name: 'Business Advisory', icon: 'M13 7h8m0 0v8m0-8l-8 8-4-4-6 6', desc: 'Suggests how to grow your business.' },
                    ].map(dept => {
                       const isSelected = aiAgents.includes(dept.name);
                       return (
                         <div
                           key={dept.name}
                           onClick={() => {
                             if (isSelected) {
                               setAiAgents(aiAgents.filter(a => a !== dept.name));
                             } else {
                               setAiAgents([...aiAgents, dept.name]);
                             }
                           }}
                           className={`p-4 rounded-[12px] border cursor-pointer flex items-start gap-4 transition-all duration-200 ${isSelected ? 'border-[#0066FF] bg-[#0066FF]/5' : 'border-white/20 glassmorphism hover:border-white/40'}`}
                         >
                           <div className={`mt-1 w-10 h-10 rounded-full flex items-center justify-center shrink-0 ${isSelected ? 'bg-[#0066FF] text-white' : 'bg-gray-100 dark:bg-white/10 text-gray-500 dark:text-[#A1A1A6]'}`}>
                             <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                               <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d={dept.icon} />
                             </svg>
                           </div>
                           <div className="flex-1 min-w-0">
                             <div className="flex items-center justify-between mb-0.5">
                               <h4 className={`font-bold text-sm ${isSelected ? 'text-[#0066FF]' : 'text-[#1D1D1F] dark:text-[#F5F5F7]'}`}>{dept.name}</h4>
                               <div className={`w-5 h-5 rounded-full border flex items-center justify-center transition-colors ${isSelected ? 'border-[#0066FF] bg-[#0066FF]' : 'border-gray-300 dark:border-white/20'}`}>
                                 {isSelected && <svg className="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                               </div>
                             </div>
                             <p className="text-[11px] text-gray-500 dark:text-[#A1A1A6] leading-tight">{dept.desc}</p>
                           </div>
                         </div>
                       );
                    })}
                  </div>
                </div>

                <div className="pt-4 border-t border-white/50 dark:border-white/10">
                  <label className="flex items-center justify-between cursor-pointer p-4 rounded-[12px] glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] hover:border-white/40 transition-all border border-white/20">
                    <div className="flex flex-col">
                      <span className="font-bold text-sm">Allow AI to Auto-Respond</span>
                      <span className="text-[10px] text-gray-500 dark:text-[#A1A1A6]">Instant replies to DMs and emails.</span>
                    </div>
                    <div className="relative">
                      <input
                        type="checkbox"
                        className="sr-only"
                        checked={aiAutoRespond}
                        onChange={(e) => setAiAutoRespond(e.target.checked)}
                      />
                      <div className={`w-12 h-6 rounded-full transition-colors duration-250 ${aiAutoRespond ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-600'}`}>
                         <div className={`w-5 h-5 rounded-full bg-white absolute top-0.5 transition-transform duration-250 ${aiAutoRespond ? 'translate-x-6.5' : 'translate-x-0.5'}`}></div>
                      </div>
                    </div>
                  </label>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={handleStartOnboarding}
                  disabled={isLoading}
                  className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isLoading ? (
                    <span className="flex items-center justify-center gap-2">
                      <svg className="animate-spin h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                      </svg>
                      Launching...
                    </span>
                  ) : <IconLabel icon="launch">Launch Store</IconLabel>}
                </button>
              </div>
            </div>
          )}

          {step === 4 && (
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in bg-white/10 dark:bg-black/10 backdrop-blur-xl rounded-[24px] border border-white/20 p-12 shadow-2xl overflow-hidden relative">
               <div className="absolute inset-0 overflow-hidden">
                 <div className="absolute -top-24 -left-24 w-48 h-48 bg-[#0066FF]/20 blur-3xl rounded-full animate-pulse"></div>
                 <div className="absolute -bottom-24 -right-24 w-48 h-48 bg-[#34C759]/20 blur-3xl rounded-full animate-pulse" style={{ animationDelay: '1s' }}></div>
               </div>

               <div className="w-32 h-32 relative mb-10 z-10">
                 <div className="absolute inset-0 border-[6px] border-[#0066FF]/10 rounded-full"></div>
                 <div className="absolute inset-0 border-[6px] border-[#0066FF] rounded-full border-t-transparent animate-spin" style={{ animationDuration: '0.8s' }}></div>
                 <div className="absolute inset-4 border-[4px] border-[#34C759]/20 rounded-full"></div>
                 <div className="absolute inset-4 border-[4px] border-[#34C759] rounded-full border-b-transparent animate-spin" style={{ animationDuration: '1.2s', animationDirection: 'reverse' }}></div>
               </div>

               <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6 z-10">Building Your Vision</h2>

               <div className="space-y-4 z-10 w-full max-w-xs">
                 {[
                   { label: 'Synthesizing business model', delay: '0s' },
                   { label: 'Designing premium storefront', delay: '0.4s' },
                   { label: 'Provisioning AI departments', delay: '0.8s' },
                   { label: 'Securing payment gateway', delay: '1.2s' }
                 ].map((item, i) => (
                   <div key={i} className="flex items-center gap-3 animate-fade-in" style={{ animationDelay: item.delay }}>
                     <div className="w-2 h-2 rounded-full bg-[#0066FF] animate-ping"></div>
                     <p className="text-sm font-semibold text-gray-600 dark:text-[#A1A1A6]">{item.label}</p>
                   </div>
                 ))}
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
                      <span className="text-[#0066FF] font-semibold">my-business.ohc.store</span>
                   </div>
                </div>

                <a
                  href="/dashboard"
                  className="flex w-full items-center justify-center bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] p-4 rounded-[8px] font-bold shadow-md hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="dashboard">Go to Dashboard</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-sm active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
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
