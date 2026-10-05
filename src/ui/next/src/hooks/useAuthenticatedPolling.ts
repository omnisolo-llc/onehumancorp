'use client';

import { useEffect, useRef } from 'react';

type PollCallback = (signal: AbortSignal) => void | Promise<void>;

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
  const inFlight = useRef<AbortController | null>(null);
  onPollRef.current = onPoll;

  useEffect(() => {
    if (!enabled) return;

    let active = true;
    const poll = async () => {
      if (!active || inFlight.current) return;
      const controller = new AbortController();
      inFlight.current = controller;
      try {
        await onPollRef.current(controller.signal);
      } catch (error) {
        if (active && !controller.signal.aborted) {
          console.error('Authenticated feed polling failed:', error);
        }
      } finally {
        if (inFlight.current === controller) inFlight.current = null;
      }
    };
    const timer = window.setInterval(() => { void poll(); }, intervalMs);

    return () => {
      active = false;
      window.clearInterval(timer);
      inFlight.current?.abort();
    };
  }, [enabled, intervalMs]);
}
