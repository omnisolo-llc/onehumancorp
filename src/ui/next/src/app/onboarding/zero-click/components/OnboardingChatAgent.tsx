import React, { useState, useEffect, useRef } from 'react';

type ChatMessage = {
  role: 'user' | 'assistant';
  content: string;
};

type IntakeProduct = {
  name: string;
  price: string;
  description?: string;
  variants?: any[];
};

type IntakeData = {
  location?: string;
  target_audience?: string;
  business_name: string;
  business_type: string;
  categories: string[];
  initial_products: IntakeProduct[];
  initial_tasks?: string[];
  sample_customer_name?: string;
  sample_customer_email?: string;
  deposit_percentage?: number;
  lead_time_days?: number;
};

export const OnboardingChatAgent = ({ onComplete }: { onComplete: (data: any) => void }) => {
  const [messages, setMessages] = useState<ChatMessage[]>([
    { role: 'assistant', content: "Hi! I'm your OHC setup agent. In one sentence, tell me about the business you want to build." }
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isProvisioning, setIsProvisioning] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === 'function') {
      try {
        messagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
      } catch (e) {
        // Ignore scroll errors in tests
      }
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isLoading, isProvisioning]);

  const handleSend = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!input.trim() || isLoading || isProvisioning) return;

    const userMessage: ChatMessage = { role: 'user', content: input.trim() };
    const newMessages = [...messages, userMessage];
    setMessages(newMessages);
    setInput('');
    setIsLoading(true);

    try {
      const response = await fetch('/api/v1/onboarding/start_zero_click', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: newMessages.filter(m => m.role === 'user').map(m => m.content).join(" ") }),
      });

      if (!response.ok) throw new Error('Failed to communicate with setup agent');

      const data = await response.json();

      setIsLoading(false);
      setIsProvisioning(true);

      setTimeout(() => {
        setIsProvisioning(false);
        onComplete(data);
      }, 1500);
    } catch (error) {
      console.error("Provisioning error:", error);
      setIsLoading(false);
      setMessages(prev => [...prev, { role: 'assistant', content: "Sorry, I ran into an issue processing that. Please try again." }]);
    }
  };

  const predefinedChips = [
    "I'm a local baker selling custom cakes",
    "I run a neighborhood handyman service",
    "I am an online music tutor",
    "I manage 15 long-term apartment rentals"
  ];

  return (
    <div className="flex flex-col min-h-[50vh] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] overflow-hidden shadow-xl rounded-[16px] w-full max-w-2xl mx-auto">
      {/* Header */}
      <div className="p-4 border-b border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex items-center gap-3 bg-transparent">
        <div className="w-10 h-10 rounded-full bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center text-xl">
          ✨
        </div>
        <div>
          <h3 className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">OHC Setup Assistant</h3>
          <p className="text-xs text-[#424245] dark:text-[#A1A1A6]">Usually replies instantly</p>
        </div>
      </div>

      {/* Chat Area */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 min-h-[300px] max-h-[500px]">
        {messages.map((msg, idx) => (
          <div key={idx} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[80%] rounded-[16px] px-4 py-3 ${
              msg.role === 'user'
                ? 'bg-[#0066FF] text-white rounded-br-sm shadow-[0_4px_14px_0_rgba(0,102,255,0.39)]'
                : 'bg-gray-100 dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-bl-sm border border-gray-200 dark:border-gray-700'
            }`}>
              {msg.content}
            </div>
          </div>
        ))}

        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] rounded-bl-sm px-4 py-3 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex gap-1 items-center">
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
              <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0.4s' }}></div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Provisioning Overlay */}
      {isProvisioning && (
        <div className="absolute inset-0 z-10 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[10px] flex flex-col items-center justify-center rounded-[16px]">
          <div className="w-16 h-16 border-4 border-[#0066FF]/20 border-t-[#0066FF] rounded-full animate-spin mb-6"></div>
          <h3 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 animate-pulse">
            Building Your Business...
          </h3>
          <p className="text-sm text-gray-500 font-medium">Provisioning workspace, products, and agents.</p>
        </div>
      )}

      {/* Input Area */}
      <div className="p-4 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-transparent">
        {messages.length === 1 && (
          <div className="flex flex-wrap gap-2 mb-4">
            {predefinedChips.map((chip, idx) => (
              <button
                key={idx}
                onClick={() => {
                  setInput(chip);
                  // Optional: auto send after setting
                  // setTimeout(() => handleSend(), 0);
                }}
                className="text-xs font-medium min-h-[44px] px-4 py-2 flex items-center justify-center bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] hover:bg-[rgba(255,255,255,0.8)] dark:hover:bg-[rgba(22,22,26,0.9)] rounded-full text-[#1D1D1F] dark:text-[#F5F5F7] transition-colors border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]"
              >
                {chip}
              </button>
            ))}
          </div>
        )}

        <form onSubmit={handleSend} className="relative flex items-center gap-2">
          <input
            id="instant-bio"
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            disabled={isLoading || isProvisioning}
            placeholder="e.g. I am a home baker in Austin selling custom vegan cakes."
            className="w-full bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-full py-3.5 pl-4 pr-12 min-h-[44px] text-[#1D1D1F] dark:text-[#F5F5F7] focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:opacity-50"
          />
          <button
            id="generate-storefront-btn"
            data-testid="generate-storefront-btn"
            type="submit"
            disabled={!input.trim() || isLoading || isProvisioning}
            className="absolute right-1 top-1.5 w-10 h-10 flex items-center justify-center bg-[#0066FF] hover:bg-[#005bb5] disabled:bg-gray-400 text-white rounded-full transition-colors"
          >
            <svg className="w-4 h-4 translate-x-[1px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
            </svg>
          </button>
        </form>
      </div>
    </div>
  );
};
