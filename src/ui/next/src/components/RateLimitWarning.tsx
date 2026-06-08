"use client";

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
          className="fixed bottom-6 right-6 bg-white/70 backdrop-blur-md border border-white/20 text-slate-800 px-6 py-4 rounded-2xl shadow-2xl z-[9999] flex items-start gap-4 max-w-sm w-full transition-all duration-300 ease-out"
          role="alert"
          aria-live="polite"
        >
          <span className="text-2xl mt-0.5">💡</span>
          <div className="flex-1">
            <h3 className="font-semibold text-sm text-slate-900 tracking-tight">Limit Reached</h3>
            <p className="text-sm mt-1 text-slate-600 leading-relaxed font-medium">{warningMessage}</p>
          </div>
          <button
            onClick={hideWarning}
            className="text-slate-400 hover:text-slate-700 transition-colors p-1.5 rounded-full hover:bg-black/5"
            aria-label="Close warning"
          >
            <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M1 1L11 11M1 11L11 1" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </button>
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