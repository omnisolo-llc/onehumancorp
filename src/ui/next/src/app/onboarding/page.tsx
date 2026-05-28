"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

export default function OnboardingWizard() {
  const {
    step, setStep,
    businessName, setBusinessName,
    whatYouSell, setWhatYouSell,
    location, setLocation,
    businessType, setBusinessType,
    categories, setCategories,
    websiteTemplate, setWebsiteTemplate,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    isLoading, setIsLoading,
    error, setError,
    startResult, setStartResult
  } = useOnboardingStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const [messages, setMessages] = useState<Array<{ role: 'ai' | 'user', content: string }>>([]);
  const [inputValue, setInputValue] = useState('');
  const chatEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setIsLoaded(true);
    setMessages([
      { role: 'ai', content: "Hi! I'm your OHC setup assistant. Let's get your business live in under 10 minutes." },
      { role: 'ai', content: "What's the name of your business?" }
    ]);
  }, []);

  useEffect(() => {
    chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!inputValue.trim()) return;

    const currentMessages = [...messages, { role: 'user', content: inputValue } as const];
    setMessages(currentMessages);
    const userMsg = inputValue;
    setInputValue('');

    if (businessName === '') {
      setBusinessName(userMsg);
      setMessages([...currentMessages, { role: 'ai', content: `Nice to meet you, ${userMsg}! What do you sell? Describe your products or services.` }]);
    } else if (whatYouSell === '') {
      setWhatYouSell(userMsg);
      setMessages([...currentMessages, { role: 'ai', content: "Got it. And where are you located? (City, State/Country)" }]);
    } else if (location === '') {
      setLocation(userMsg);
      setMessages([...currentMessages, { role: 'ai', content: "Perfect! I'm analyzing your business details to generate your storefront..." }]);
      handleIntake(userMsg);
    }
  };

  const handleIntake = async (lastInput: string) => {
    setIsLoading(true);
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const combinedDescription = `Business Name: ${businessName}\nWhat we sell: ${whatYouSell}\nLocation: ${lastInput}`;

      const intakeRes = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ description: combinedDescription })
      });

      if (!intakeRes.ok) throw new Error('Failed to process business details');

      const intakeData = await intakeRes.json();

      setBusinessType(intakeData.business_type || 'Online Store');
      setFirstProductName(intakeData.initial_items?.[0]?.name || 'First Product');
      setFirstProductPrice(intakeData.initial_items?.[0]?.price || '10.00');
      setCategories(intakeData.categories || ['physical']);

      setMessages(prev => [...prev,
        { role: 'ai', content: `I've prepared a plan for ${intakeData.business_name}. It looks like a ${intakeData.business_type}.` },
        { role: 'ai', content: `I'll set up your first item: ${intakeData.initial_items?.[0]?.name} for $${intakeData.initial_items?.[0]?.price}.` },
        { role: 'ai', content: "Ready to launch? Or do you want to review the details?" }
      ]);
      setStep(2); // Show review/launch step
    } catch (err: any) {
      setError(err.message || 'An error occurred');
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setStep(4); // Loading screen

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
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
          selling_categories: categories,
          payment_pref: 'online',
          admin_email: 'admin@ohc.app',
          admin_name: 'Admin',
          admin_password: 'password123',
          website_template: websiteTemplate,
          first_product_name: firstProductName,
          first_product_price: firstProductPrice,
          domain_choice: 'subdomain',
          price_type: 'fixed'
        })
      });

      if (!startRes.ok) throw new Error('Failed to start onboarding');

      const result = await startRes.json();
      setStartResult(result);
      localStorage.setItem('has_onboarded', 'true');
      setStep(5); // Live screen
    } catch (err: any) {
      setError(err.message || 'An error occurred');
      setStep(2);
    } finally {
      setIsLoading(false);
    }
  };

  if (!isLoaded) return null;

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] font-inter flex flex-col justify-center px-4 py-8">
      <div className="w-full max-w-[375px] mx-auto mac-glass-container rounded-[24px] shadow-2xl overflow-hidden flex flex-col h-[667px] relative border border-white/20">

        {/* Header */}
        <div className="p-4 border-b border-white/10 flex items-center gap-3 bg-white/50 dark:bg-black/20 backdrop-blur-md">
          <div className="w-8 h-8 bg-[#0066FF] rounded-full flex items-center justify-center">
             <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </div>
          <div>
            <h1 className="text-sm font-bold text-[#1D1D1F] dark:text-white">OHC Assistant</h1>
            <p className="text-[10px] text-gray-500 uppercase tracking-widest font-semibold">Born Live Onboarding</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50/50 dark:bg-black/10">
          {messages.map((m, i) => (
            <div key={i} className={`flex ${m.role === 'ai' ? 'justify-start' : 'justify-end'} animate-fade-in`}>
              <div className={`max-w-[80%] p-3 rounded-[18px] text-sm shadow-sm ${m.role === 'ai' ? 'bg-white dark:bg-[#2c2c2e] text-[#1D1D1F] dark:text-[#F5F5F7] rounded-tl-none' : 'bg-[#0066FF] text-white rounded-tr-none'}`}>
                {m.content}
              </div>
            </div>
          ))}

          {step === 2 && (
             <div className="bg-white/80 dark:bg-[#1c1c1e]/80 backdrop-blur-md p-4 rounded-[18px] border border-[#0066FF]/20 space-y-3 animate-fade-in shadow-lg">
                <h3 className="text-xs font-bold text-[#0066FF] uppercase">Review Your Store</h3>
                <div className="space-y-2">
                  <div className="flex justify-between text-xs"><span className="text-gray-500">Name:</span> <span className="font-semibold">{businessName}</span></div>
                  <div className="flex justify-between text-xs"><span className="text-gray-500">Type:</span> <span className="font-semibold">{businessType}</span></div>
                  <div className="flex justify-between text-xs"><span className="text-gray-500">Item:</span> <span className="font-semibold">{firstProductName}</span></div>
                  <div className="flex justify-between text-xs"><span className="text-gray-500">Price:</span> <span className="font-semibold">${firstProductPrice}</span></div>
                </div>
                <button
                  onClick={handleStartOnboarding}
                  className="w-full bg-[#0066FF] text-white py-3 rounded-[12px] font-bold text-sm shadow-md active:scale-95 transition-all"
                >
                  Launch My Storefront
                </button>
             </div>
          )}

          <div ref={chatEndRef} />
        </div>

        {/* Input Area */}
        {step === 1 && (
          <div className="p-4 bg-white/50 dark:bg-black/20 backdrop-blur-md border-t border-white/10">
            <div className="flex gap-2">
              <input
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
                placeholder="Type your message..."
                className="flex-1 bg-white dark:bg-[#2c2c2e] border border-gray-200 dark:border-white/10 rounded-full px-4 py-2 text-sm outline-none focus:border-[#0066FF]"
              />
              <button
                onClick={handleSendMessage}
                className="bg-[#0066FF] text-white w-10 h-10 rounded-full flex items-center justify-center shadow-lg active:scale-90 transition-all"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
              </button>
            </div>
          </div>
        )}

        {/* Loading Overlay */}
        {step === 4 && (
          <div className="absolute inset-0 bg-white/90 dark:bg-black/90 backdrop-blur-xl flex flex-col items-center justify-center z-50 p-8 text-center">
             <div className="w-16 h-16 border-4 border-[#0066FF] border-t-transparent rounded-full animate-spin mb-6"></div>
             <h2 className="text-xl font-bold mb-2">Generating Storefront</h2>
             <p className="text-sm text-gray-500">Provisioning agents, configuring products, and booking systems...</p>
          </div>
        )}

        {/* Live Screen */}
        {step === 5 && (
          <div className="absolute inset-0 bg-[#0066FF] flex flex-col items-center justify-center z-50 p-8 text-center text-white">
             <div className="w-20 h-20 bg-white/20 rounded-full flex items-center justify-center mb-6">
                <svg className="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
             </div>
             <h2 className="text-3xl font-black mb-4">YOU'RE LIVE!</h2>
             <p className="text-white/80 mb-8">{startResult?.message}</p>
             <div className="w-full space-y-3">
                <a href="/dashboard" className="block w-full bg-white text-[#0066FF] py-4 rounded-[16px] font-black shadow-xl">GO TO DASHBOARD</a>
                <a href="/store" className="block w-full bg-white/10 border border-white/30 py-4 rounded-[16px] font-bold">VIEW STOREFRONT</a>
             </div>
          </div>
        )}
      </div>
    </div>
  );
}
