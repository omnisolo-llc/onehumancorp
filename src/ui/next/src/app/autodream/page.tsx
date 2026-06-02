'use client';

import React, { useState, useEffect, useRef } from 'react';
import { useRouter } from 'next/navigation';

interface Message {
  role: 'user' | 'agent' | 'system';
  content: string;
  id: string;
}

export default function AutoDreamPage() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [scaffoldResult, setScaffoldResult] = useState<any>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const router = useRouter();

  useEffect(() => {
    // Initial greeting
    setMessages([
      {
        role: 'agent',
        content: "Hi! I'm your Operations Manager. Tell me what kind of business you want to start, and I'll build it for you in seconds. For example: 'I bake vegan cakes in Austin' or 'I am a freelance handyman'.",
        id: 'msg-0'
      }
    ]);
  }, []);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, scaffoldResult, isLoading]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || isLoading) return;

    const userMessage: Message = { role: 'user', content: input, id: `msg-${Date.now()}` };
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    // Simulate Agent reasoning messages
    const reasoningMessages = [
      "Analyzing business type...",
      "Drafting your menu...",
      "Configuring local taxes...",
      "Generating storefront design..."
    ];

    for (let i = 0; i < reasoningMessages.length; i++) {
      await new Promise(resolve => setTimeout(resolve, 800));
      setMessages(prev => [...prev, { role: 'system', content: reasoningMessages[i], id: `sys-${Date.now()}-${i}` }]);
    }

    try {
      const res = await fetch('/api/autodream/scaffold', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: userMessage.content })
      });

      if (!res.ok) {
        throw new Error("Failed to scaffold business");
      }

      const data = await res.json();
      setScaffoldResult(data);

      setMessages(prev => [...prev, {
        role: 'agent',
        content: `I've prepared a draft for your business: **${data.business_name}**. I've added ${data.products?.length || 0} initial products and configured your default settings.`,
        id: `msg-agent-${Date.now()}`
      }]);

    } catch (err) {
      setMessages(prev => [...prev, {
        role: 'agent',
        content: "Sorry, I ran into an issue building your business. Please try again.",
        id: `msg-err-${Date.now()}`
      }]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleApprove = async () => {
    // In a real app, we'd confirm the tenant creation. Here we just redirect to the dashboard.
    if (scaffoldResult) {
      localStorage.setItem('tenant_id', scaffoldResult.tenant_id || 'autodream-tenant');
      localStorage.setItem('tenant_name', scaffoldResult.business_name || 'My New Business');
    }
    router.push('/dashboard');
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-black font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
      <div className="max-w-[375px] mx-auto h-screen flex flex-col bg-white dark:bg-[#111] shadow-2xl relative overflow-hidden">

        {/* Header */}
        <div className="p-4 border-b border-gray-200 dark:border-white/10 mac-glass-container sticky top-0 z-10 flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-blue-500 to-purple-500 flex items-center justify-center text-white shadow-lg">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <div>
            <h1 className="font-bold text-lg leading-tight">Operations Manager</h1>
            <p className="text-xs text-green-500 font-semibold">Online</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4 pb-24">
          {messages.map(msg => (
            <div key={msg.id} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
              <div className={`
                max-w-[85%] p-3 rounded-2xl text-sm
                ${msg.role === 'user'
                  ? 'bg-[#0066FF] text-white rounded-tr-sm'
                  : msg.role === 'system'
                    ? 'bg-transparent text-gray-400 italic text-xs py-1'
                    : 'bg-gray-100 dark:bg-white/10 text-[#1D1D1F] dark:text-white rounded-tl-sm shadow-sm'
                }
              `}>
                {msg.content}
              </div>
            </div>
          ))}

          {isLoading && (
             <div className="flex justify-start">
               <div className="bg-gray-100 dark:bg-white/10 text-gray-500 p-3 rounded-2xl rounded-tl-sm flex gap-1 items-center h-10 shadow-sm">
                 <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce"></div>
                 <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                 <div className="w-2 h-2 rounded-full bg-gray-400 animate-bounce" style={{ animationDelay: '0.4s' }}></div>
               </div>
             </div>
          )}

          {/* Preview Card (Agentic Solution) */}
          {scaffoldResult && !isLoading && (
            <div className="mt-4 p-4 rounded-xl border border-blue-500/30 bg-blue-50 dark:bg-blue-900/20 animate-fade-in shadow-md">
              <div className="flex items-center gap-2 mb-3">
                <svg className="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                <h3 className="font-bold text-blue-700 dark:text-blue-300">Draft Ready for Review</h3>
              </div>
              <div className="space-y-2 text-sm mb-4">
                <div className="flex justify-between border-b border-blue-200/50 dark:border-blue-800/50 pb-1">
                  <span className="text-gray-500 dark:text-gray-400">Business</span>
                  <span className="font-semibold">{scaffoldResult.business_name}</span>
                </div>
                <div className="flex justify-between border-b border-blue-200/50 dark:border-blue-800/50 pb-1">
                  <span className="text-gray-500 dark:text-gray-400">Type</span>
                  <span className="font-semibold">{scaffoldResult.business_type}</span>
                </div>
                <div className="flex justify-between pb-1">
                  <span className="text-gray-500 dark:text-gray-400">Products</span>
                  <span className="font-semibold">{scaffoldResult.products?.length || 0} added</span>
                </div>
              </div>

              <button
                onClick={handleApprove}
                className="w-full bg-[#0066FF] hover:bg-blue-600 active:scale-95 transition-all text-white font-bold py-3 rounded-lg shadow-lg"
              >
                Approve & Launch
              </button>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="absolute bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-white via-white dark:from-[#111] dark:via-[#111] to-transparent pt-10">
          <form onSubmit={handleSubmit} className="relative flex items-center">
            <input
              type="text"
              value={input}
              onChange={e => setInput(e.target.value)}
              placeholder={scaffoldResult ? "Ask for changes..." : "Describe your business..."}
              disabled={isLoading}
              className="w-full bg-white dark:bg-black border border-gray-200 dark:border-white/20 rounded-full pl-5 pr-12 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] shadow-lg disabled:opacity-50"
            />
            <button
              type="submit"
              disabled={!input.trim() || isLoading}
              className="absolute right-2 w-8 h-8 flex items-center justify-center bg-[#0066FF] text-white rounded-full hover:bg-blue-600 disabled:opacity-50 transition-colors"
            >
              <svg className="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
            </button>
          </form>
        </div>

      </div>
    </div>
  );
}
