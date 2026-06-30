'use client';
import React, { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';

function generateSubdomain(name: string): string {
  if (!name || name.trim() === '') return 'my-business.ohc.app';
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return cleanName ? `${cleanName}.ohc.app` : 'my-business.ohc.app';
}

export default function OnboardingChat() {
  const router = useRouter();

  const [messages, setMessages] = useState([
    {
      id: '1',
      sender: 'assistant',
      text: "Hi there! I'm your OHC Work Assistant. What kind of work do you do?"
    }
  ]);
  const [input, setInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const [showApprovalCard, setShowApprovalCard] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [isLive, setIsLive] = useState(false);
  const [errorMessage, setErrorMessage] = useState('');
  const [config, setConfig] = useState<any>(null);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === 'function') {
      messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isTyping, showApprovalCard]);

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

  const handleSend = async () => {
    if (!input.trim()) return;

    const userText = input.trim();
    setInput('');
    setErrorMessage('');

    setMessages(prev => [...prev, { id: Date.now().toString(), sender: 'user', text: userText }]);
    setIsTyping(true);

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    try {
      const res = await fetchWithRetry('/api/onboarding/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify({ messages: [{ role: 'user', content: userText }] })
      });
      const data = await res.json();

      setIsTyping(false);

      setConfig({
        businessName: data.intake_data?.business_name || userText,
        firstProductName: data.intake_data?.initial_products?.[0]?.name || 'Custom Service',
        categories: data.intake_data?.categories || ['Service'],
        businessType: data.intake_data?.business_type || 'services',
        location: data.intake_data?.location,
        targetAudience: data.intake_data?.target_audience
      });

      setMessages(prev => [...prev, {
        id: Date.now().toString(),
        sender: 'assistant',
        text: data.reply || "Got it! I've drafted a complete storefront and operation setup for you based on that. Look good?"
      }]);
      setShowApprovalCard(true);
    } catch (err: any) {
      setIsTyping(false);
      setErrorMessage(err.message || 'Failed to process request');
    }
  };

  const handleApprove = async () => {
    setIsDeploying(true);
    setErrorMessage('');

    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    try {
      const startPayload = {
        company_name: config?.businessName || 'My Business',
        business_type: config?.businessType || 'services',
        selling_categories: config?.categories || ['Service'],
        first_product_name: config?.firstProductName || 'Service',
        location: config?.location,
        target_audience: config?.targetAudience,
        ai_auto_respond: true,
        ai_agents: ['Work Triage', 'Customer Assistant']
      };

      const startRes = await fetchWithRetry('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-Tenant-ID': tenantId, 'X-User-ID': userId },
        body: JSON.stringify(startPayload)
      });
      await startRes.json();

      const launchRes = await fetchWithRetry('/api/onboarding/launch', {
        method: 'POST',
        headers: { 'X-Tenant-ID': tenantId, 'X-User-ID': userId }
      });
      await launchRes.json();

      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_onboarded', 'true');
        localStorage.setItem('tenant', tenantId);
        localStorage.setItem('businessName', startPayload.company_name);
      }

      setIsDeploying(false);
      setIsLive(true);
    } catch (err: any) {
      setIsDeploying(false);
      setErrorMessage(err.message || 'Failed to launch');
    }
  };

  if (isLive) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col items-center justify-center p-4">
        <div className="w-full max-w-md bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] rounded-2xl p-8 text-center shadow-lg border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
          <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6 mx-auto">
            <svg className="w-10 h-10 text-[#34C759]" width="40" height="40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h2 className="text-2xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">You\'re Live!</h2>
          <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8">
            Your business has been successfully launched.
          </p>
          <div className="space-y-4">
            <button
              onClick={() => router.push('/assistant')}
              className="w-full bg-[#0066FF] text-white p-4 font-bold rounded-[8px] hover:bg-[#0052cc] transition-colors"
            >
              Open Assistant
            </button>
            <button
              onClick={() => router.push('/builder')}
              className="w-full bg-white dark:bg-[#2C2C2E] text-[#1D1D1F] dark:text-white p-4 font-bold rounded-[8px] shadow-sm hover:bg-gray-50 dark:hover:bg-[#3C3C3E] transition-colors border border-gray-200 dark:border-gray-700"
            >
              Preview Storefront
            </button>
          </div>
        </div>
      </div>
    );
  }

  if (isDeploying) {
    return (
      <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col items-center justify-center p-4">
        <div className="w-full max-w-md text-center">
          <div className="w-24 h-24 relative mb-8 mx-auto">
            <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
            <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
          </div>
          <h2 className="text-2xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
          <p className="text-gray-500 dark:text-[#A1A1A6]">Generating product catalog, configuring payments, and onboarding your AI agents.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] flex flex-col items-center justify-center p-4 md:p-8">
      <div className="w-full max-w-md h-[85vh] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] rounded-2xl shadow-lg border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col overflow-hidden relative">

        {/* Error Banner */}
        {errorMessage && (
          <div className="absolute top-0 left-0 right-0 z-10 bg-[#FF3B30] text-white p-3 text-sm font-semibold text-center shadow-md animate-fade-in flex items-center justify-between">
            <span className="flex-1">{errorMessage}</span>
            <button onClick={() => setErrorMessage('')} className="p-1 hover:bg-black/10 rounded-full">
              <svg className="w-4 h-4" width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
          </div>
        )}

        {/* Header */}
        <div className="p-4 border-b border-[rgba(0,0,0,0.05)] dark:border-[rgba(255,255,255,0.05)] bg-white/50 dark:bg-black/20 flex items-center gap-3">
          <div className="w-10 h-10 bg-gradient-to-tr from-[#0066FF] to-[#5AC8FA] rounded-full flex items-center justify-center shadow-sm">
            <svg className="w-5 h-5 text-white" width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </div>
          <div>
            <h1 className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">OHC Assistant</h1>
            <p className="text-xs text-gray-500 dark:text-[#A1A1A6]">Setting up your business</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.map((msg) => (
            <div key={msg.id} className={`flex ${msg.sender === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`max-w-[85%] p-3 rounded-2xl ${
                msg.sender === 'user'
                  ? 'bg-[#0066FF] text-white rounded-tr-sm'
                  : 'bg-white dark:bg-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] rounded-tl-sm shadow-sm border border-gray-100 dark:border-gray-800'
              }`}>
                <p className="text-[15px] leading-relaxed">{msg.text}</p>
              </div>
            </div>
          ))}

          {isTyping && (
            <div className="flex justify-start">
              <div className="bg-white dark:bg-[#2C2C2E] p-4 rounded-2xl rounded-tl-sm shadow-sm border border-gray-100 dark:border-gray-800 flex gap-1.5 items-center">
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }}></div>
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }}></div>
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }}></div>
              </div>
            </div>
          )}

          {/* Rich Approval Card */}
          {showApprovalCard && (
            <div className="flex justify-start w-full">
              <div className="w-full bg-white dark:bg-[#2C2C2E] rounded-xl shadow-sm border border-[#0066FF]/30 overflow-hidden mt-2">
                <div className="bg-[#0066FF]/5 dark:bg-[#0066FF]/10 p-3 border-b border-gray-100 dark:border-gray-800 flex items-center gap-2">
                  <svg className="w-5 h-5 text-[#0066FF]" width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                  <span className="font-semibold text-sm text-[#1D1D1F] dark:text-[#F5F5F7]">Proposed Configuration</span>
                </div>
                <div className="p-4 space-y-3">
                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-full bg-orange-100 dark:bg-orange-900/30 flex items-center justify-center flex-shrink-0 mt-0.5">
                      <svg className="w-4 h-4 text-orange-600 dark:text-orange-400" width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" /></svg>
                    </div>
                    <div>
                      <h4 className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Product Setup</h4>
                      <p className="text-xs text-gray-500 dark:text-[#A1A1A6]">"{config?.firstProductName || 'Custom Deposit'}" item created with variable pricing.</p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="w-8 h-8 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center flex-shrink-0 mt-0.5">
                      <svg className="w-4 h-4 text-green-600 dark:text-green-400" width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
                    </div>
                    <div>
                      <h4 className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Delivery Zones</h4>
                      <p className="text-xs text-gray-500 dark:text-[#A1A1A6]">Configured for "Local Pickup Only" with automated scheduling.</p>
                    </div>
                  </div>
                </div>
                <div className="p-3 bg-gray-50 dark:bg-[#1C1C1E] border-t border-gray-100 dark:border-gray-800">
                  <button
                    onClick={handleApprove}
                    className="w-full bg-[#0066FF] text-white py-3 rounded-lg font-bold shadow-sm hover:bg-[#0052cc] transition-colors flex items-center justify-center gap-2"
                  >
                    Approve & Go Live
                    <svg className="w-4 h-4" width="16" height="16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>
                  </button>
                </div>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        {!showApprovalCard && (
          <div className="p-4 bg-white/80 dark:bg-black/40 backdrop-blur-md border-t border-[rgba(0,0,0,0.05)] dark:border-[rgba(255,255,255,0.05)]">
            <div className="flex items-center gap-2">
              <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                placeholder="Type your response..."
                className="flex-1 bg-gray-100 dark:bg-[#2C2C2E] border-transparent rounded-full px-4 py-3 text-[15px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 text-[#1D1D1F] dark:text-white"
                disabled={isTyping}
              />
              <button
                onClick={handleSend}
                disabled={!input.trim() || isTyping}
                className="w-11 h-11 bg-[#0066FF] text-white rounded-full flex items-center justify-center disabled:opacity-50 disabled:bg-gray-400 transition-colors flex-shrink-0"
              >
                <svg className="w-5 h-5 ml-1" width="20" height="20" fill="currentColor" viewBox="0 0 20 20"><path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" /></svg>
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
