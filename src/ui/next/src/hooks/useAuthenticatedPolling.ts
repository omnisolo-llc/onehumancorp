'use client';

import { useEffect, useRef } from 'react';

type PollCallback = () => void | Promise<void>;

type UseAuthenticatedPollingOptions = {
  onPoll: PollCallback;
  intervalMs?: number;
  enabled?: boolean;
};

export function useAuthenticatedPolling({
  onPoll,
  intervalMs = 5_000,
  enabled = true,
}: UseAuthenticatedPollingOptions) {
  const onPollRef = useRef(onPoll);
  onPollRef.current = onPoll;

  useEffect(() => {
    if (!enabled) return;

    let active = true;
    const timer = window.setInterval(() => {
      if (!active) return;
      void Promise.resolve(onPollRef.current()).catch((error) => {
        if (active) {
          console.error('Authenticated feed polling failed:', error);
        }
      });
    }, intervalMs);

    return () => {
      active = false;
      window.clearInterval(timer);
    };
  }, [enabled, intervalMs]);
}
