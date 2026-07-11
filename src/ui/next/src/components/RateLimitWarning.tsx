"use client";


import { WithTooltip } from "./TooltipRegistry";

import React, { createContext, useContext, useState, useEffect, ReactNode, useCallback } from 'react';

interface RateLimitWarningContextType {
  warningMessage: string | null;
  showWarning: (msg: string) => void;
  hideWarning: () => void;
}

const RateLimitWarningContext = createContext<RateLimitWarningContextType | undefined>(undefined);

// Set up the global fetch interceptor immediately so it runs before any React rendering
let interceptorInstalled = false;
let globalShowWarning: ((msg: string) => void) | null = null;

if (typeof window !== 'undefined' && !interceptorInstalled) {
  const originalFetch = window.fetch;

  window.fetch = async (...args) => {
    const response = await originalFetch(...args);

    const warning = response.headers.get('x-ratelimit-warning');
    if (warning && globalShowWarning) {
      globalShowWarning(warning);
    }

    return response;
  };
  interceptorInstalled = true;
}

export function RateLimitWarningProvider({ children }: { children: ReactNode }) {
  const [warningMessage, setWarningMessage] = useState<string | null>(null);

  const showWarning = useCallback((msg: string) => {
    setWarningMessage(msg);
  }, []);

  const hideWarning = useCallback(() => {
    setWarningMessage(null);
  }, []);

  useEffect(() => {
    // Bind the global function to this instance's showWarning
    globalShowWarning = showWarning;

    return () => {
      if (globalShowWarning === showWarning) {
        globalShowWarning = null;
      }
    };
  }, [showWarning]);

  return (
    <RateLimitWarningContext.Provider value={{ warningMessage, showWarning, hideWarning }}>
      {children}
      {warningMessage && (
        <div
          className="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] backdrop-saturate-[210%] border border-white/40 dark:border-white/10 text-amber-800 dark:text-amber-100 px-6 py-4 rounded-xl shadow-lg z-[9999] flex items-start gap-3 max-w-md w-full"
          role="alert"
          aria-live="polite"
        >
          <span className="text-xl">💡</span>
          <div className="flex-1">
            <h3 className="font-semibold text-sm">Limit Reached</h3>
            <p className="text-sm mt-1 leading-relaxed">{warningMessage}</p>
          </div>
          <WithTooltip id="rate-limit-close-tooltip" defaultText="Dismiss this warning.">
            <button
              onClick={hideWarning}
              className="text-amber-500 hover:text-amber-700 transition-colors p-1"
              aria-label="Close warning"
            >
              ✕
            </button>
          </WithTooltip>
        </div>
      )}
    </RateLimitWarningContext.Provider>
  );
}

export function useRateLimitWarning() {
  const context = useContext(RateLimitWarningContext);
  if (context === undefined) {
    throw new Error('useRateLimitWarning must be used within a RateLimitWarningProvider');
  }
  return context;
}
