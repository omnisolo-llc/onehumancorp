import React, { useState, useRef, useEffect } from 'react';
import { readPreparation, readPreparedResult, resultForPreparation } from '../../contracts';

interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}

interface IntakeData {
  business_name: string;
  business_type: string;
  categories: string[];
  location?: string;
  target_audience?: string;
  initial_products: { name: string; price: string; description?: string; variants?: string[] }[];
}

interface OnboardingChatAgentProps {
  onComplete: (data: import('@/lib/builder-types').OnboardingResult) => void;
}

export function OnboardingChatAgent({ onComplete }: OnboardingChatAgentProps) {
  const [isLoaded, setIsLoaded] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([
    { role: 'assistant', content: "Hi there! I'm your OmniSolo setup assistant. What kind of business do you want to build or manage today?" }
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isProvisioning, setIsProvisioning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [review, setReview] = useState<{ intake: IntakeData; prompt: string } | null>(null);
  const busy = useRef(false);
  const epoch = useRef(0);
  const unknownPreparation = useRef(false);
  useEffect(() => () => { epoch.current += 1; busy.current = false; }, []);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (error) {
      setError(null);
    }
  }, [input]);

  const scrollToBottom = () => {
    if (messagesEndRef.current && typeof messagesEndRef.current.scrollIntoView === 'function') {
      try {
        messagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
      } catch  {
        // Ignore scroll errors in tests
      }
    }
  };

  useEffect(() => {
    let cancelled = false;
    fetch('/api/v1/onboarding/state')
      .then((res) => {
        if (!res.ok) throw new Error('Failed to fetch state');
        return res.json();
      })
      .then((data) => {
        if (cancelled) return;
        if (data?.preparation) { onComplete(resultForPreparation(readPreparation(data.preparation))); return; }
        if (data?.chatMessages && Array.isArray(data.chatMessages) && data.chatMessages.length > 0) {
          setMessages(data.chatMessages);
        }
      })
      .catch((err) => {
        console.error('Failed to load onboarding state', err);
      })
      .finally(() => {
        if (!cancelled) setIsLoaded(true);
      });
    return () => { cancelled = true; };
  }, []);

  useEffect(() => {
    if (isLoaded) {
      scrollToBottom();
    }
  }, [messages, isLoading, isProvisioning, isLoaded]);

  const saveStateToBackend = async (chatMessagesToSave: ChatMessage[]) => {
    try {
      await fetch('/api/v1/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ step: -2, chatMessages: chatMessagesToSave }),
      });
    } catch (err) {
      console.error('Failed to save chat state', err);
    }
  };

  const handleSend = async (e?: React.FormEvent) => {
    e?.preventDefault();
    if (!input.trim() || busy.current) return;
    busy.current = true;
    const version = ++epoch.current;
    const active = () => version === epoch.current;
    const newMessages: ChatMessage[] = [...messages, { role: 'user', content: input.trim() }];
    setMessages(newMessages); setInput(''); setIsLoading(true); setReview(null); setError(null);
    void saveStateToBackend(newMessages);
    try {
      const response = await fetch('/api/v1/onboarding/chat', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ messages: newMessages }) });
      if (!response.ok) throw new Error('Failed to communicate with setup agent');
      const data = await response.json();
      if (!active()) return;
      if (typeof data.reply !== 'string') throw new Error('The setup reply is incomplete');
      const updated: ChatMessage[] = [...newMessages, { role: 'assistant', content: data.reply }];
      setMessages(updated); void saveStateToBackend(updated);
      if (data.is_complete) {
        const intake = data.intake_data;
        if (!intake?.business_name || !Array.isArray(intake.initial_products) || !intake.initial_products.length || intake.initial_products.some((product: IntakeData['initial_products'][number]) => !product.name || product.price == null)) throw new Error('The setup draft is incomplete. Please add your business and product details.');
        setReview({ intake, prompt: newMessages.filter(message => message.role === 'user').map(message => message.content).join(' ') });
      }
    } catch (cause) {
      if (active()) setError(cause instanceof Error ? cause.message : 'Failed to communicate with setup agent');
    } finally {
      if (active()) { busy.current = false; setIsLoading(false); }
    }
  };

  const handleProvisioning = async () => {
    if (!review || busy.current) return;
    busy.current = true;
    const version = ++epoch.current;
    const active = () => version === epoch.current;
    setIsProvisioning(true); setError(null);
    try {
      if (unknownPreparation.current) {
        const response = await fetch('/api/v1/onboarding/state');
        if (!response.ok) throw new Error('Could not check the previous setup. Reload before retrying.');
        const state = await response.json();
        if (!active()) return;
        if (state.preparation) { onComplete(resultForPreparation(readPreparation(state.preparation))); return; }
        unknownPreparation.current = false;
      }
      const intake = review.intake;
      const first = intake.initial_products[0];
      const payload = {
        business_type: intake.business_type || 'Service Business', company_name: intake.business_name,
        company_description: review.prompt, selling_categories: intake.categories || [], payment_pref: 'online',
        website_template: 'Modern', first_product_name: first.name, first_product_price: String(first.price),
        domain_choice: 'subdomain', price_type: 'fixed', location: intake.location || '', target_audience: intake.target_audience || '',
        initial_products: intake.initial_products.map(product => ({ ...product, price: String(product.price), description: product.description || '', variants: product.variants || [] })), ai_agents: [], ai_auto_respond: false,
      };
      const response = await fetch('/api/v1/onboarding/start', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
      if (!response.ok) throw new Error('Setup preparation could not be confirmed. Check its status before retrying.');
      const result = readPreparedResult(await response.json());
      if (active()) onComplete(result);
    } catch (cause) {
      if (active()) { unknownPreparation.current = true; setError(cause instanceof Error ? cause.message : 'Setup preparation could not be confirmed'); }
    } finally {
      if (active()) { busy.current = false; setIsProvisioning(false); }
    }
  };

  const predefinedChips = [
    "I'm a local baker selling custom cakes",
    "I run a neighborhood handyman service",
    "I am an online music tutor",
    "I manage 15 long-term apartment rentals"
  ];

  if (!isLoaded) {
    return (
      <div className="flex items-center justify-center min-h-[50vh] w-full max-w-2xl mx-auto">
        <div className="w-8 h-8 border-4 border-[#0066FF]/20 border-t-[#0066FF] rounded-full animate-spin"></div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-[50vh] glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] overflow-hidden shadow-xl rounded-[16px] w-full max-w-2xl mx-auto">
      {/* Header */}
      <div className="p-4 border-b border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex items-center gap-3 bg-transparent">
        <div className="w-10 h-10 rounded-full bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center text-xl">
          ✨
        </div>
        <div>
          <h3 className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">OmniSolo Setup Assistant</h3>
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
            Preparing your workspace...
          </h3>
          <p className="text-sm text-gray-500 font-medium">Saving your reviewed workspace and catalog.</p>
        </div>
      )}

      {review && <section aria-label="Review setup" className="p-4">
        <h3>Review {review.intake.business_name}</h3>
        <ul>{review.intake.initial_products.map((product, index) => <li key={index}>{product.name}: {product.price}</li>)}</ul>
        <p>You can send another message to revise these details before preparing your workspace.</p>
        <button type="button" disabled={isProvisioning} onClick={handleProvisioning}>Approve &amp; Prepare Workspace</button>
      </section>}
      {/* Input Area */}
      <div className="p-4 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-transparent">
        {error && (
          <div id="instant-error" className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded-[8px] text-sm">
            {error}
          </div>
        )}
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
                className="text-xs font-medium min-h-[44px] px-4 py-2 flex items-center justify-center bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] hover:bg-[rgba(255,255,255,0.8)] dark:hover:bg-[rgba(22,22,26,0.9)] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] transition-colors border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]"
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
            className="glass-control w-full bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-[8px] py-3.5 pl-4 pr-12 min-h-[44px] text-[#1D1D1F] dark:text-[#F5F5F7] focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:opacity-50"
          />
          <button
            id="generate-storefront-btn"
            data-testid="generate-storefront-btn"
            type="submit"
            disabled={!input.trim() || isLoading || isProvisioning}
            className="absolute right-1 top-1.5 w-auto px-4 h-10 flex items-center justify-center bg-[#0066FF] hover:bg-[#005bb5] disabled:bg-gray-400 text-white rounded-[8px] transition-colors"
          >
            {error ? "Generate My Workspace" : (
              <svg className="w-4 h-4 translate-x-[1px]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
              </svg>
            )}
          </button>
        </form>
      </div>
    </div>
  );
}
