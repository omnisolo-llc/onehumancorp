import { useEffect, useState } from 'react';

interface UseSyncGatewayOptions {
  topics: string[];
  enabled?: boolean;
}

export function useSyncGateway({ topics, enabled = true }: UseSyncGatewayOptions) {
  const [lastMessage, setLastMessage] = useState<any>(null);
  const [isConnected, setIsConnected] = useState(false);
  const topicsKey = topics.join(',');

  useEffect(() => {
    if (!enabled || typeof process.env.VITEST !== 'undefined' || process.env.NODE_ENV === 'test') return;

    // WebSocket upgrades cannot traverse the authenticated Next.js transport
    // without exposing browser-controlled identity. Native clients own streaming;
    // browser sync remains bounded HTTP through /api/v1 proxy routes.
    setIsConnected(false);
    return undefined;
  }, [enabled, topicsKey]);

  return { lastMessage, isConnected };
}
