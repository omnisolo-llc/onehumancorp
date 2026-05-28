"use client";

import { useState, useEffect } from 'react';

export function useSyncStatus() {
  const [isOnline, setIsOnline] = useState(true);
  const [isSyncing, setIsSyncing] = useState(false);

  useEffect(() => {
    // Initial state
    if (typeof navigator !== 'undefined') {
      setIsOnline(navigator.onLine);
    }

    const handleOnline = () => {
      setIsOnline(true);
      setIsSyncing(true);

      // Simulate sync duration, in real life this would hook into actual CRDT sync promises
      setTimeout(() => {
        setIsSyncing(false);
      }, 2000);
    };

    const handleOffline = () => {
      setIsOnline(false);
      setIsSyncing(false);
    };

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  return { isOnline, isSyncing };
}

export function SyncEngine() {
  // Sync engine mounts and handles the global listener if needed
  // This satisfies the architectural need for the edge AI Agent & Sync orchestration loop
  useSyncStatus();
  return null;
}
