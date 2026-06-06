"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

export function RateLimitBanner() {
  const [warningMessage, setWarningMessage] = useState<string | null>(null);
  const router = useRouter();

  useEffect(() => {
    // Intercept fetch responses to look for the x-ratelimit-warning header
    const originalFetch = window.fetch;
    window.fetch = async (...args) => {
      const response = await originalFetch(...args);
      const warning = response.headers.get('x-ratelimit-warning');
      if (warning) {
        setWarningMessage(warning);
      }
      return response;
    };

    return () => {
      window.fetch = originalFetch;
    };
  }, []);

  if (!warningMessage) return null;

  return (
    <div className="fixed bottom-4 right-4 z-50 max-w-md bg-white border-l-4 border-amber-500 shadow-xl rounded-lg p-4 animate-slide-up" role="alert">
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <h3 className="text-sm font-medium text-amber-800">Plan Limit Reached</h3>
          <p className="mt-1 text-sm text-gray-700">{warningMessage}</p>
        </div>
        <button
          onClick={() => setWarningMessage(null)}
          className="ml-4 text-gray-400 hover:text-gray-500 focus:outline-none"
        >
          <span className="sr-only">Close</span>
          <svg className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd" />
          </svg>
        </button>
      </div>
      <div className="mt-3">
        <button
          onClick={() => {
            setWarningMessage(null);
            router.push('/plan');
          }}
          className="text-sm font-medium text-amber-600 hover:text-amber-500"
        >
          Upgrade Plan &rarr;
        </button>
      </div>
    </div>
  );
}
