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

  const handleSendChatMessage = async () => {
    if (!chatInput.trim() && !chatImageUrl.trim()) return;

    const newMessage = {
      role: 'user',
      content: chatInput,
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

        let email = adminEmail;
        let password = adminPassword;
        let name = adminName;
        if (!adminEmail.trim() || !adminPassword.trim()) {
           updateState({ adminEmail: "owner@example.com", adminPassword: "password", adminName: "Owner" });
           email = "owner@example.com";
           password = "password";
           name = "Owner";
        }

        try {
          const startRes = await fetch('/api/onboarding/start', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
            body: JSON.stringify({
              org_name: intakeData.business_name || "My Business",
              domain: domainChoice === 'custom' ? websiteTemplate : `${(intakeData.business_name || "my-business").toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '')}.ohc.app`,
              business_type: intakeData.business_type || "Online Store",
              first_product: intakeData.initial_products?.[0]?.name || "First Product",
              admin_email: email,
              admin_password: password
            })
          });
          const startData = await startRes.json();
          updateState({ startResult: startData });
        } catch (err) {
          console.error("Failed to start:", err);
        }

        try {
          await fetch('/api/onboarding/launch', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
          });
        } catch (err) {
          console.error("Failed to launch:", err);
        }

        router.push('/feed');
        return;
      } else {
                              setValidationErrors(prev => { const { firstProductPrice, ...rest } = prev; return rest; });
                           }
                        }}
                        className={`w-full p-3 sm:p-4 border ${validationErrors.firstProductPrice ? 'border-[#FF3B30]' : 'border-white/40 dark:border-white/10 focus:border-[#0066FF]'} outline-none bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                      />
                      {validationErrors.firstProductPrice && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.firstProductPrice}</p>}
                   </div>
                </div>
              </div>

              {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
              <div className="mt-auto pt-6">
                <button
                  onClick={() => {
                    let hasError = false;
                    const newErrors: Record<string, string> = { ...validationErrors };
                    if (businessName.trim().length < 3) {
                      newErrors.businessName = 'Must be at least 3 characters.';
                      hasError = true;
                    }
                    if (businessType.trim().length === 0) {
                      newErrors.businessType = 'Business Type is required to configure your agents.';
                      hasError = true;
                    }
                    if (firstProductPrice.trim().length === 0) {
                      newErrors.firstProductPrice = 'A price is needed to set up your Stripe catalog.';
                      hasError = true;
                    }

                    if (hasError || Object.keys(newErrors).length > 0) {
                      setValidationErrors(newErrors);
                      setValidationError('Please fix the errors before continuing.');
                      return;
                    }

                    setValidationError('');
                    updateState({ step: 3 }); syncStateToBackend({ step: 3 });
                  }}
                  className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <IconLabel icon="next">Continue</IconLabel>
                </button>
              </div>
            </div>
          )}

          {step === 3 && (
            <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
              <button onClick={() => { updateState({ step: 2 }); syncStateToBackend({ step: 2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
              </button>
              <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Style & Team</h2>
              <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
                <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
                  Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
                </p>
                <button
                  onClick={() => handleSaveDraft()}
                  className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
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
                        onClick={() => updateState({ websiteTemplate: template })}
                        className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
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
                      onClick={() => updateState({ domainChoice: 'subdomain' })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] flex flex-col items-center justify-center text-center ${domainChoice === 'subdomain' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
                    >
                      <span className="font-semibold text-sm mb-1">Free Subdomain</span>
                      <span className="text-[10px] opacity-70">your-name.ohc.app</span>
                    </div>
                    <div
                      onClick={() => updateState({ domainChoice: 'custom' })}
                      className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] flex flex-col items-center justify-center text-center ${domainChoice === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-white/10 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
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

                        autoCapitalize="words"
                        autoComplete="name"
                        value={adminName}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ adminName: val });
                          if (!val.trim()) {
                            setValidationErrors(prev => ({ ...prev, adminName: 'Admin Name is required' }));
                          } else {
                            setValidationErrors(prev => { const { adminName, ...rest } = prev; return rest; });
                          }
                        }}
                        placeholder="e.g. Maya Smith"
                        className={`w-full p-3 sm:p-4 border ${validationErrors.adminName ? "border-[#FF3B30]" : "border-white/40 dark:border-white/10 focus:border-[#0066FF]"} outline-none bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                        inputMode="text"
                        enterKeyHint="next"
                      />
                      {validationErrors.adminName && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminName}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Email</label>
                      <input
                        type="email"
                    autoCapitalize="none"
                    autoComplete="email"
                    inputMode="email"
                    enterKeyHint="next"
                    value={adminEmail}
                    onChange={(e) => {
                      const val = e.target.value;
                      updateState({ adminEmail: val });
                      if (!val.trim()) {
                        setValidationErrors(prev => ({ ...prev, adminEmail: 'Admin Email is required' }));
                      } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val)) {
                        setValidationErrors(prev => ({ ...prev, adminEmail: 'Please enter a valid email address' }));
                      } else {
                        setValidationErrors(prev => { const { adminEmail, ...rest } = prev; return rest; });
                      }
                    }}
                    placeholder="you@example.com"
                    className={`w-full p-3 sm:p-4 border ${validationErrors.adminEmail ? "border-[#FF3B30]" : "border-white/40 dark:border-white/10 focus:border-[#0066FF]"} outline-none bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                  />
                      {validationErrors.adminEmail && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminEmail}</p>}
                    </div>
                    <div>
                      <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Password</label>
                      <input
                        type="password"
                        enterKeyHint="done"
                        autoComplete="new-password"
                        value={adminPassword}
                        onChange={(e) => {
                          const val = e.target.value;
                          updateState({ adminPassword: val });
                          if (!val.trim()) {
                            setValidationErrors(prev => ({ ...prev, adminPassword: 'Password is required' }));
                          } else if (val.length < 8 || !/\d/.test(val)) {
                            setValidationErrors(prev => ({ ...prev, adminPassword: 'Password must be at least 8 characters and contain a number' }));
                          } else {
                            setValidationErrors(prev => { const { adminPassword, ...rest } = prev; return rest; });
                          }
                        }}
                        placeholder="••••••••"
                        className={`w-full p-3 sm:p-4 border ${validationErrors.adminPassword ? "border-[#FF3B30]" : "border-white/40 dark:border-white/10 focus:border-[#0066FF]"} outline-none bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px]`}
                      />
                      {validationErrors.adminPassword && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminPassword}</p>}
                    </div>
                  </div>
                </div>

                <div className="pt-2 border-t border-white/50 dark:border-white/10">
                  <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Auto-Configured AI Departments</label>
                  <p className="text-gray-500 dark:text-[#A1A1A6] text-xs mb-2">
                    Here are the AI departments we've configured for you.
                  </p>
                  <div className="flex flex-col sm:flex-row flex-wrap gap-2 mt-2">
                    {aiAgents.map(agent => (
                      <div
                        key={agent}
                        className="px-3 py-1.5 rounded-full border border-[#34C759] bg-[#34C759]/10 text-[#34C759] flex items-center gap-1.5 text-sm font-semibold transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
                      >
                        <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                        {agent}
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-2">
                  <label className="flex items-center justify-between cursor-pointer p-3 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 rounded-[8px] text-[#1D1D1F] dark:text-white">
                    <span className="font-semibold text-sm">Allow AI to Auto-Respond</span>
                    <input
                      type="checkbox"
                      className="sr-only"
                      checked={aiAutoRespond}
                      onChange={(e) => updateState({ aiAutoRespond: e.target.checked })}
                    />
                    <div className={`w-10 h-6 rounded-full transition-colors ${aiAutoRespond ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-600'} relative`}>
                       <div className={`w-4 h-4 rounded-full bg-white absolute top-1 transition-transform ${aiAutoRespond ? 'translate-x-5' : 'translate-x-1'}`}></div>
                    </div>
                  </label>
                </div>
              </div>

              <div className="mt-auto pt-6">
                <button
                  onClick={() => handleStartOnboarding()}
                  disabled={isLoading}
                  className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isLoading ? (
                    <span className="flex items-center justify-center gap-2">
                      <svg className="animate-spin h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                        <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                        <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                      </svg>
                      Launching...
                    </span>
                  ) : <IconLabel icon="launch">Approve & Publish</IconLabel>}
                </button>
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
