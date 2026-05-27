"use client";

import React, { useEffect, useState } from 'react';
import { GrowthMilestonePrompt } from './GrowthMilestonePrompt';

import { useRouter } from 'next/navigation';

export const FetchInterceptor: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [showPrompt, setShowPrompt] = useState(false);
  const [promptMessage, setPromptMessage] = useState("");
  const [pendingAction, setPendingAction] = useState<{ args: any[] } | null>(null);
  const router = useRouter();

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const originalFetch = window.fetch;
      window.fetch = async (...args) => {
        const response = await originalFetch(...args);

        // Clone the response so we can read it without locking the stream for the caller
        const clonedResponse = response.clone();

        if (response.status === 402) {
          try {
            const data = await clonedResponse.json();
            if (data && data.error === "LIMIT_EXCEEDED") {
              setPromptMessage(data.message);
              setPendingAction({ args });
              setShowPrompt(true);

              // We return a Promise that never resolves, so the application code
              // "waits" indefinitely instead of showing a red error box.
              // When the user pays, we resume.
              return new Promise(() => {});
            }
          } catch (e) {
            // Ignore JSON parsing errors for other 402s
          }
        }

        return response;
      };

      return () => {
        window.fetch = originalFetch;
      };
    }
  }, []);

  const handleUpgrade = () => {
    setShowPrompt(false);

    if (pendingAction) {
        try {
            sessionStorage.setItem('pending_action', JSON.stringify({
                url: typeof pendingAction.args[0] === 'string' ? pendingAction.args[0] : pendingAction.args[0].url,
                options: pendingAction.args[1] || {}
            }));
        } catch (e) {}
    }

    router.push('/checkout?tier=Starter');
  };

  return (
    <>
      {children}
      {showPrompt && (
        <GrowthMilestonePrompt
          message={promptMessage}
          onClose={() => {
              setShowPrompt(false);
              setPendingAction(null);
              // Trigger a page reload or generic error to break the pending state if they cancel
              window.location.reload();
          }}
          onUpgrade={handleUpgrade}
        />
      )}
    </>
  );
};
