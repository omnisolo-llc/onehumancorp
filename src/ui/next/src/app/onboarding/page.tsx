"use client";

import React, { useState, useRef, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { motion, AnimatePresence } from 'framer-motion';

interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}

export default function AutonomousOnboarding() {
  const router = useRouter();
  const [messages, setMessages] = useState<ChatMessage[]>([
    { role: 'assistant', content: "Hi! Let's get your business online. What do you sell?" }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [setupComplete, setSetupComplete] = useState(false);

  // Credentials collection state
  const [needsCredentials, setNeedsCredentials] = useState(false);
  const [adminName, setAdminName] = useState('');
  const [adminEmail, setAdminEmail] = useState('');
  const [adminPassword, setAdminPassword] = useState('');

  const [intakeData, setIntakeData] = useState<any>(null);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === 'function') {
        messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isProcessing]);

  const handleStartOnboarding = async () => {
    if (!intakeData) return;
    setIsProcessing(true);
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
          business_type: intakeData.business_type || "Online Business",
          company_name: intakeData.business_name || "My Business",
          company_description: intakeData.business_name || "",
          selling_categories: intakeData.categories || ["physical"],
          payment_pref: "online",
          admin_email: adminEmail || "owner@example.com",
          admin_name: adminName || "Owner",
          admin_password: adminPassword || "password123!",
          website_template: "Modern",
          first_product_name: intakeData.initial_products?.[0]?.name || "Product",
          first_product_price: intakeData.initial_products?.[0]?.price || "10.00",
          domain_choice: "subdomain",
          price_type: "fixed",
          location: intakeData.location || "",
          target_audience: intakeData.target_audience || "",
          initial_products: intakeData.initial_products || [],
          ai_agents: ["Operations", "Marketing", "Customer Success"],
          ai_auto_respond: true,
        })
      });

      if (startRes.ok) {
         setSetupComplete(true);
         setNeedsCredentials(false);
         setMessages(prev => [...prev, { role: 'assistant', content: "Great! I've set up your business. Here is a preview link. Would you like me to connect a bank account to start taking deposits?" }]);
      } else {
         throw new Error("Failed to start onboarding");
      }
    } catch (err) {
      console.error(err);
      setMessages(prev => [...prev, { role: 'assistant', content: "Oops, something went wrong while creating your account. Let's try that again." }]);
    } finally {
      setIsProcessing(false);
    }
  }

  const handleSend = async () => {
    if (!inputValue.trim()) return;

    const newMessages = [...messages, { role: 'user' as const, content: inputValue.trim() }];
    setMessages(newMessages);
    setInputValue('');
    setIsProcessing(true);

    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront';
      const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

      const res = await fetch('/api/onboarding/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ messages: newMessages })
      });

      if (!res.ok) throw new Error('Chat failed');
      const data = await res.json();

      setMessages(prev => [...prev, { role: 'assistant', content: data.reply }]);

      if (data.is_complete && data.intake_data) {
        setIntakeData(data.intake_data);
        setNeedsCredentials(true);
      }

    } catch (err) {
      console.error(err);
      setMessages(prev => [...prev, { role: 'assistant', content: "Oops, something went wrong. Let's try that again." }]);
    } finally {
      setIsProcessing(false);
    }
  };

  const navigateToDashboard = () => {
    router.push('/dashboard');
  };

  return (
    <div className="flex flex-col h-screen w-full bg-[#F5F5F7] dark:bg-[#16161a] items-center justify-center font-sans">
      <div className="w-full max-w-[375px] h-full sm:h-[812px] sm:max-h-screen bg-white/40 dark:bg-black/40 backdrop-blur-xl sm:rounded-[32px] sm:border border-white/20 shadow-2xl flex flex-col relative overflow-hidden">

        {/* Header */}
        <div className="flex items-center justify-center p-4 border-b border-gray-200/50 dark:border-gray-800/50 bg-white/50 dark:bg-black/50 backdrop-blur-md z-10">
          <h1 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">OHC Setup Agent</h1>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4 custom-scrollbar relative z-0">
          <AnimatePresence>
            {messages.map((msg, idx) => (
              <motion.div
                key={idx}
                initial={{ opacity: 0, y: 10, scale: 0.95 }}
                animate={{ opacity: 1, y: 0, scale: 1 }}
                transition={{ duration: 0.3 }}
                className={`flex w-full ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
              >
                <div
                  className={`max-w-[85%] p-3 rounded-[20px] text-[15px] leading-relaxed shadow-sm ${
                    msg.role === 'user'
                      ? 'bg-[#007AFF] text-white rounded-tr-sm'
                      : 'bg-white/80 dark:bg-[#2C2C2E]/80 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] rounded-tl-sm border border-white/20'
                  }`}
                >
                  {msg.content}
                </div>
              </motion.div>
            ))}

            {isProcessing && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex w-full justify-start"
              >
                <div className="max-w-[85%] p-4 rounded-[20px] bg-white/80 dark:bg-[#2C2C2E]/80 backdrop-blur-md text-[#1D1D1F] dark:text-[#F5F5F7] rounded-tl-sm border border-white/20 flex items-center space-x-2">
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                  <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
          <div ref={messagesEndRef} />
        </div>

        {/* Input Area */}
        <div className="p-4 bg-white/60 dark:bg-black/60 backdrop-blur-xl border-t border-gray-200/50 dark:border-gray-800/50 z-10">
          {!setupComplete && !needsCredentials && (
            <div className="flex items-center bg-white/50 dark:bg-gray-800/50 rounded-full p-1 border border-gray-300/50 dark:border-gray-700/50 shadow-inner">
              <input
                type="text"
                value={inputValue}
                onChange={(e) => setInputValue(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                placeholder="Type your message..."
                disabled={isProcessing}
                className="flex-1 bg-transparent px-4 py-2 outline-none text-[#1D1D1F] dark:text-[#F5F5F7] placeholder-gray-500 disabled:opacity-50"
              />
              <button
                onClick={handleSend}
                disabled={isProcessing || !inputValue.trim()}
                className="bg-[#007AFF] text-white p-2 rounded-full disabled:opacity-50 hover:bg-[#0066CC] transition-colors"
                aria-label="Send"
              >
                <svg className="w-5 h-5 ml-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </button>
            </div>
          )}

          {needsCredentials && (
            <div className="flex flex-col space-y-3 bg-white/50 dark:bg-gray-800/50 p-4 rounded-[20px] shadow-sm">
              <h3 className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Create Owner Account</h3>
              <input
                type="text"
                value={adminName}
                onChange={(e) => setAdminName(e.target.value)}
                placeholder="Your Name"
                className="w-full bg-white dark:bg-black/50 px-4 py-2 outline-none text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[10px] border border-gray-200 dark:border-gray-700"
              />
              <input
                type="email"
                value={adminEmail}
                onChange={(e) => setAdminEmail(e.target.value)}
                placeholder="Email Address"
                className="w-full bg-white dark:bg-black/50 px-4 py-2 outline-none text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[10px] border border-gray-200 dark:border-gray-700"
              />
              <input
                type="password"
                value={adminPassword}
                onChange={(e) => setAdminPassword(e.target.value)}
                placeholder="Password (8+ chars, 1 number)"
                className="w-full bg-white dark:bg-black/50 px-4 py-2 outline-none text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[10px] border border-gray-200 dark:border-gray-700"
              />
              <button
                onClick={handleStartOnboarding}
                disabled={isProcessing || !adminEmail || !adminPassword || !adminName}
                className="w-full bg-[#007AFF] text-white font-semibold py-3 rounded-[10px] shadow-md hover:bg-[#0066CC] disabled:opacity-50 transition-all"
              >
                Create Business
              </button>
            </div>
          )}

          {setupComplete && (
            <div className="flex flex-col space-y-3">
              <button
                onClick={navigateToDashboard}
                className="w-full bg-[#007AFF] text-white font-semibold py-3.5 rounded-[14px] shadow-md hover:bg-[#0066CC] transition-all transform active:scale-95"
              >
                Go to Owner Dashboard
              </button>
              <button
                onClick={() => setSetupComplete(false)}
                className="w-full bg-transparent text-[#007AFF] font-semibold py-3.5 rounded-[14px] hover:bg-black/5 transition-colors"
              >
                Connect Bank First
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
