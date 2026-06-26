import React, { useState, useEffect } from 'react';

interface OnboardingChatAgentProps {
  onComplete: (data: any) => void;
}

export function OnboardingChatAgent({ onComplete }: OnboardingChatAgentProps) {
  const [input, setInput] = useState('');
  const [isProvisioning, setIsProvisioning] = useState(false);
  const [loadingText, setLoadingText] = useState('Setting up your catalog...');

  const predefinedChips = [
    "I'm a dog walker in Seattle",
    "Custom vegan cakes in Austin",
    "Mobile car detailing service",
    "Boutique yoga studio"
  ];

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isProvisioning) {
      const messages = [
        "Setting up your catalog...",
        "Applying a clean, modern design...",
        "Configuring booking and payment flows...",
        "Finalizing your storefront..."
      ];
      let i = 0;
      interval = setInterval(() => {
        i = (i + 1) % messages.length;
        setLoadingText(messages[i]);
      }, 3000);
    }
    return () => clearInterval(interval);
  }, [isProvisioning]);

  const handleSend = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!input.trim() || isProvisioning) return;

    setIsProvisioning(true);

    try {
      const response = await fetch('/api/v1/onboarding/zero-click-intake', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: input.trim(), image_url: null }),
      });

      if (!response.ok) throw new Error('Failed to generate business');

      const data = await response.json();

      setTimeout(() => {
        onComplete(data);
        setIsProvisioning(false);
      }, 1000);
    } catch (error) {
      console.error(error);
      setIsProvisioning(false);
      alert('Failed to generate your business. Please try again.');
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden flex flex-col relative w-full mb-8">

      {/* Provisioning Overlay with Glassmorphism */}
      {isProvisioning && (
        <div className="absolute inset-0 z-10 bg-white/60 dark:bg-black/60 backdrop-blur-md flex flex-col items-center justify-center rounded-2xl">
          <div className="w-16 h-16 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-6 shadow-lg"></div>
          <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-2 animate-pulse">
            Building Your Business...
          </h3>
          <p className="text-sm text-gray-700 dark:text-gray-300 font-medium transition-all duration-500">
            {loadingText}
          </p>
        </div>
      )}

      {/* Single Prompt Input Area */}
      <div className="p-6">
        <h2 className="text-xl font-semibold mb-6 text-gray-900 dark:text-white text-center">Tell me about your business...</h2>

        <div className="flex flex-wrap justify-center gap-2 mb-6">
          {predefinedChips.map((chip, idx) => (
            <button
              key={idx}
              onClick={() => {
                setInput(chip);
              }}
              className="text-sm font-medium px-4 py-2 bg-gray-50 dark:bg-gray-800/50 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-full text-gray-700 dark:text-gray-300 transition-colors border border-gray-200 dark:border-gray-700"
            >
              {chip}
            </button>
          ))}
        </div>

        <form onSubmit={handleSend} className="relative flex flex-col gap-4">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            disabled={isProvisioning}
            placeholder="E.g., I'm a dog walker in Seattle..."
            className="w-full bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-xl p-4 min-h-[120px] text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-indigo-500 disabled:opacity-50 resize-none shadow-inner"
          />
          <div className="flex justify-between items-center">
            <button
              type="button"
              className="flex items-center gap-2 text-sm text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
              disabled={isProvisioning}
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
              </svg>
              Add Photo (Optional)
            </button>
            <button
              type="submit"
              disabled={!input.trim() || isProvisioning}
              className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 text-white px-6 py-3 rounded-xl font-bold transition-all shadow-sm active:scale-[0.98]"
            >
              Generate Store
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path>
              </svg>
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
