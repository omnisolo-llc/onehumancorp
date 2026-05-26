"use client";

import React, { useEffect, useRef, useState } from 'react';
import { useOnboardingStore } from './store';

// OHC Premium Design Tokens: Outfit/Inter fonts, Glassmorphism, accessible contrast.
// We simulate these with tailwind classes for now, ensuring 375px responsiveness.

type Message = {
  id: string;
  sender: 'ai' | 'user';
  text: string;
  isWidget?: boolean;
};

export default function OnboardingWizard() {
  const {
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    template, setTemplate,
    domain,
    isLoading, setIsLoading,
    error, setError,
    intakeData, setIntakeData,
    startResult, setStartResult
  } = useOnboardingStore();

  const [messages, setMessages] = useState<Message[]>([
    { id: '1', sender: 'ai', text: "Hi there! I'm The Advisor. What kind of business are you starting? Tell me about your products or services." }
  ]);
  const [inputValue, setInputValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isLoading, startResult]);

  const handleSend = async () => {
    if (!inputValue.trim() || isLoading) return;

    const userMsg: Message = { id: Date.now().toString(), sender: 'user', text: inputValue };
    setMessages(prev => [...prev, userMsg]);
    setInputValue("");
    setError("");
    setIsLoading(true);

    try {
      const response = await fetch('/api/onboarding/intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: userMsg.text }),
      });

      if (!response.ok) {
        throw new Error('Failed to process intake');
      }

      const data = await response.json();
      setIntakeData(data);

      // Add AI response with widget
      setMessages(prev => [...prev, {
        id: Date.now().toString(),
        sender: 'ai',
        text: "I've generated a draft of your storefront! Review it below and let me know if you want to publish.",
        isWidget: true
      }]);

    } catch (err: any) {
      setError(err.message || 'An error occurred during intake.');
      setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'ai', text: "Sorry, I had trouble understanding that. Could you try again?" }]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleStartOnboarding = async () => {
    setIsLoading(true);
    setError("");

    try {
      const startRequest = {
        business_type: intakeData.business_type || "Retail",
        company_name: intakeData.business_name || "My Business",
        company_description: "",
        selling_categories: intakeData.categories || [],
        payment_pref: "stripe",
        admin_email: "admin@example.com",
        admin_name: "Admin",
        admin_password: "password123",
        website_template: template,
        domain: domain,
        first_product_name: firstProductName || intakeData.initial_products?.[0]?.name || "Sample Product",
        first_product_price: firstProductPrice || intakeData.initial_products?.[0]?.price || "10.00",
      };

      const response = await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(startRequest),
      });

      if (!response.ok) {
        throw new Error('Failed to start onboarding');
      }

      const data = await response.json();
      setStartResult(data);
    } catch (err: any) {
      setError(err.message || 'An error occurred starting your business.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div id="setup-screen" className="flex flex-col items-center justify-center min-h-screen bg-gray-50 dark:bg-[#000] font-inter">
      <style dangerouslySetInnerHTML={{__html: `
        .glass-container {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.4);
          box-shadow:
            0 8px 32px 0 rgba(31, 38, 135, 0.1),
            inset 0 0 0 1px rgba(255, 255, 255, 0.3);
        }
        @media (prefers-color-scheme: dark) {
          .glass-container {
            background: rgba(22, 22, 26, 0.7);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.1);
            box-shadow:
              0 8px 32px 0 rgba(0, 0, 0, 0.4),
              inset 0 0 0 1px rgba(255, 255, 255, 0.05);
          }
          .glass-container h1, .glass-container h2, .glass-container .text-[#1D1D1F] dark:text-[#F5F5F7] dark:text-[#F5F5F7] {
            color: #F5F5F7;
          }
          .glass-container p, .glass-container .text-gray-500 dark:text-[#A1A1A6] dark:text-[#A1A1A6] {
            color: #A1A1A6;
          }
          .glass-container input, .glass-container textarea, .glass-container .bg-white\\/80 {
            background: rgba(0, 0, 0, 0.4);
            color: #F5F5F7;
            border-color: rgba(255, 255, 255, 0.15);
            box-shadow: inset 0 2px 4px rgba(0,0,0,0.2);
          }
          .glass-container input:focus, .glass-container textarea:focus {
            box-shadow: 0 0 0 2px rgba(0, 102, 255, 0.5), inset 0 2px 4px rgba(0,0,0,0.2);
          }
        }
        .animate-fade-in {
          animation: fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1);
        }
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }
      `}} />
      <div className="w-full max-w-[375px] mx-auto h-[100dvh] sm:h-[812px] shadow-2xl flex flex-col relative sm:rounded-[16px] overflow-hidden glass-container mac-glass-container backdrop-blur-xl bg-white/30">

        {/* Header */}
        <div className="w-full p-4 pt-12 flex justify-between items-center z-10 border-b border-white/20 dark:border-white/5 bg-white/40 dark:bg-black/20 backdrop-blur-md shrink-0">
           <h1 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">The Advisor</h1>
           <div className="w-8 h-8 rounded-full bg-gradient-to-tr from-[#0066FF] to-[#0052cc] flex items-center justify-center text-white text-xs shadow-sm">AI</div>
        </div>

        {/* Chat Feed */}
        <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
          {startResult ? (
            <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in my-auto">
              <div className="w-20 h-20 bg-green-50 rounded-full flex items-center justify-center mb-6">
                <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] mb-2">You're Live!</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
                {startResult.message || "Your business has been successfully launched."}
              </p>

              <div className="w-full space-y-3">
                <a
                  href="/dashboard"
                  className="block w-full bg-[#1D1D1F] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-black active:scale-[0.98] transition-all"
                >
                  Go to Dashboard
                </a>
                <a
                  href="/builder"
                  className="block w-full bg-white/70 dark:bg-white/10 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 p-4 rounded-[8px] font-bold shadow-sm hover:bg-white/90 dark:hover:bg-white/20 active:scale-[0.98] transition-all"
                >
                  Preview Storefront
                </a>
              </div>
            </div>
          ) : (
            <>
              {error && (
                <div className="p-3 bg-red-50 border border-red-200 text-red-700 text-sm rounded-xl mb-2 text-center animate-fade-in">
                  {error}
                </div>
              )}
              {messages.map((msg) => (
                <div key={msg.id} className={`flex flex-col animate-fade-in ${msg.sender === 'user' ? 'items-end' : 'items-start'}`}>
                  <div className={`max-w-[85%] p-3 rounded-[16px] text-sm ${msg.sender === 'user' ? 'bg-[#0066FF] text-white rounded-br-sm' : 'bg-white/70 dark:bg-black/40 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 shadow-sm rounded-bl-sm'}`}>
                    {msg.text}
                  </div>

                  {msg.isWidget && intakeData && (
                    <div className="mt-3 w-full max-w-full bg-white/80 dark:bg-black/30 backdrop-blur-md p-4 rounded-[16px] border border-white/50 dark:border-white/10 shadow-sm animate-fade-in">
                      <div className="flex items-center justify-between mb-3 border-b border-gray-200 dark:border-white/10 pb-2">
                        <span className="font-bold text-sm text-[#1D1D1F] dark:text-white">Storefront Preview</span>
                        <span className="text-xs bg-blue-100 dark:bg-blue-900/40 text-[#0066FF] px-2 py-1 rounded-md">{intakeData.business_type}</span>
                      </div>

                      <div className="space-y-4">
                        <div>
                          <label className="text-[10px] font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wider block mb-1">First Product / Service</label>
                          <div className="flex gap-2">
                            <input
                              type="text"
                              value={firstProductName || (intakeData.initial_products?.[0]?.name || '')}
                              onChange={(e) => setFirstProductName(e.target.value)}
                              className="flex-1 p-2 text-sm rounded-[6px] border border-gray-200 dark:border-white/10 bg-white dark:bg-black/40 text-[#1D1D1F] dark:text-white outline-none focus:border-[#0066FF]"
                              placeholder="Name"
                            />
                            <input
                              type="text"
                              value={firstProductPrice || (intakeData.initial_products?.[0]?.price || '')}
                              onChange={(e) => setFirstProductPrice(e.target.value)}
                              className="w-20 p-2 text-sm rounded-[6px] border border-gray-200 dark:border-white/10 bg-white dark:bg-black/40 text-[#1D1D1F] dark:text-white outline-none focus:border-[#0066FF]"
                              placeholder="$0.00"
                            />
                          </div>
                        </div>

                        <div>
                          <label className="text-[10px] font-semibold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wider block mb-1">Theme</label>
                          <div className="flex gap-2 overflow-x-auto pb-1 hide-scrollbar">
                            {['Modern', 'Elegant', 'Minimal'].map((t) => (
                              <button
                                key={t}
                                onClick={() => setTemplate(t)}
                                className={`whitespace-nowrap px-3 py-1.5 rounded-[6px] text-xs transition-colors border ${template === t ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] font-medium' : 'border-gray-200 dark:border-white/10 text-gray-600 dark:text-gray-300'}`}
                              >
                                {t}
                              </button>
                            ))}
                          </div>
                        </div>

                        <button
                          onClick={handleStartOnboarding}
                          disabled={isLoading}
                          className="w-full mt-2 bg-gradient-to-r from-[#34C759] to-[#2eb350] text-white p-3 rounded-[8px] font-bold shadow-md hover:shadow-lg active:scale-[0.98] transition-all disabled:opacity-70 text-sm flex justify-center items-center"
                        >
                          {isLoading ? (
                            <span className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                          ) : (
                            "Publish Now"
                          )}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              ))}

              {isLoading && !msgHasWidgetPending() && (
                <div className="flex items-start animate-fade-in">
                  <div className="p-3 rounded-[16px] rounded-bl-sm bg-white/70 dark:bg-black/40 backdrop-blur-md border border-white/50 dark:border-white/10">
                    <div className="flex space-x-1 items-center h-5">
                      <div className="w-1.5 h-1.5 bg-[#0066FF] rounded-full animate-bounce [animation-delay:-0.3s]"></div>
                      <div className="w-1.5 h-1.5 bg-[#0066FF] rounded-full animate-bounce [animation-delay:-0.15s]"></div>
                      <div className="w-1.5 h-1.5 bg-[#0066FF] rounded-full animate-bounce"></div>
                    </div>
                  </div>
                </div>
              )}
              <div ref={messagesEndRef} />
            </>
          )}
        </div>

        {/* Chat Input */}
        {!startResult && (
          <div className="p-4 bg-white/40 dark:bg-black/20 backdrop-blur-md border-t border-white/20 dark:border-white/5 shrink-0">
            <div className="relative flex items-center">
              <input
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') handleSend();
                }}
                disabled={isLoading || !!intakeData} // Disable input once intake is done and widget is showing
                placeholder={intakeData ? "Review the generated storefront above..." : "Message The Advisor..."}
                className="w-full bg-white/60 dark:bg-black/40 backdrop-blur-md border border-white/50 dark:border-white/10 rounded-full py-3 pl-4 pr-12 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none focus:border-[#0066FF] shadow-inner text-sm disabled:opacity-50"
              />
              <button
                onClick={handleSend}
                disabled={!inputValue.trim() || isLoading || !!intakeData}
                className="absolute right-2 w-8 h-8 rounded-full bg-[#0066FF] text-white flex items-center justify-center disabled:bg-gray-300 dark:disabled:bg-gray-700 transition-colors"
              >
                <svg className="w-4 h-4 translate-x-[1px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </button>
            </div>
            <p className="text-[10px] text-center text-gray-500 mt-2">AI can make mistakes. Verify important info.</p>
          </div>
        )}
      </div>
    </div>
  );

  function msgHasWidgetPending() {
    return false; // we show loading whenever isLoading is true and handled
  }
}
