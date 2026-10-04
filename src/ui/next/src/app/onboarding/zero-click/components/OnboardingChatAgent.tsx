import React, { useState, useRef, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { sameOwner } from '@/lib/sync/queueIdentity';
import { initializeOnboardingDraft, onboardingStorageFailed, useOnboardingStore } from '../../store';
import { openOnboardingSession, onboardingOwner, fetchForOnboardingOwner, subscribeOnboardingInvalidation, type DraftOwner } from '../../draftSession';
import { normalizeReviewedProducts, readPreparation, readPreparedResult, resultForPreparation } from '../../contracts';

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
  initial_products: { name: string; price: string | number; description?: string | null; variants?: { name: string; price_modifier: string | number }[] | null }[];
}

interface OnboardingChatAgentProps {
  onComplete: (data: import('@/lib/builder-types').OnboardingResult) => void;
}

export function OnboardingChatAgent({ onComplete }: OnboardingChatAgentProps) {
  const router = useRouter();
  const [manualPrompt, setManualPrompt] = useState<string | null>(null);
  const [viewOwner, setViewOwner] = useState<DraftOwner | null>(null);
  const [identityError, setIdentityError] = useState('');
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
    let loadVersion = 0;
    const load = async () => {
      const version = ++loadVersion;
      const active = () => !cancelled && version === loadVersion;
      setIsLoaded(false); setIdentityError('');
      try {
        const owner = await openOnboardingSession();
        if (!active()) return;
        setViewOwner(owner);
        const response = await fetchForOnboardingOwner('/api/v1/onboarding/state', {}, owner);
        if (!response.ok) throw new Error('Failed to fetch state');
        const data = await response.json();
        if (!active()) return;
        if (data?.preparation) { onComplete(resultForPreparation(readPreparation(data.preparation))); return; }
        if (Array.isArray(data?.chatMessages) && data.chatMessages.length > 0) setMessages(data.chatMessages);
        setIsLoaded(true);
      } catch (cause) {
        if (active()) { setViewOwner(null); setIdentityError(cause instanceof Error ? cause.message : 'Verify your session before using setup.'); }
      }
    };
    const unsubscribe = subscribeOnboardingInvalidation(restart => {
      loadVersion += 1;
      epoch.current += 1; busy.current = false; unknownPreparation.current = false;
      setInput(''); setReview(null); setManualPrompt(null); setMessages([{role:'assistant',content:'Sign in to continue your setup.'}]); setIsLoading(false); setIsProvisioning(false); setViewOwner(null);
      if (restart) void load(); else { setIsLoaded(false); setIdentityError('Your session could not be verified. Sign in again to continue.'); }
    });
    void load();
    return () => { cancelled = true; loadVersion += 1; unsubscribe(); };
  }, []);

  useEffect(() => {
    if (isLoaded) {
      scrollToBottom();
    }
  }, [messages, isLoading, isProvisioning, isLoaded]);

  const saveStateToBackend = async (chatMessagesToSave: ChatMessage[]) => {
    try {
      await fetchForOnboardingOwner('/api/v1/onboarding/state', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ step: -2, chatMessages: chatMessagesToSave }),
      }, viewOwner);
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
    setMessages(newMessages); setInput(''); setIsLoading(true); setReview(null); setManualPrompt(null); setError(null);
    void saveStateToBackend(newMessages);
    try {
      const response = await fetchForOnboardingOwner('/api/v1/onboarding/chat', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ messages: newMessages }) }, viewOwner);
      const data = await response.json();
      if (!active()) return;
      if (response.status === 503 && data?.error === 'onboarding_ai_unconfigured') {
        setManualPrompt(newMessages.filter(message => message.role === 'user').map(message => message.content).join(' '));
        setError('AI-assisted setup is unavailable because no model provider is configured. You can review and enter your details manually.');
        return;
      }
      if (!response.ok) throw new Error('Failed to communicate with setup agent');
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

  const handleManualReview = async () => {
    if (manualPrompt === null || !viewOwner || busy.current) return;
    busy.current = true; setIsLoading(true);
    const version = ++epoch.current; const intended = { ...viewOwner };
    const active = () => version === epoch.current;
    try {
      if ([...manualPrompt].length > 4000) throw new Error('Your setup description exceeds 4000 characters. Keep a shorter description before continuing manually.');
      const owner = await initializeOnboardingDraft();
      if (!active()) return;
      const currentOwner = onboardingOwner();
      if (!currentOwner || !sameOwner(owner, intended) || !sameOwner(currentOwner, intended)) throw new Error('Your session changed. Reopen setup before continuing.');
      const previous = useOnboardingStore.getState();
      previous.updateState({ step: 2, bio: manualPrompt, businessDescription: previous.businessDescription || previous.whatYouSell || manualPrompt, isLoading: false, error: '' });
      if (onboardingStorageFailed()) throw new Error('Your manual setup details could not be saved on this device. Keep this conversation open and preserve your input before retrying.');
      if (active()) router.push('/onboarding');
    } catch (cause) { if (active()) setError(cause instanceof Error ? cause.message : 'Manual setup could not be opened.'); }
    finally { if (active()) { busy.current = false; setIsLoading(false); } }
  };

  const handleProvisioning = async () => {
    if (!review || busy.current) return;
    busy.current = true;
    const version = ++epoch.current;
    const active = () => version === epoch.current;
    setIsProvisioning(true); setError(null);
    try {
      if (unknownPreparation.current) {
        const response = await fetchForOnboardingOwner('/api/v1/onboarding/state', {}, viewOwner);
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
        initial_products: normalizeReviewedProducts(intake.initial_products), ai_agents: [], ai_auto_respond: false,
      };
      const response = await fetchForOnboardingOwner('/api/v1/onboarding/start', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) }, viewOwner);
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

  if (identityError) return <div role="alert">{identityError} Your other session’s draft remains held.</div>;
  if (!isLoaded || !viewOwner) {
    return (
      <div role="status" aria-label="Restoring setup" aria-busy="true" className="flex items-center justify-center min-h-[50vh] w-full max-w-2xl mx-auto">
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
      {manualPrompt !== null && <section aria-label="Manual setup available" className="p-4"><p>Your conversation is preserved above. Review this description, then enter your own business, product and price details.</p><label htmlFor="manual-description">Description for manual setup</label><textarea id="manual-description" value={manualPrompt} disabled={isLoading || isProvisioning} onChange={event => setManualPrompt(event.target.value)} /><button type="button" disabled={isLoading || isProvisioning} onClick={() => void handleManualReview()}>Review Details Manually</button></section>}
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
            aria-label="Send message"
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
