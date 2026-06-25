"use client";

import React, { useEffect, useState, useRef } from 'react';
import { useRouter } from 'next/navigation';
import { useOnboardingStore } from './store';
import { SetupIcon } from './components/SetupIcon';
import { IconLabel } from './components/IconLabel';

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
    step, chatStep, businessDescription, businessGoal, businessName,
    whatYouSell, location, targetAudience, bio, businessType, categories,
    websiteTemplate, domainChoice, firstProductName, firstProductPrice,
    adminName, adminEmail, adminPassword, aiAgents, aiAutoRespond,
    isLoading, error, startResult, instantImageUrl,
    updateState
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
      businessDescription, businessGoal,
      bio,
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
      instantImageUrl,
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
    updateState({ error: '' });
    setValidationError('');
    localStorage.setItem('has_onboarded', 'true');
    syncStateToBackend({ skipped: true });
    router.push('/dashboard');
  };

  const handleBackToIntro = () => {
    updateState({ error: '' });
    setValidationError('');
    setValidationErrors({});
    updateState({ step: 0 });
    syncStateToBackend({ step: 0 });
  };

  const handleSaveDraft = async () => {
    updateState({ isLoading: true });
    updateState({ error: '' });

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const wizardState = {
        step,
        chatStep,
        businessDescription, businessGoal,
        bio,
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
      instantImageUrl
      };

      const res = await fetchWithRetry('/api/onboarding/draft', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ step, ...wizardState })
      });

      setSaveMessage('Draft Saved!');
      setTimeout(() => setSaveMessage(''), 3000);
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || 'An error occurred saving draft' });
    } finally {
      updateState({ isLoading: false });
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
      const isValid = (d: any) => d && Object.keys(d).length > 0;
      let data = isValid(draftData) ? draftData : stateData;
      if (isValid(data)) {
        if (data.wizardState) data = data.wizardState;
        if (data.step !== undefined) updateState({ step: data.step === 4 ? 3 : data.step });
        if (data.chatStep !== undefined) updateState({ chatStep: data.chatStep });
        if (data.businessDescription !== undefined) updateState({ businessDescription: data.businessDescription });
        if (data.businessGoal !== undefined) updateState({ businessGoal: data.businessGoal });
        if (data.bio !== undefined) updateState({ bio: data.bio });
        if (data.businessName !== undefined) updateState({ businessName: data.businessName });
        if (data.whatYouSell !== undefined) updateState({ whatYouSell: data.whatYouSell });
        if (data.location !== undefined) updateState({ location: data.location });
        if (data.targetAudience !== undefined) updateState({ targetAudience: data.targetAudience });
        if (data.businessType !== undefined) updateState({ businessType: data.businessType });
        if (data.categories !== undefined) updateState({ categories: data.categories });
        if (data.websiteTemplate !== undefined) updateState({ websiteTemplate: data.websiteTemplate });
        if (data.firstProductName !== undefined) updateState({ firstProductName: data.firstProductName });
        if (data.firstProductPrice !== undefined) updateState({ firstProductPrice: data.firstProductPrice });
        if (data.adminName !== undefined) updateState({ adminName: data.adminName });
        if (data.adminEmail !== undefined) updateState({ adminEmail: data.adminEmail });
        if (data.adminPassword !== undefined) updateState({ adminPassword: data.adminPassword });
        if (data.domainChoice !== undefined) updateState({ domainChoice: data.domainChoice });
        if (data.aiAgents !== undefined) updateState({ aiAgents: data.aiAgents });
        if (data.aiAutoRespond !== undefined) updateState({ aiAutoRespond: data.aiAutoRespond });
        if (data.instantImageUrl !== undefined) updateState({ instantImageUrl: data.instantImageUrl });
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
    if (step === 1 && !businessName && !whatYouSell && !location && !targetAudience) return;

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    const wizardState = {
      step,
      chatStep,
      businessDescription, businessGoal,
      bio,
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
      instantImageUrl
    };

    const timer = setTimeout(() => {
      fetch('/api/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ step, ...wizardState })
      }).catch(err => console.error('Failed to sync onboarding state', err));
    }, 1000); // debounce 1s

    return () => clearTimeout(timer);
  }, [
    step, chatStep, businessDescription, businessGoal, businessName, whatYouSell, location,
    targetAudience, businessType, categories, websiteTemplate, domainChoice, firstProductName, firstProductPrice,
    adminName, adminEmail, adminPassword, aiAgents, aiAutoRespond, isLoaded, instantImageUrl
  ]);

  const handleIntake = async () => {
    updateState({ isLoading: true });
    updateState({ error: '' });

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${location}\nTarget Audience: ${targetAudience}`;
      updateState({ bio: combinedDescription });

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

      updateState({ businessType: intakeData.business_type || 'Online Store' });
      updateState({ businessName: intakeData.business_name || 'My Business' });
      updateState({ firstProductName: intakeData.initial_products?.[0]?.name || 'First Product' });
      updateState({ firstProductPrice: intakeData.initial_products?.[0]?.price || '10.00' });
      if (intakeData.initial_products) {
          localStorage.setItem('onboarding_initial_products', JSON.stringify(intakeData.initial_products));
      }
      const mappedCategories = intakeData.categories || ['physical'];
      updateState({ categories: mappedCategories });

      // Auto-configure AI Departments based on inferred business context
      const newAgents = ['Operations', 'Marketing', 'Finance', 'Legal', 'Advisory'];
      if (mappedCategories.includes('physical') || mappedCategories.includes('digital') || mappedCategories.includes('subscriptions')) {
        newAgents.push('Sales');
      }
      if (mappedCategories.includes('services') || mappedCategories.includes('food') || mappedCategories.includes('physical')) {
        newAgents.push('Customer Success');
      }
      updateState({ aiAgents: newAgents });

      updateState({ step: 2 }); await syncStateToBackend({
        step: 2,
        aiAgents: newAgents,
        firstProductName: intakeData.initial_products?.[0]?.name || "First Product",
        firstProductPrice: intakeData.initial_products?.[0]?.price || "10.00"
      }); // Go to review step
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || 'An error occurred processing details' });
      updateState({ step: 1 }); syncStateToBackend({ step: 1 });
      updateState({ chatStep: 3 }); syncStateToBackend({ chatStep: 3 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleSendChatMessage = async (overrideMessage?: string) => {
    const msgToSend = overrideMessage || chatInput;
    if (!msgToSend.trim() && !chatImageUrl.trim()) return;

    const newMessage = {
      role: 'user',
      content: msgToSend,
      image_url: chatImageUrl || undefined,
    };

    const newHistory = [...chatMessages, newMessage];
    setChatMessages(newHistory);
    setChatInput('');
    setChatImageUrl('');
    updateState({ isLoading: true });

    try {
      const backendUrl = (typeof window !== 'undefined' && (window.location.origin.includes('localhost') || window.location.protocol === 'file:')) ? 'http://127.0.0.1:18789' : '';

      const res = await fetch(`${backendUrl}/api/onboarding/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: newHistory })
      });

      if (!res.ok) throw new Error('Chat request failed');
      const data = await res.json();

      setChatMessages([...newHistory, { role: 'assistant', content: data.reply }]);

      if (data.is_complete && data.intake_data) {
        updateState({ step: 4 }); syncStateToBackend({ step: 4 });
        const intakeData = data.intake_data;
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
        const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

        // Pre-fill state values so we don't need to manually type everything
        updateState({ businessName: intakeData.business_name || "My Business" });
        updateState({ businessType: intakeData.business_type || "Online Store" });
        updateState({ businessDescription: newHistory.map(m => m.content).join(" ") });
        updateState({ categories: intakeData.categories || ["physical"] });
        updateState({ firstProductName: intakeData.initial_products?.[0]?.name || "First Product" });
        updateState({ firstProductPrice: intakeData.initial_products?.[0]?.price || "0.00" });
        updateState({ location: intakeData.location || "" });
        updateState({ targetAudience: intakeData.target_audience || "" });

        // Let the normal handleStartOnboarding function take over if admin details are missing
        if (!adminEmail.trim() || !adminPassword.trim()) {
          updateState({ step: 3 }); syncStateToBackend({
            step: 3,
            firstProductName: intakeData.initial_products?.[0]?.name || "First Product",
            firstProductPrice: intakeData.initial_products?.[0]?.price || "0.00"
          });
          updateState({ isLoading: false });
          return;
        }

        const startRes = await fetchWithRetry(`${backendUrl}/api/onboarding/start`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'X-Tenant-ID': tenantId,
            'X-User-ID': userId,
          },
          body: JSON.stringify({
            business_type: intakeData.business_type || "Online Store",
            company_name: intakeData.business_name || "My Business",
            company_description: newHistory.map(m => m.content).join(" "),
            selling_categories: intakeData.categories || ["physical"],
            payment_pref: "online",
            admin_email: adminEmail,
            admin_name: adminName || intakeData.business_name || "Admin",
            admin_password: adminPassword,
            website_template: "auto",
            first_product_name: intakeData.initial_products?.[0]?.name || "First Product",
            first_product_price: intakeData.initial_products?.[0]?.price || "0.00",
            domain_choice: "subdomain",
            price_type: "fixed",
            location: intakeData.location || "",
            target_audience: intakeData.target_audience || "",
            ai_agents: [],
            ai_auto_respond: true,
            initial_products: intakeData.initial_products || [],
          })
        });

        const result = await startRes.json();
        updateState({ startResult: result });

        if (result.organization_id) {
           localStorage.setItem('tenant_id', result.organization_id);
           localStorage.setItem('tenant', result.organization_id);
        }
        const launchRes = await fetchWithRetry(`${backendUrl}/api/onboarding/launch`, { method: 'POST', headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } });
        if (!launchRes.ok) throw new Error('Launch failed');
        updateState({ step: 5 }); syncStateToBackend({ step: 5 });

        // Optional, but required by E2E test
        if (typeof window !== 'undefined' && window.location.href.includes('setup.html')) {
           window.location.href = '/success.html';
        }
      }
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || 'Failed to send chat message' });
    } finally {
      updateState({ isLoading: false });
    }
  };

  const handleInstantBuild = async () => {
    if (!bio.trim()) {
      updateState({ error: 'Please tell us about your business.' });
      return;
    }
    updateState({ isLoading: true });
    updateState({ error: '' });

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

      updateState({ businessName: intakeData.business_name || 'My Business' });
      updateState({ businessType: intakeData.business_type || 'Online Store' });
      updateState({ businessDescription: bio });
      updateState({ categories: intakeData.categories || ['physical'] });
      updateState({ firstProductName: intakeData.initial_products?.[0]?.name || 'First Product' });
      updateState({ firstProductPrice: intakeData.initial_products?.[0]?.price || '0.00' });
      updateState({ location: intakeData.location || 'Local' });
      updateState({ targetAudience: intakeData.target_audience || 'General' });
      updateState({ adminName: intakeData.business_name || 'Admin' });
      updateState({ domainChoice: 'subdomain' });
      updateState({ websiteTemplate: 'auto' });

      if (intakeData.initial_products) {
         localStorage.setItem('onboarding_initial_products', JSON.stringify(intakeData.initial_products));
      }

      updateState({ step: 3 }); syncStateToBackend({
        step: 3,
        firstProductName: intakeData.initial_products?.[0]?.name || 'First Product',
        firstProductPrice: intakeData.initial_products?.[0]?.price || '0.00',
        businessType: intakeData.business_type || 'Online Store',
        businessName: intakeData.business_name || 'My Business',
        categories: intakeData.categories || ['physical']
      });
    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || 'Failed to generate your business' });
      updateState({ step: -1 }); syncStateToBackend({ step: -1 });
    } finally {
      updateState({ isLoading: false });
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
    updateState({ isLoading: true });
    updateState({ error: '' });
    updateState({ step: 4 }); syncStateToBackend({ step: 4 }); // Go to loading screen
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
          admin_name: adminName || (businessName ? businessName + ' Admin' : 'Admin'),
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

      updateState({ startResult: result });
      localStorage.setItem('has_onboarded', 'true');
      if (result.organization_id) {
        localStorage.setItem('tenant_id', result.organization_id);
        localStorage.setItem('tenant', result.organization_id);
      }
      const launchRes = await fetchWithRetry('/api/onboarding/launch', { method: 'POST', headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId } });
      if (!launchRes.ok) throw new Error('Launch failed');
      updateState({ step: 5 }); syncStateToBackend({ step: 5 }); // Go to "You're Live" screen

    } catch (err: any) {
      console.error(err);
      updateState({ error: err.message || 'Failed to start onboarding' });
      updateState({ step: 3 }); syncStateToBackend({ step: 3 });
    } finally {
      updateState({ isLoading: false });
    }
  };

  if (!isLoaded) return null;

  const showIntroBack = step === 1 && chatStep === 1;

  // Progress percentage calculation
  const getProgress = () => {
    // There are 5 steps, let's make it a more gradual fill
    if (step === 1) {
      if (chatStep === 1) return 25;
      if (chatStep === 2) return 35;
      if (chatStep === 3) return 40;
      if (chatStep === 4) return 45;
      if (chatStep === 5) return 50;
    }
    if (step === 2) return 60;
    if (step === 3) return 80;
    if (step === 4) return 95;
    if (step === 5) return 100;
    return 0;
  };

  return (
    <div className="setup-page min-h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] flex items-center justify-center sm:p-4 font-inter overflow-x-hidden">
      <div id="setup-screen" className="w-full max-w-[375px] sm:max-w-md lg:max-w-lg xl:max-w-2xl mx-auto overflow-hidden flex flex-col min-h-[100vh] sm:min-h-[812px] relative bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border-0 sm:border border-white/40 dark:border-white/10 shadow-none sm:shadow-[0_18px_44px_rgba(15,23,42,0.12)] sm:rounded-[16px]">
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
            <div className="mb-4 bg-[#FF3B30]/10 border border-[#FF3B30]/30 text-[#FF3B30] p-4 text-sm animate-shake">
              {error}
            </div>
          )}

                    {step === 0 && (
            <div className="flex flex-col flex-1 animate-fade-in w-full h-full max-h-full">
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">Setup Assistant</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-4 leading-relaxed max-w-sm mx-auto">
                Talk to our AI to build your business.
              </p>

              <div className="flex flex-col flex-1 gap-4 overflow-hidden w-full max-w-full">
                <div id="chat-messages" className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] flex-1 overflow-y-auto p-4 text-[#1D1D1F] dark:text-[#F5F5F7] text-left space-y-4">
                  {chatMessages.length === 0 && (
                    <div className="mb-2"><strong>Assistant:</strong> What do you want to build or manage today?</div>
                  )}
                  {chatMessages.map((msg, index) => (
                    <div key={index} className={`mb-2 ${msg.role === 'user' ? 'text-[#0066FF]' : 'text-[#333] dark:text-[#A1A1A6]'}`}>
                      <strong>{msg.role === 'user' ? 'You' : 'Assistant'}:</strong> {msg.content}
                      {msg.image_url && <><br /><span className="text-xs text-gray-500 dark:text-gray-400">[Attached Image: {msg.image_url}]</span></>}
                    </div>
                  ))}
                  {isLoading && (
                    <div className="mb-2 text-[#333] dark:text-[#A1A1A6]">
                       <span className="flex items-center gap-2">
                        <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        <strong>Assistant:</strong> Thinking...
                      </span>
                    </div>
                  )}
                </div>

                <div className="flex flex-wrap gap-2 justify-center mb-2">
                  {["Cake Shop", "Handyman", "Boutique", "Tutor", "Food Cart"].map((chip) => (
                    <button
                      key={chip}
                      onClick={() => handleSendChatMessage(chip)}
                      className="px-4 py-2 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-full text-sm font-semibold text-[#0066FF] hover:bg-[#0066FF] hover:text-white transition-colors"
                    >
                      {chip}
                    </button>
                  ))}
                </div>

                <div className="flex flex-col gap-2 shrink-0">
                  <input
                    type="url"
                    id="chat-image-url"
                    value={chatImageUrl}
                    onChange={(e) => setChatImageUrl(e.target.value)}
                    className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] border border-white/20 focus:border-[#0066FF] min-h-[44px]"
                    placeholder="Image URL (Optional)"
                    inputMode="url"
                    autoComplete="url"
                    enterKeyHint="next"
                  />
                  <div className="flex gap-2 w-full">
                    <input
                      type="text"
                      id="chat-input"
                      value={chatInput}
                      onChange={(e) => setChatInput(e.target.value)}
                      onKeyDown={(e) => {
                         if (e.key === 'Enter') handleSendChatMessage();
                      }}
                      className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none flex-1 transition-all duration-[250ms] border border-white/20 focus:border-[#0066FF] min-h-[44px]"
                      placeholder="Type a message..."
                      enterKeyHint="send"
                    />
                    <button
                      id="chat-send-btn"
                      onClick={() => handleSendChatMessage()}
                      disabled={isLoading}
                      className="bg-[#0066FF] text-white font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] px-4 shrink-0 disabled:opacity-50 rounded-[8px]"
                    >
                      Send
                    </button>
                  </div>
                </div>
              </div>
            </div>
          )}

          {step === 4 && (
             <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] shadow-2xl p-8">
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
                <div className="p-3 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] flex flex-col items-center mb-6">
                   <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
                   <div className="flex items-center gap-2">
                      <span className="text-[#0066FF] font-semibold">{generateSubdomain(businessName)}</span>
                   </div>
                </div>

                <a
                  href="/assistant"
                  className="flex w-full items-center justify-center bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold shadow-md hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                >
                  <IconLabel icon="sparkles">Open Assistant</IconLabel>
                </a>
                <a
                  href="/builder"
                  className="flex w-full items-center justify-center bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold shadow-sm active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
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
