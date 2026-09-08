'use client';

import { useEffect, useRef, useState, useCallback } from 'react';

interface UseAgentWebSocketOptions {
  url: string;
  onMessage: (data: any) => void;
  reconnectInterval?: number;
}

export function useAgentWebSocket({
  url,
  onMessage,
  reconnectInterval = 3000,
}: UseAgentWebSocketOptions) {
  const [connected, setConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const onMessageRef = useRef(onMessage);
  const mountedRef = useRef(true);

  onMessageRef.current = onMessage;

  const cleanup = useCallback(() => {
    mountedRef.current = false;
    if (reconnectTimerRef.current !== null) {
      clearTimeout(reconnectTimerRef.current);
      reconnectTimerRef.current = null;
    }
    if (wsRef.current) {
      wsRef.current.onclose = null;
      wsRef.current.close();
      wsRef.current = null;
    }
  }, []);

  useEffect(() => {
    if (!url) return;
    mountedRef.current = true;
    let disposed = false;
    // Preserve received frame order across reconnects within this subscription.
    let pending = Promise.resolve();

    const connect = () => {
      if (disposed || !mountedRef.current) return;

      const ws = typeof DecompressionStream === 'function'
        ? new WebSocket(url, ['omnisolo.gzip.v1'])
        : new WebSocket(url);
      ws.binaryType = 'arraybuffer';
      wsRef.current = ws;

      ws.onopen = () => {
        if (disposed || wsRef.current !== ws) return;
        setConnected(true);
      };

      ws.onmessage = (event) => {
        pending = pending.then(async () => {
          let text: string;
          if (typeof event.data === 'string') {
            text = event.data;
          } else if (event.data instanceof ArrayBuffer) {
            const stream = new Blob([event.data]).stream()
              .pipeThrough(new DecompressionStream('gzip'));
            text = await new Response(stream).text();
          } else {
            throw new Error('Unsupported WebSocket frame');
          }
          if (disposed) return;
          const data = JSON.parse(text);
          if (data.type === 'batch' && Array.isArray(data.items)) {
            for (const item of data.items) onMessageRef.current(item);
          } else {
            onMessageRef.current(data);
          }
        }).catch((err) => {
          console.error('Failed to parse WebSocket message:', err);
        });
      };

      ws.onclose = () => {
        if (disposed || wsRef.current !== ws) return;
        setConnected(false);
        wsRef.current = null;
        if (mountedRef.current) {
          reconnectTimerRef.current = setTimeout(connect, reconnectInterval);
        }
      };

      ws.onerror = () => {
        ws.close();
      };
    };

    connect();

    return () => { disposed = true; cleanup(); };
  }, [url, reconnectInterval, cleanup]);

  return { connected };
}
