"use client";

import React, { useEffect, useState } from 'react';
import { useOnboardingStore } from './store';
import { SmartBlock, DraggableBlock } from '../builder/components';
import { useWalkthrough } from '../../components/help';
import { WithTooltip } from '../../components/TooltipRegistry';
import { AppShell } from '../components/AppShell';

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

export function LegacyChatWizard() {
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
    userName, setUserName,
    adminEmail, setAdminEmail,
    adminPassword, setAdminPassword,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
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
        throw new Error('Failed to save draft');
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
        if (data.wizardState.adminEmail) setAdminEmail(data.wizardState.adminEmail);
        if (data.wizardState.adminPassword) setAdminPassword(data.wizardState.adminPassword);
        if (data.wizardState.domainChoice) setDomainChoice(data.wizardState.domainChoice);
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
      domainChoice,
      firstProductName,
      firstProductPrice,
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
    adminEmail, adminPassword, aiAgents, aiAutoRespond, isLoaded
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
          admin_email: adminEmail || 'admin@ohc.app',
          admin_name: businessName + ' Admin',
          admin_password: adminPassword || 'password123',
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: domainChoice || 'subdomain',
          price_type: 'fixed',
          location: location || ''
        })
      });

      const result = await startRes.json();
      if (!startRes.ok) {
        throw new Error(result.error || result.message || 'Failed to start onboarding');
      }

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

  // Progress percentage calculation
  const getProgress = () => {
    if (step === 1) return (chatStep / 3) * 33;
    if (step === 2) return 50;
    if (step === 3) return 75;
    if (step === 4) return 90;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <AppShell
      title="Setup"
      subtitle="Guided business setup in the same operations-console layout."
      statusItems={[
        { label: "Step", value: `${step}/5`, tone: "neutral" },
        { label: "Progress", value: `${Math.round(getProgress())}%`, tone: step === 5 ? "good" : "warn" },
      ]}
      actions={[{ label: "Dashboard", href: "/dashboard" }]}
    >
      <div className="app-grid two">
        <div id="setup-screen" className="app-panel w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[16px] mac-glass-container">
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
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
                Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
              </p>

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
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
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
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all shadow-inner"
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
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner"
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
                    className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.businessName ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]`}
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
                        setValidationErrors(prev => ({ ...prev, businessType: 'Required field.' }));
                      } else {
                        setValidationErrors(prev => { const { businessType, ...rest } = prev; return rest; });
                      }
                    }}
                    className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.businessType ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]`}
                  />
                  {validationErrors.businessType && <p className="text-red-500 text-xs mt-1">{validationErrors.businessType}</p>}
                </div>
                <div>
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Categories (Comma separated)</label>
                  <input
                    type="text"
                    value={categories.join(', ')}
                    onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                    className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                  />
                </div>
                <div className="grid grid-cols-2 gap-2">
                   <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">First Product</label>
                      <input
                        type="text"
                        value={firstProductName}
                        onChange={(e) => setFirstProductName(e.target.value)}
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
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
                              setValidationErrors(prev => ({ ...prev, firstProductPrice: 'Required field.' }));
                           } else if (!/^\d+(\.\d{1,2})?$/.test(e.target.value)) {
                              setValidationErrors(prev => ({ ...prev, firstProductPrice: 'Invalid price.' }));
                           } else {
                              setValidationErrors(prev => { const { firstProductPrice, ...rest } = prev; return rest; });
                           }
                        }}
                        className={`w-full p-3 sm:p-4 rounded-[8px] border ${validationErrors.firstProductPrice ? 'border-red-500' : 'border-white/50 dark:border-white/10 focus:border-[#0066FF]'} outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]`}
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
                        className={`p-3 rounded-[8px] border cursor-pointer transition-all ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
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
                      className={`p-3 rounded-[8px] border cursor-pointer transition-all flex flex-col items-center justify-center text-center ${domainChoice === 'subdomain' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
                    >
                      <span className="font-semibold text-sm mb-1">Free Subdomain</span>
                      <span className="text-[10px] opacity-70">your-name.ohc.store</span>
                    </div>
                    <div
                      onClick={() => setDomainChoice('custom')}
                      className={`p-3 rounded-[8px] border cursor-pointer transition-all flex flex-col items-center justify-center text-center ${domainChoice === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
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
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Email</label>
                      <input
                        type="email"
                        value={adminEmail}
                        onChange={(e) => setAdminEmail(e.target.value)}
                        placeholder="you@example.com"
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Password</label>
                      <input
                        type="password"
                        value={adminPassword}
                        onChange={(e) => setAdminPassword(e.target.value)}
                        placeholder="••••••••"
                        className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7]"
                      />
                    </div>
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
                           className={`p-3 rounded-[8px] border cursor-pointer flex items-center justify-between transition-all ${isSelected ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 mac-glass-container text-[#1D1D1F] dark:text-white'}`}
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
                  <label className="flex items-center justify-between cursor-pointer p-3 rounded-[8px] mac-glass-container text-[#1D1D1F] dark:text-white">
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
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
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
                <div className="p-3 mac-glass-container rounded-[8px] flex flex-col items-center mb-6">
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
                  className="flex w-full items-center justify-center mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[8px] font-bold shadow-sm active:scale-[0.98] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="eye">Preview Storefront</IconLabel>
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
        <aside className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Setup Progress</div>
              <div className="app-list-subtitle">This panel now lives in the same side-menu application frame.</div>
            </div>
          </div>
          <div className="app-list">
            {[
              ['Business intake', step > 1],
              ['Review details', step > 2],
              ['Style and team', step > 3],
              ['Launch', step > 4],
            ].map(([label, complete]) => (
              <div key={String(label)} className="app-list-item">
                <div>
                  <div className="app-list-title">{label}</div>
                  <div className="app-list-subtitle">{complete ? 'Complete' : 'Pending'}</div>
                </div>
                <span className={`app-badge ${complete ? 'good' : ''}`}>{complete ? 'Done' : 'Open'}</span>
              </div>
            ))}
          </div>
        </aside>
      </div>
    </AppShell>
  );
}


// DEPRECATED: Moved to onboarding/page.tsx

export function StepByStepWizard() {

  const {
    step, setStep,
    businessName, setBusinessName,
    businessType, setBusinessType,
    hasPhysicalProducts, setHasPhysicalProducts,
    hasDigitalProducts, setHasDigitalProducts,
    firstProductName: productName, setFirstProductName: setProductName,
    firstProductPrice: productPrice, setFirstProductPrice: setProductPrice,
    paymentMethod, setPaymentMethod,
    userName, setUserName,
    adminEmail, setAdminEmail,
    adminPassword, setAdminPassword,
    template, setTemplate,
    bio, setBio,
    domainChoice, setDomainChoice,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
    blocks, setBlocks,
    status, setStatus,
    liveUrl, setLiveUrl
  } = useOnboardingStore();


  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [tenantId, setTenantId] = useState("storefront");
  const [saveMessage, setSaveMessage] = useState("");

  const handleSaveDraft = async () => {
    setStatus("generating"); // Just show some loading state or disable button
    try {
      const state = useOnboardingStore.getState();
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const user = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenant,
          'x-user-id': user
        },
        body: JSON.stringify({
          wizardState: state
        })
      });

      if (res.ok) {
        setSaveMessage("Draft Saved!");
        setTimeout(() => setSaveMessage(""), 3000);
      }
    } catch (e) {
      console.error('Failed to save draft', e);
    } finally {
      setStatus("idle");
    }
  };












  const { startWalkthrough } = useWalkthrough();

  // Read state from server on mount
  useEffect(() => {
    const tenantIdStr = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
    setTenantId(tenantIdStr);
    const userId = localStorage.getItem('user_id') || 'test-user';
    fetch('/api/onboarding/state', {
      headers: { 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userId }
    })
    .then(res => res.json())
    .then(data => {
      if (data && data.builderState) {
        if (data.builderState.bio) setBio(data.builderState.bio);
        if (data.builderState.blocks && Array.isArray(data.builderState.blocks)) setBlocks(data.builderState.blocks);
        if (data.builderState.status) setStatus(data.builderState.status);
      }
      if (data && data.wizardState) {
        if (data.wizardState.step !== undefined) setStep(data.wizardState.step);
        if (data.wizardState.businessName) setBusinessName(data.wizardState.businessName);
        if (data.wizardState.businessType) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.hasPhysicalProducts !== undefined) setHasPhysicalProducts(data.wizardState.hasPhysicalProducts);
        if (data.wizardState.hasDigitalProducts !== undefined) setHasDigitalProducts(data.wizardState.hasDigitalProducts);
        if (data.wizardState.productName) setProductName(data.wizardState.productName);
        if (data.wizardState.productPrice) setProductPrice(data.wizardState.productPrice);
        if (data.wizardState.paymentMethod) setPaymentMethod(data.wizardState.paymentMethod);
        if (data.wizardState.adminEmail) setAdminEmail(data.wizardState.adminEmail);
        if (data.wizardState.adminEmail) setAdminEmail(data.wizardState.adminEmail);
        if (data.wizardState.adminPassword) setAdminPassword(data.wizardState.adminPassword);
        if (data.wizardState.template) setTemplate(data.wizardState.template);
        if (data.wizardState.bio) setBio(data.wizardState.bio);
        if (data.wizardState.domainChoice) setDomainChoice(data.wizardState.domainChoice);
        if (data.wizardState.aiAgents) setAiAgents(data.wizardState.aiAgents);
        if (data.wizardState.aiAutoRespond !== undefined) setAiAutoRespond(data.wizardState.aiAutoRespond);
      }
    })
    .catch(err => console.error('Failed to load builder state', err));
  }, []);

  // Sync full state to backend
  useEffect(() => {
    // Only save if there's actual state
    if (step !== 0 || bio !== '' || blocks.length > 0 || businessName !== '') {
      const tenantIdStr = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
      const userId = localStorage.getItem('user_id') || 'test-user';

      const wizardState = {
        step: step,
        businessName,
        businessType,
        hasPhysicalProducts,
        hasDigitalProducts,
        productName,
        productPrice,
        paymentMethod,
        adminEmail,
        adminEmail,
        adminPassword,
        template,
        bio,
        domainChoice,
        aiAgents,
        aiAutoRespond
      };

      const payload = {
        builderState: { bio, blocks, status },
        wizardState
      };

      const timer = setTimeout(() => {
        fetch('/api/onboarding/state', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantIdStr, 'X-User-ID': userId },
          body: JSON.stringify(payload)
        }).catch(err => console.error('Failed to sync builder state', err));
      }, 1000); // debounce 1s

      return () => clearTimeout(timer);
    }
  }, [step, businessName, businessType, hasPhysicalProducts, hasDigitalProducts, productName, productPrice, paymentMethod, adminEmail, adminEmail, adminPassword, template, bio, domainChoice, aiAgents, aiAutoRespond, blocks, status]);



  const updateStatus = (newStatus: "idle" | "generating" | "draft" | "live") => {
    setStatus(newStatus);
    localStorage.setItem("ohc_builder_status", newStatus);
  };

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: bio })
      });

      const data = await response.json();
      const blocks = data.pages[0].blocks.map((b: any) => ({
        type: b.block_type === 'HeroBlock' ? 'Hero' :
              b.block_type === 'ProductGridBlock' ? 'Catalog' :
              b.block_type === 'ServiceBookingBlock' ? 'Booking' :
              b.block_type === 'TestimonialBlock' ? 'Testimonials' : b.block_type,
        props: b.content
      }));
      setBlocks(blocks);
      localStorage.setItem("ohc_builder_blocks", JSON.stringify(blocks));
      updateStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      updateStatus("idle");
    }
  };

  const moveBlock = (fromIndex: number, toIndex: number) => {
    if (toIndex < 0 || toIndex >= blocks.length || fromIndex === toIndex) return;

    setBlocks(prev => {
      const newBlocks = [...prev];
      const [moved] = newBlocks.splice(fromIndex, 1);
      newBlocks.splice(toIndex, 0, moved);
      localStorage.setItem("ohc_builder_blocks", JSON.stringify(newBlocks));
      return newBlocks;
    });

    if (selectedBlockIndex === fromIndex) {
      setSelectedBlockIndex(toIndex);
    } else if (selectedBlockIndex === toIndex) {
      setSelectedBlockIndex(fromIndex);
    }
  };

  const handleLaunch = async () => {
    try {
      const draftBlocks = blocks.map((b, i) => ({
        block_type: b.type === 'Hero' ? 'HeroBlock' :
                    b.type === 'Catalog' ? 'ProductGridBlock' :
                    b.type === 'Booking' ? 'ServiceBookingBlock' :
                    b.type === 'Testimonials' ? 'TestimonialBlock' : b.type,
        content: b.props,
        sort_order: i
      }));

      const payload = {
          domain: null,
          draft: {
              domain: null,
              pages: [{
                  path: '/',
                  title: 'Home',
                  blocks: draftBlocks,
                  seo_metadata: {
                    "@context": "https://schema.org",
                    "@type": "LocalBusiness",
                    "name": bio
                  }
              }]
          }
      };

      const response = await fetch('/api/v1/builder/publish_draft', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
      });
      if (response.ok) {
        const data = await response.json();
        setStatus("live");
        const url = `https://${data.domain || 'myshop'}.ohc.store`;
        setLiveUrl(url);
        localStorage.setItem("ohc_builder_liveUrl", url);
      } else {
        console.error('Failed to publish');
      }
    } catch (error) {
      console.error('Error publishing:', error);
    }
  };

  if (status === "idle") {
    const handleBack = () => {
      if (step === 1) setStep(0);
      else if (step === 2) setStep(1);
      else if (step === 3) setStep(2);
      else if (step === 4) setStep(3);
      else if (step === 5) setStep(4);
      else if (step === 6) setStep(5);
      else if (step === 7) setStep(6);
      else if (step === '7.5') setStep(7);
      else if (step === 8) setStep('7.5');
      else if (step === '8.5') setStep(8);
      else if (step === 9) setStep('8.5');
      else if (step === 'instant-build') setStep(0);
    };

    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] bg-fixed">
      {/* Background Glows for Premium Aesthetic */}
      <div className="fixed top-[-10%] left-[-10%] w-[40%] h-[40%] bg-[#0066FF]/10 blur-[120px] rounded-full pointer-events-none"></div>
      <div className="fixed bottom-[-10%] right-[-10%] w-[40%] h-[40%] bg-[#34C759]/10 blur-[120px] rounded-full pointer-events-none"></div>


        <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container">

          <div className="px-8 pb-8 pt-8 flex flex-col flex-1 justify-start overflow-y-auto relative">
            {step !== 0 && (
              <button
                onClick={handleBack}
                className="absolute top-6 left-8 text-[#0071E3] font-medium text-sm hover:underline transition-all z-10 flex items-center gap-1 bg-white/50 backdrop-blur-md px-3 py-1 rounded-full shadow-sm border border-white/20"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                Back
              </button>
            )}


            <div className="absolute top-6 right-8 flex items-center gap-4 z-10">
              {saveMessage && <span className="text-[#34C759] text-sm font-semibold animate-fade-in">{saveMessage}</span>}
              {step !== 0 && step !== 'instant-build' && (
                <button
                  onClick={handleSaveDraft}
                  className="text-[#0071E3] font-medium text-sm hover:underline transition-all bg-white/50 backdrop-blur-md px-3 py-1 rounded-full shadow-sm border border-white/20"
                >
                  Save Draft
                </button>
              )}
            </div>

            <div className={`animate-fade-in ${step !== 0 ? 'mt-10' : 'mt-4'}`} style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>


              {step === 0 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">10-Minute Setup Wizard</h1>
                  <h2 className="text-xl font-semibold font-outfit text-gray-800 dark:text-[#e5e5e7] mb-2">Your business, live in minutes.</h2>
                  <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                    Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
                  </p>

                  <div className="flex flex-col gap-4">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setStep(1)}
                    >
                      Start My Business
                    </button>

                    <button
                      className="w-full bg-white text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setStep('instant-build')}
                    >
                      Instant Build
                    </button>
                  </div>
                </>
              )}

              {step === 1 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What kind of business are you building?</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setBusinessType('Online Store'); setStep(2); }}
                    >
                      Online Store
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setBusinessType('Restaurant'); setStep(2); }}
                    >
                      Restaurant
                    </button>
                  </div>
                </>
              )}

              {step === 2 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Give your business a name</h1>
                  <div id="step-3" className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="What is your business called?"
                      value={businessName}
                      onChange={(e) => setBusinessName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="e.g. Maya's Cakes"
                      value={bio}
                      onChange={(e) => setBio(e.target.value)}
                    />
                    <button
                      disabled={!(businessName || '').trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setStep(3)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 3 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">What do you sell?</h1>
                  <div id="step-4" className="mt-6 flex flex-col gap-4">
                    <label className="flex items-center gap-3 p-4 mac-glass-container rounded-[8px] cursor-pointer hover:bg-white/60 dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0071E3]"
                        checked={hasPhysicalProducts}
                        onChange={(e) => setHasPhysicalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Physical Products</span>
                    </label>
                    <label className="flex items-center gap-3 p-4 mac-glass-container rounded-[8px] cursor-pointer hover:bg-white/60 dark:hover:bg-white/10 text-[#1D1D1F] dark:text-[#F5F5F7]">
                      <input
                        type="checkbox"
                        className="w-5 h-5 accent-[#0071E3]"
                        checked={hasDigitalProducts}
                        onChange={(e) => setHasDigitalProducts(e.target.checked)}
                      />
                      <span className="font-semibold text-gray-800">Digital Products</span>
                    </label>
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setStep(4)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 4 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Product details</h1>
                  <div id="step-5" className="mt-6 flex flex-col gap-4">
                    <input
                      type="text"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="What is the name of this product?"
                      value={productName}
                      onChange={(e) => setProductName(e.target.value)}
                    />
                    <input
                      type="text"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="0.00"
                      value={productPrice}
                      onChange={(e) => setProductPrice(e.target.value)}
                    />
                    <button
                      disabled={!(productName || '').trim() || !(productPrice || '').trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setStep(5)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 5 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">How do you want to receive payments?</h1>
                  <div className="mt-6 flex flex-col gap-4">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setPaymentMethod('Online'); setStep(6); }}
                    >
                      Online
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setPaymentMethod('In Person'); setStep(6); }}
                    >
                      In Person
                    </button>
                  </div>
                </>
              )}

              {step === 6 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Create your account</h1>
                  <div id="step-7" className="mt-6 flex flex-col gap-4">
                                        <input
                      type="text"
                      className="w-full p-3 sm:p-4 rounded-[8px] focus:border-[#0066FF] outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] shadow-inner"
                      placeholder="e.g. Maya Smith"
                      value={userName}
                      onChange={(e) => setUserName(e.target.value)}
                    />
                    <input
                      type="email"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="you@email.com"
                      value={adminEmail}
                      onChange={(e) => setAdminEmail(e.target.value)}
                    />
                    <input
                      type="password"
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="Password"
                      value={adminPassword}
                      onChange={(e) => setAdminPassword(e.target.value)}
                    />
                    <button
                      disabled={!adminEmail.trim() || !adminEmail.trim() || !adminPassword.trim()}
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4 disabled:opacity-50 disabled:cursor-not-allowed"
                      onClick={() => setStep(7)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 7 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Template selection</h1>
                  <div id="step-8" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setTemplate('Modern'); setStep('7.5'); }}
                    >
                      Modern
                    </button>
                    <button
                      className="w-full text-[#1D1D1F] dark:text-[#F5F5F7] mac-glass-container p-4 font-bold rounded-[8px] shadow-sm hover:bg-white/60 dark:hover:bg-white/10 transition-all text-left"
                      onClick={() => { setTemplate('Bold'); setStep('7.5'); }}
                    >
                      Bold
                    </button>
                  </div>
                </>
              )}

              {step === '7.5' && (
                <>
                  <div id="step-8" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setStep(8)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 8 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Choose your domain</h1>
                  <div id="step-9" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => setStep('8.5')}
                    >
                      Free OHC Domain
                    </button>
                    <button
                      className="w-full bg-white text-[#0071E3] border border-[#0071E3] p-4 font-bold rounded-[8px] shadow-sm hover:bg-blue-50 transition-all"
                      onClick={() => setStep('8.5')}
                    >
                      Connect Custom Domain
                    </button>
                  </div>
                </>
              )}

              {step === '8.5' && (
                <>
                  <div id="step-9" className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all mt-4"
                      onClick={() => setStep(9)}
                    >
                      Next
                    </button>
                  </div>
                </>
              )}

              {step === 9 && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Review your choices</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => {
                        setStatus('generating');
                        setTimeout(() => {
                           setStatus('live');
                        }, 2000);
                      }}
                    >
                      Publish my business
                    </button>
                  </div>
                </>
              )}

              {step === 'instant-build' && (
                <>
                  <h1 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Describe your business in a sentence</h1>
                  <div className="flex flex-col gap-4 mt-6">
                    <textarea
                      className="w-full mac-glass-container p-4 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800 dark:text-[#f5f5f7] shadow-inner"
                      style={{ borderRadius: '8px' }}
                      placeholder="e.g. I run a local bakery"
                      rows={4}
                    />
                    <button
                      className="w-full bg-[#0071E3] text-white p-4 font-bold rounded-[8px] shadow-md hover:bg-[#005bb5] transition-all"
                      onClick={() => {
                        setStatus('generating');
                        setTimeout(() => {
                           setStatus('live');
                        }, 2000);
                      }}
                    >
                      Generate Storefront
                    </button>
                  </div>
                </>
              )}

            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] bg-fixed">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden justify-center items-center mac-glass-container">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
            <p className="text-gray-500 dark:text-[#a1a1a6] font-medium">Agents are building your store...</p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] bg-fixed">
        <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden text-center p-8 justify-center mac-glass-container">
          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-[#f5f5f7] mb-2">Success! Your business is live!</h1>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">Your automated storefront is successfully published.</p>
          <p className="text-gray-500 dark:text-[#a1a1a6] mb-6 text-sm">You're set up! Here's what to do next:</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 dark:text-[#a1a1a6] truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          <button
            className="w-full bg-[#0071E3] text-white font-bold p-4 active:scale-[0.98] transition-all hover:bg-[#005bb5]"
            style={{ borderRadius: '8px' }}
          >
            View Welcome Checklist
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8 sm:px-6 lg:px-8 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] bg-fixed">
      <div className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto min-h-[100dvh] sm:min-h-[812px] shadow-2xl flex flex-col relative rounded-[16px] overflow-hidden mac-glass-container">
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {Array.isArray(blocks) && blocks.map((b, i) => (
            <DraggableBlock
              key={b.type + i}
              isSelected={selectedBlockIndex === i}
              onClick={() => setSelectedBlockIndex(i === selectedBlockIndex ? null : i)}
              onDragStart={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.effectAllowed = 'move';
                  (e as React.DragEvent).dataTransfer.setData('text/plain', i.toString());
                }
                setDraggedIndex(i);
                setSelectedBlockIndex(i);
              }}
              onDragOver={(e) => {
                if (e.type.includes('drag') && (e as React.DragEvent).dataTransfer) {
                  (e as React.DragEvent).dataTransfer.dropEffect = 'move';
                }
              }}
              onDragEnter={() => {
                if (draggedIndex !== null && draggedIndex !== i) {
                  moveBlock(draggedIndex, i);
                  setDraggedIndex(i);
                }
              }}
              onDragEnd={() => setDraggedIndex(null)}
              onMoveUp={i > 0 ? () => moveBlock(i, i - 1) : undefined}
              onMoveDown={i < blocks.length - 1 ? () => moveBlock(i, i + 1) : undefined}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          {/* Default to false for premium status here. In a full implementation, we'd fetch this from the user's profile. */}
          <SmartBlock type="PoweredBy" props={{ tenantId, isPremium: false }} />
        </div>

        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50" style={{ borderRadius: '0 0 16px 16px' }}>
          <WithTooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              style={{ borderRadius: '8px' }}
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </WithTooltip>
        </div>
      </div>
    </div>
  );
}


export default function OnboardingWizard() { return <StepByStepWizard />; }