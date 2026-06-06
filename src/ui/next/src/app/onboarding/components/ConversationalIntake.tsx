import React, { useRef, useEffect, useState } from 'react';
import { useOnboardingStore } from '../store';
import { ChatMessage, Message } from './ChatMessage';
import { IconLabel } from './Icons';

export function ConversationalIntake({ onSaveDraft }: { onSaveDraft: () => Promise<void> }) {
  const {
    setStep,
    chatStep, setChatStep,
    businessName, setBusinessName,
    whatYouSell, setWhatYouSell,
    location, setLocation,
    setBusinessType,
    setCategories,
    setFirstProductName,
    setFirstProductPrice,
    isLoading, setIsLoading,
    setError,
    saveMessage
  } = useOnboardingStore();

  const [messages, setMessages] = useState<Message[]>([]);
  const [validationError, setValidationError] = useState('');
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const msgs: Message[] = [];
    if (chatStep >= 1) {
      msgs.push({ id: '0', role: 'user', text: "Let's go!" });
      msgs.push({ id: '1', role: 'agent', text: "Welcome! Let's get your business online in under 10 minutes. What's the name of your business?" });
    }
    if (chatStep >= 2) {
      msgs.push({ id: '2', role: 'user', text: businessName });
      msgs.push({ id: '3', role: 'agent', text: `Great name! Now, what do you sell at ${businessName}?` });
    }
    if (chatStep >= 3) {
      msgs.push({ id: '4', role: 'user', text: whatYouSell });
      msgs.push({ id: '5', role: 'agent', text: "Perfect. And where are you located? (This helps with shipping and taxes)." });
    }
    setMessages(msgs);
  }, [chatStep, businessName, whatYouSell]);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  const addUserMessage = (text: string) => {
    setMessages(prev => [...prev, { id: Date.now().toString() + Math.random(), role: 'user', text }]);
  };

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

      setStep(2);
    } catch (err: any) {
      console.error(err);
      setError(err.message || 'An error occurred processing details');
      setChatStep(3);
    } finally {
      setIsLoading(false);
    }
  };

  if (chatStep === 0) {
    return (
      <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in text-center">
        <div className="w-20 h-20 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6 shadow-inner animate-pulse">
          <svg className="w-10 h-10 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h2 className="text-4xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 tracking-tight">One Human Corp</h2>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-lg mb-8 max-w-xs mx-auto">
          Let's get your business online in under 10 minutes.
        </p>
        <button
          role="link"
          onClick={() => {
              addUserMessage("Let's go!");
              setTimeout(() => setChatStep(1), 500);
          }}
          className="w-full sm:max-w-xs bg-[#0066FF] text-white h-[58px] rounded-[16px] font-bold text-lg shadow-[0_4px_14px_0_rgba(0,102,255,0.4)] hover:bg-[#0052cc] hover:shadow-[0_6px_20px_rgba(0,102,255,0.3)] active:scale-[0.97] transition-all duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]"
        >
          Start Onboarding
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col flex-1 overflow-hidden">
      <div className="flex items-center justify-between mb-4 pb-4 border-b border-white/10 shrink-0">
        <div className="flex items-center gap-2">
           <div className="w-3 h-3 rounded-full bg-[#34C759] shadow-[0_0_8px_#34C759]"></div>
           <span className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Onboarding Expert</span>
        </div>
        <button
          onClick={onSaveDraft}
          className="text-xs font-bold text-[#0066FF] bg-[#0066FF]/10 px-3 py-1.5 rounded-full hover:bg-[#0066FF]/20 transition-all"
        >
          <IconLabel icon="save">Save Draft</IconLabel>
        </button>
      </div>

      {saveMessage && <p className="text-[#34C759] text-xs font-bold mb-4 animate-fade-in">{saveMessage}</p>}

      <div ref={scrollRef} className="flex-1 overflow-y-auto custom-scrollbar pr-2 pb-4 space-y-2">
        {messages.map((m) => (
          <ChatMessage key={m.id} message={m} />
        ))}
      </div>

      <div className="mt-auto pt-4 shrink-0">
        {chatStep === 1 && (
          <div className="animate-fade-in space-y-4">
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
                  addUserMessage(businessName);
                  setTimeout(() => setChatStep(2), 600);
                }
              }}
              placeholder="Maya's Custom Cakes"
              className="w-full p-4 rounded-[16px] focus:ring-2 focus:ring-[#0066FF]/30 outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner border border-white/20"
            />
            {validationError && <p className="text-red-500 text-xs font-bold ml-1">{validationError}</p>}
            <button
              onClick={() => {
                if (businessName.trim().length < 3) {
                  setValidationError('Business Name must be at least 3 characters.');
                  return;
                }
                setValidationError('');
                addUserMessage(businessName);
                setTimeout(() => setChatStep(2), 600);
              }}
              disabled={!businessName.trim()}
              className="w-full bg-[#0066FF] text-white h-[54px] rounded-[16px] font-bold shadow-lg hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
            >
              <IconLabel icon="next">Next</IconLabel>
            </button>
          </div>
        )}

        {chatStep === 2 && (
          <div className="animate-fade-in space-y-4">
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
                  addUserMessage(whatYouSell);
                  setTimeout(() => setChatStep(3), 600);
                }
              }}
              placeholder="e.g. I bake custom vegan cakes for weddings..."
              className="w-full p-4 rounded-[16px] focus:ring-2 focus:ring-[#0066FF]/30 outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all shadow-inner border border-white/20"
            />
            {validationError && <p className="text-red-500 text-xs font-bold ml-1">{validationError}</p>}
            <button
              onClick={() => {
                if (!whatYouSell.trim()) {
                  setValidationError('Please tell us what you sell.');
                  return;
                }
                setValidationError('');
                addUserMessage(whatYouSell);
                setTimeout(() => setChatStep(3), 600);
              }}
              disabled={!whatYouSell.trim()}
              className="w-full bg-[#0066FF] text-white h-[54px] rounded-[16px] font-bold shadow-lg hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
            >
              <IconLabel icon="next">Next</IconLabel>
            </button>
          </div>
        )}

        {chatStep === 3 && (
          <div className="animate-fade-in space-y-4">
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
                    addUserMessage(location);
                    setTimeout(() => handleIntake(), 800);
                  }
                }
              }}
              placeholder="Portland, OR"
              className="w-full p-4 rounded-[16px] focus:ring-2 focus:ring-[#0066FF]/30 outline-none mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all shadow-inner border border-white/20"
            />
            {validationError && <p className="text-red-500 text-xs font-bold ml-1">{validationError}</p>}
            <button
              onClick={() => {
                if (!location.trim()) {
                  setValidationError('Please tell us your location.');
                  return;
                }
                setValidationError('');
                addUserMessage(location);
                setTimeout(() => handleIntake(), 800);
              }}
              disabled={!location.trim() || isLoading}
              className="w-full bg-[#0066FF] text-white h-[54px] rounded-[16px] font-bold shadow-lg hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
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
        )}
      </div>
    </div>
  );
}
