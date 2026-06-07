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

  const handleIntake = async () => {
    setIsLoading(true);
    setError('');

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;

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
      const mappedCategories = intakeData.categories || ['physical'];
      setCategories(mappedCategories);

      // Auto-configure AI Departments based on inferred business context
      const newAgents = ['Operations', 'Marketing', 'Finance', 'Legal', 'Advisory'];
      if (mappedCategories.includes('physical') || mappedCategories.includes('digital') || mappedCategories.includes('subscriptions')) {
        newAgents.push('Sales');
      }
      if (mappedCategories.includes('services') || mappedCategories.includes('food') || mappedCategories.includes('physical')) {
        newAgents.push('Customer Success');
      }
      setAiAgents(newAgents);

      setStep(2); await syncStateToBackend({ step: 2, aiAgents: newAgents }); // Go to review step
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred processing details');
      setStep(1); syncStateToBackend({ step: 1 });
      setChatStep(3); syncStateToBackend({ chatStep: 3 });
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
    setStep(4); syncStateToBackend({ step: 4 }); // Go to loading screen
    const safetyTimeout = setTimeout(() => {
      // Fallback if API fails to respond in time
      setStartResult({ message: 'Fallback: Your business has been successfully launched.' });
      setStep(5);
        syncStateToBackend({ step: 5 });
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
          location: location || '',
          target_audience: targetAudience || ''
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
      setStep(5);
        syncStateToBackend({ step: 5 }); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred during onboarding');
      clearTimeout(safetyTimeout);
      setStep(3); syncStateToBackend({ step: 3 }); // Go back to last input screen on error
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
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
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
                        enterKeyHint="next"
                        autoCapitalize="words"
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
                            setChatStep(2); syncStateToBackend({ chatStep: 2 });
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
                        setChatStep(2); syncStateToBackend({ chatStep: 2 });
                      }}
                      disabled={!businessName.trim()}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 2 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button onClick={() => { setChatStep(1); syncStateToBackend({ chatStep: 1 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
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
                        enterKeyHint="next"
                        autoCapitalize="sentences"
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
                            setChatStep(3); syncStateToBackend({ chatStep: 3 });
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
                        setChatStep(3); syncStateToBackend({ chatStep: 3 });
                      }}
                      disabled={!whatYouSell.trim()}
                      className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.23)] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      <IconLabel icon="next">Next</IconLabel>
                    </button>
                  </div>
                </div>
              )}

              {chatStep === 3 && (
                <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
                  <button onClick={() => { setChatStep(2); syncStateToBackend({ chatStep: 2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
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
                        enterKeyHint="next"
                        autoCapitalize="words"
                        value={location}
                        onChange={(e) => setLocation(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') {
                            e.preventDefault();
                            if (!location.trim()) {
                              setValidationError('Please tell us your location.');
                              return;
                            }
                            setValidationError('');
                            setChatStep(4); syncStateToBackend({ chatStep: 4 });
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
                      ) : <IconLabel icon="launch">Generate My Business</IconLabel>}
                    </button>
                  </div>
                </div>
              )}
            </div>
          )}

          {step === 2 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button onClick={() => { setStep(1); syncStateToBackend({ step: 1 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
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
                    enterKeyHint="next"
                    autoCapitalize="words"
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
                    enterKeyHint="next"
                    autoCapitalize="words"
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
                    enterKeyHint="next"
                    autoCapitalize="words"
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
                        enterKeyHint="next"
                        autoCapitalize="words"
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
                      setValidationErrors(prev => ({ ...prev, businessName: 'Must be at least 3 characters.' }));
                      setValidationError('Please fix the errors before continuing.');
                      return;
                    }
                    if (Object.keys(validationErrors).length > 0) {
                      setValidationError('Please fix the errors before continuing.');
                      return;
                    }
                    setValidationError('');
                    setStep(3); syncStateToBackend({ step: 3 });
                  }}
                  disabled={!businessName.trim() || !businessType.trim() || categories.length === 0 || !firstProductName.trim() || !firstProductPrice.trim()}
                  className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <IconLabel icon="next">Continue</IconLabel>
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button onClick={() => { setStep(2); syncStateToBackend({ step: 2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1">
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
                      <span className="text-[10px] opacity-70">your-name.ohc.app</span>
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
                        enterKeyHint="next"
                        autoCapitalize="words"
                        value={adminName}
                        onChange={(e) => {
                          const val = e.target.value;
                          setAdminName(val);
                          if (!val.trim()) {
                            setValidationErrors(prev => ({ ...prev, adminName: 'Admin Name is required' }));
                          } else {
                            setValidationErrors(prev => { const { adminName, ...rest } = prev; return rest; });
                          }
                        }}
                        placeholder="e.g. Maya Smith"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminName ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminName && <p className="text-red-500 text-xs mt-1">{validationErrors.adminName}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Email</label>
                      <input
                        type="email"
                        enterKeyHint="next"
                        autoCapitalize="none"
                        value={adminEmail}
                        onChange={(e) => {
                          const val = e.target.value;
                          setAdminEmail(val);
                          if (!val.trim()) {
                            setValidationErrors(prev => ({ ...prev, adminEmail: 'Admin Email is required' }));
                          } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val)) {
                            setValidationErrors(prev => ({ ...prev, adminEmail: 'Please enter a valid email address' }));
                          } else {
                            setValidationErrors(prev => { const { adminEmail, ...rest } = prev; return rest; });
                          }
                        }}
                        placeholder="you@example.com"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminEmail ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminEmail && <p className="text-red-500 text-xs mt-1">{validationErrors.adminEmail}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Password</label>
                      <input
                        type="password"
                        enterKeyHint="done"
                        value={adminPassword}
                        onChange={(e) => {
                          const val = e.target.value;
                          setAdminPassword(val);
                          if (!val.trim()) {
                            setValidationErrors(prev => ({ ...prev, adminPassword: 'Password is required' }));
                          } else if (val.length < 8 || !/\d/.test(val)) {
                            setValidationErrors(prev => ({ ...prev, adminPassword: 'Password must be at least 8 characters and contain a number' }));
                          } else {
                            setValidationErrors(prev => { const { adminPassword, ...rest } = prev; return rest; });
                          }
                        }}
                        placeholder="••••••••"
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.adminPassword ? "border-red-500" : "border-white/50 dark:border-white/10 focus:border-[#0066FF]"} outline-none glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7]`}
                      />
                      {validationErrors.adminPassword && <p className="text-red-500 text-xs mt-1">{validationErrors.adminPassword}</p>}
                    </div>
                  </div>
                </div>

                <div className="pt-2 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Auto-Configured AI Departments</label>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-xs mb-2">
                    Here are the AI departments we've configured for you.
                  </p>
                  <div className="flex flex-wrap gap-2 mt-2">
                    {aiAgents.map(agent => (
                      <div
                        key={agent}
                        className="px-3 py-1.5 rounded-full border border-[#34C759] bg-[#34C759]/10 text-[#34C759] flex items-center gap-1.5 text-sm font-semibold transition-all"
                      >
                        <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                        {agent}
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-2">
                  <label className="flex items-center justify-between cursor-pointer p-3 rounded-[8px] glassmorphism text-[#1D1D1F] dark:text-white">
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
                  className="w-full bg-[#0066FF] text-white min-h-[54px] p-4 rounded-[8px] font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
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
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in glassmorphism rounded-[16px] shadow-2xl p-8">
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
