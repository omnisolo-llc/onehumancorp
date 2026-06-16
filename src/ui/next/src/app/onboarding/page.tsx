"use client";

import React, { useEffect, useState, useRef } from 'react';
import { useRouter } from 'next/navigation';
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
  const router = useRouter();
  const {
    step, setStep,
    chatStep, setChatStep,
    businessDescription, setBusinessDescription,
    businessGoal, setBusinessGoal,
    businessName, setBusinessName,
    whatYouSell, setWhatYouSell,
    location, setLocation,
    targetAudience, setTargetAudience,
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
    startResult, setStartResult,
    instantImageUrl, setInstantImageUrl
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const initialStateLoaded = useRef(false);
  const [chatMessages, setChatMessages] = useState<{role: string, content: string, image_url?: string}[]>([]);
  const [chatInput, setChatInput] = useState('');
  const [chatImageUrl, setChatImageUrl] = useState('');
  const chatMessagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    chatMessagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [chatMessages]);

  const fetchWithRetry = async (url: string, options: RequestInit, retries = 3, backoff = process.env.NODE_ENV === 'test' ? 10 : 500) => {
    for (let i = 0; i < retries; i++) {
      try {
        const response = await fetch(url, options);
        if (!response.ok) {
           let errMsg = `HTTP error! status: ${response.status}`;
           try {
              const result = await response.clone().json();
              errMsg = result.error || result.message || errMsg;
           } catch (e) {}
           throw new Error(errMsg);
        }
        return response;
      } catch (err: any) {
        if (i === retries - 1) throw err;
        await new Promise(res => setTimeout(res, backoff * Math.pow(2, i)));
      }
    }
    throw new Error('Max retries reached');
  };

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

  const handleSkipSetup = () => {
    setError('');
    setValidationError('');
    localStorage.setItem('has_onboarded', 'true');
    syncStateToBackend({ skipped: true });
    router.push('/dashboard');
  };

  const handleBackToIntro = () => {
    setError('');
    setValidationError('');
    setValidationErrors({});
    setStep(0);
    syncStateToBackend({ step: 0 });
  };

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

      const res = await fetchWithRetry('/api/onboarding/draft', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ wizardState })
      });

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
        if (data.wizardState.step !== undefined) setStep(data.wizardState.step >= 4 ? data.wizardState.step : 0);
        if (data.wizardState.chatStep !== undefined) setChatStep(data.wizardState.chatStep);
        if (data.wizardState.businessDescription !== undefined) setBusinessDescription(data.wizardState.businessDescription);
        if (data.wizardState.businessName !== undefined) setBusinessName(data.wizardState.businessName);
        if (data.wizardState.whatYouSell !== undefined) setWhatYouSell(data.wizardState.whatYouSell);
        if (data.wizardState.location !== undefined) setLocation(data.wizardState.location);
        if (data.wizardState.targetAudience !== undefined) setTargetAudience(data.wizardState.targetAudience);
        if (data.wizardState.businessType !== undefined) setBusinessType(data.wizardState.businessType);
        if (data.wizardState.categories !== undefined) setCategories(data.wizardState.categories);
        if (data.wizardState.websiteTemplate !== undefined) setWebsiteTemplate(data.wizardState.websiteTemplate);
        if (data.wizardState.firstProductName !== undefined) setFirstProductName(data.wizardState.firstProductName);
        if (data.wizardState.firstProductPrice !== undefined) setFirstProductPrice(data.wizardState.firstProductPrice);
        if (data.wizardState.adminName !== undefined) setAdminName(data.wizardState.adminName);
        if (data.wizardState.adminEmail !== undefined) setAdminEmail(data.wizardState.adminEmail);
        if (data.wizardState.adminPassword !== undefined) setAdminPassword(data.wizardState.adminPassword);
        if (data.wizardState.domainChoice !== undefined) setDomainChoice(data.wizardState.domainChoice);
        if (data.wizardState.aiAgents !== undefined) setAiAgents(data.wizardState.aiAgents);
        if (data.wizardState.aiAutoRespond !== undefined) setAiAutoRespond(data.wizardState.aiAutoRespond);
        initialStateLoaded.current = true;
      }
    })
    .catch(err => console.error('Failed to load onboarding state', err))
    .finally(() => {
      initialStateLoaded.current = true;
      setIsLoaded(true);
    });
  }, []);

  // Sync state to backend
  useEffect(() => {
    if (!isLoaded || !initialStateLoaded.current) return;

    // Only save if we are past the initial state
    if (step === 0 && !bio) return;

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


  const handleInstantBuild = async () => {
    if (!bio.trim()) {
      setError('Please tell us about your business.');
      return;
    }
    if (!adminEmail.trim() || !adminPassword.trim()) {
      setError('Admin Email and Password are required.');
      return;
    }

    setIsLoading(true);
    setError('');
    setStep(4); syncStateToBackend({ step: 4 });

    try {
      const backendUrl = (typeof window !== 'undefined' && (window.location.origin.includes('localhost') || window.location.protocol === 'file:')) ? 'http://127.0.0.1:18789' : '';
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      let combinedInput = bio;
      if (instantImageUrl) {
        combinedInput += `\nImage provided: ${instantImageUrl}`;
      }

      const intakeRes = await fetchWithRetry(`${backendUrl}/api/onboarding/intake`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: combinedInput, image_url: instantImageUrl })
      });
      const intakeData = await intakeRes.json();

      setBusinessName(intakeData.business_name || 'My Business');
      setBusinessType(intakeData.business_type || 'Online Store');
      setBusinessDescription(bio);
      setCategories(intakeData.categories || ['physical']);
      setFirstProductName(intakeData.initial_products?.[0]?.name || 'First Product');
      setFirstProductPrice(intakeData.initial_products?.[0]?.price || '0.00');
      setLocation(intakeData.location || 'Local');
      setTargetAudience(intakeData.target_audience || 'General');
      setAdminName(intakeData.business_name || 'Admin');
      setAdminEmail('admin@mybusiness.com');
      setAdminPassword('password123');
      setDomainChoice('subdomain');
      setWebsiteTemplate('auto');

      setTimeout(() => {
        handleStartOnboarding(intakeData);
      }, 100);
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'Failed to generate your business');
      setStep(0); syncStateToBackend({ step: 0 });
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async (intakeDataOverride?: any) => {
    if (intakeDataOverride && intakeDataOverride.business_name) {
      setIsLoading(true);
      setError('');
      setStep(4); syncStateToBackend({ step: 4 });
      try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
        const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';
        const startRes = await fetchWithRetry('/api/onboarding/start', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
          body: JSON.stringify({
            business_type: intakeDataOverride.business_type || 'Online Store',
            company_name: intakeDataOverride.business_name || 'My Business',
            company_description: bio,
            selling_categories: intakeDataOverride.categories || ['physical'],
            payment_pref: 'online',
            admin_email: adminEmail || 'admin@mybusiness.com',
            admin_name: intakeDataOverride.business_name || 'Admin',
            admin_password: adminPassword || 'password123',
            website_template: 'auto',
            first_product_name: intakeDataOverride.initial_products?.[0]?.name || 'First Product',
            first_product_price: intakeDataOverride.initial_products?.[0]?.price || '0.00',
            domain_choice: 'subdomain',
            price_type: 'fixed',
            location: intakeDataOverride.location || 'Local',
            target_audience: intakeDataOverride.target_audience || 'General',
            ai_agents: [],
            ai_auto_respond: true,
            initial_products: intakeDataOverride.initial_products || []
          })
        });

        const result = await startRes.json().catch(() => ({}));
        if (!startRes.ok) throw new Error(result.error || result.message || 'Failed to start onboarding');

        await new Promise(resolve => setTimeout(resolve, 500));
        setStartResult(result);
        localStorage.setItem('has_onboarded', 'true');
        if (result.organization_id) {
          localStorage.setItem('tenant_id', result.organization_id);
          localStorage.setItem('tenant', result.organization_id);
        }
        setStep(5); syncStateToBackend({ step: 5 });
        fetch('/api/onboarding/launch', { method: 'POST', headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } }).catch(console.error);
        return;
      } catch (err: any) {
        console.error(err);
        setError(err.message || 'Failed to start onboarding');
        setStep(3); syncStateToBackend({ step: 3 });
        setIsLoading(false);
        return;
      }
    }

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
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const startRes = await fetchWithRetry('/api/onboarding/start', {
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
          admin_email: adminEmail,
          admin_name: adminName || businessName + ' Admin',
          admin_password: adminPassword,
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: domainChoice || 'subdomain',
          price_type: 'fixed',
          location: location || '',
          target_audience: targetAudience || '',
          ai_agents: aiAgents,
          ai_auto_respond: aiAutoRespond,
          initial_products: JSON.parse(localStorage.getItem('onboarding_initial_products') || '[]')
        })
      });

      const result = await startRes.json().catch(() => ({}));
      if (!startRes.ok) {
        throw new Error(result.error || result.message || 'Failed to start onboarding');
      }

      // UX: enforce a minimum loading screen display of 500ms so the user sees progress
      await new Promise(resolve => setTimeout(resolve, 500));

      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      if (result.organization_id) {
        localStorage.setItem('tenant_id', result.organization_id);
        localStorage.setItem('tenant', result.organization_id);
      }
      setStep(5); syncStateToBackend({ step: 5 }); // Go to "You're Live" screen
      fetch('/api/onboarding/launch', { method: 'POST', headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } }).catch(console.error);

    } catch (err: any) {
      console.error(err);
      setError(err.message || 'Failed to start onboarding');
      setStep(3); syncStateToBackend({ step: 3 });
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  const showIntroBack = false;

  // Progress percentage calculation
  const getProgress = () => 0;

  return (
    <div className="setup-page min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center p-4">
      <div id="setup-screen" className="w-full sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[640px] sm:min-h-[812px] relative rounded-[16px] glassmorphism border border-white/20 shadow-2xl">
        <div className="px-6 pt-5 text-center">
          <div className="setup-header-main">
            {showIntroBack ? (
              <button type="button" onClick={handleBackToIntro} className="setup-nav-button">
                Back
              </button>
            ) : (
              <span className="setup-nav-spacer" aria-hidden="true"></span>
            )}
            <div>
              <h1 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Setup</h1>
              <p className="text-sm text-gray-500 dark:text-[#A1A1A6]">Your business, live in minutes.</p>
            </div>
            <button type="button" onClick={handleSkipSetup} className="setup-nav-button">
              Skip setup
            </button>
          </div>
        </div>
        {/* Progress Bar */}
        <div className="h-1.5 w-full bg-gray-200 overflow-hidden">
          <div
            className="h-full bg-[#0066FF] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] shadow-[0_0_10px_rgba(0,102,255,0.5)]"
            style={{ width: `${getProgress()}%` }}
          ></div>
        </div>

        <div className="p-6 flex-1 flex flex-col overflow-y-auto custom-scrollbar">
          {error && (
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-4 rounded-[16px] text-sm animate-shake">
              {error}
            </div>
          )}

          {step === 0 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">

              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8 leading-relaxed max-w-sm">
                Our AI will handle the rest in 30 seconds.
              </p>

              <div className="flex flex-col gap-4 w-full">
                <textarea
                  id="instant-bio"
                  data-testid="instant-bio"
                  className={`glassmorphism w-full p-4 rounded-[16px] text-[#1D1D1F] dark:text-[#F5F5F7] outline-none min-h-[44px] transition-all duration-[250ms] ${error === "Please tell us about your business." || error ? "border border-[#FF3B30]" : "border border-white/20 focus:border-[#0066FF]"}`}
                  placeholder="e.g. I run a local bakery that sells custom vegan cakes..."
                  rows={6}
                  style={{ resize: 'none' }}
                  value={bio}
                  onChange={(e) => {
                    setBio(e.target.value);
                    if (error) setError('');
                  }}
                />

                <input
                  id="instant-image-url"
                  data-testid="instant-image-url"
                  type="url"
                  className="glassmorphism w-full p-4 rounded-[16px] text-[#1D1D1F] dark:text-[#F5F5F7] outline-none min-h-[44px] border border-white/20 focus:border-[#0066FF] transition-all duration-[250ms]"
                  placeholder="Image URL (Optional)"
                  value={instantImageUrl}
                  onChange={(e) => setInstantImageUrl(e.target.value)}
                />


                <input
                  id="admin-email"
                  data-testid="admin-email"
                  type="email"
                  className="glassmorphism w-full p-4 rounded-[16px] text-[#1D1D1F] dark:text-[#F5F5F7] outline-none min-h-[44px] border border-white/20 focus:border-[#0066FF] transition-all duration-[250ms]"
                  placeholder="Admin Email"
                  value={adminEmail}
                  onChange={(e) => setAdminEmail(e.target.value)}
                />

                <input
                  id="admin-password"
                  data-testid="admin-password"
                  type="password"
                  className="glassmorphism w-full p-4 rounded-[16px] text-[#1D1D1F] dark:text-[#F5F5F7] outline-none min-h-[44px] border border-white/20 focus:border-[#0066FF] transition-all duration-[250ms]"
                  placeholder="Admin Password"
                  value={adminPassword}
                  onChange={(e) => setAdminPassword(e.target.value)}
                />

                <div className="mt-4">

                  <button
                    onClick={handleInstantBuild}
                    disabled={!bio.trim() || !adminEmail.trim() || !adminPassword.trim() || isLoading}
                    className="w-full bg-[#0066FF] text-white p-4 font-bold rounded-[16px] shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed min-h-[44px]"
                  >
                    Next
                  </button>
                </div>
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
                <div className="p-3 glassmorphism rounded-[16px] flex flex-col items-center mb-6">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-semibold">{generateSubdomain(businessName)}</span>
                   </div>
                </div>

                <a
                  href="/assistant"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[16px] font-bold shadow-md hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="sparkles">Open Assistant</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center glassmorphism text-[#1D1D1F] dark:text-[#F5F5F7] p-4 rounded-[16px] font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
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
