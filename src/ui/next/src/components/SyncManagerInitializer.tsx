"use client";

import { useEffect } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';
import { webSocketSyncClient } from '../lib/sync/WebSocketSyncClient';

export function SyncManagerInitializer() {
  useEffect(() => {
    // Ensure the SyncManager is instantiated on mount
    SyncManager.getInstance();

    // Connect the real-time WebSocket sync client
    webSocketSyncClient.connect();

    return () => {
      webSocketSyncClient.disconnect();
    };
  }, []);

  return null;
}
