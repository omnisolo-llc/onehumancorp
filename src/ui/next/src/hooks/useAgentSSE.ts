'use client';

import { useEffect, useRef, useState, useCallback } from 'react';

interface SSEOptions {
  agentId: string;
  onMessage: (data: any) => void;
  onError?: (error: Event) => void;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
}

export function useAgentSSE({
  agentId,
  onMessage,
  onError,
  reconnectInterval = 3000,
  maxReconnectAttempts = 10,
}: SSEOptions) {
  const [connected, setConnected] = useState(false);
  const [reconnectAttempts, setReconnectAttempts] = useState(0);
  const eventSourceRef = useRef<EventSource | null>(null);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onMessageRef = useRef(onMessage);
  const onErrorRef = useRef(onError);

  onMessageRef.current = onMessage;
  onErrorRef.current = onError;

  const cleanup = useCallback(() => {
    if (reconnectTimerRef.current !== null) {
      clearTimeout(reconnectTimerRef.current);
      reconnectTimerRef.current = null;
    }
    if (eventSourceRef.current) {
      eventSourceRef.current.onopen = null;
      eventSourceRef.current.onmessage = null;
      eventSourceRef.current.onerror = null;
      eventSourceRef.current.close();
      eventSourceRef.current = null;
    }
  }, []);

  useEffect(() => {
    if (!agentId) return;

    const connect = () => {
      const es = new EventSource(`/api/v1/agents/${agentId}/events`);
      eventSourceRef.current = es;

      es.onopen = () => {
        setConnected(true);
        setReconnectAttempts(0);
      };

      es.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          onMessageRef.current(data);
        } catch {
          onMessageRef.current(event.data);
        }
      };

      es.onerror = (event) => {
        setConnected(false);
        onErrorRef.current?.(event);
        es.close();
        eventSourceRef.current = null;

        setReconnectAttempts((prev) => {
          const next = prev + 1;
          if (next > maxReconnectAttempts) return next;
          const delay = Math.min(next, 5) * reconnectInterval;
          reconnectTimerRef.current = setTimeout(connect, delay);
          return next;
        });
      };
    };

    connect();

    return cleanup;
  }, [agentId, reconnectInterval, maxReconnectAttempts, cleanup]);

  return { connected, reconnectAttempts };
}
