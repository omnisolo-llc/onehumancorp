import { useEffect, useState, useCallback, useRef } from 'react';

interface UseSyncGatewayOptions {
  topics: string[];
  enabled?: boolean;
}

export function useSyncGateway({ topics, enabled = true }: UseSyncGatewayOptions) {
  const [lastMessage, setLastMessage] = useState<any>(null);
  const [isConnected, setIsConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = useCallback(() => {
    if (!enabled) return;

    if (wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const topicsQuery = topics.join(',');
    const wsUrl = `${protocol}//${window.location.host}/api/v1/sync/ws?topics=${topicsQuery}`;

    if (typeof process.env.VITEST !== 'undefined' || process.env.NODE_ENV === 'test') return;

    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onopen = () => {
      setIsConnected(true);
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        setLastMessage(data);
      } catch (e) {
        console.error('Failed to parse sync message', e);
      }
    };

    ws.onclose = () => {
      setIsConnected(false);
      // Implement exponential backoff reconnection here if needed
      setTimeout(connect, 3000);
    };

    ws.onerror = (error) => {
      console.error('Sync WebSocket error', error);
      ws.close();
    }
  }, [topics, enabled]);

  useEffect(() => {
    connect();
    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [connect]);

  return { lastMessage, isConnected };
}
