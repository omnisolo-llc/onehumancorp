import { useState, useEffect, useCallback } from 'react';

type SyncMutation = {
  id: string;
  type: string;
  payload: any;
  timestamp: string;
  sync_status: 'PENDING' | 'SYNCED' | 'FAILED';
};

export function useSyncManager(storageKey: string, syncEndpoint: string) {
  const [isOffline, setIsOffline] = useState(false);
  const [syncing, setSyncing] = useState(false);

  // Load pending from local storage
  const getQueue = useCallback(() => {
    try {
      const q = localStorage.getItem(storageKey);
      return q ? JSON.parse(q) : [];
    } catch {
      return [];
    }
  }, [storageKey]);

  const saveQueue = useCallback((queue: SyncMutation[]) => {
    localStorage.setItem(storageKey, JSON.stringify(queue));
  }, [storageKey]);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);

    if (typeof window !== 'undefined') {
      setIsOffline(!navigator.onLine);
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
    }

    return () => {
      if (typeof window !== 'undefined') {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      }
    };
  }, []);

  const queueMutation = useCallback((type: string, payload: any) => {
    const mutation: SyncMutation = {
      id: `mut_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`,
      type,
      payload,
      timestamp: new Date().toISOString(),
      sync_status: 'PENDING'
    };

    const queue = getQueue();
    queue.push(mutation);
    saveQueue(queue);

    return mutation;
  }, [getQueue, saveQueue]);

  useEffect(() => {
    const flushQueue = async () => {
      if (typeof window === 'undefined' || !navigator.onLine || syncing) return;

      const queue = getQueue();
      if (queue.length === 0) return;

      setSyncing(true);
      try {
        const response = await fetch(syncEndpoint, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ mutations: queue })
        });

        if (response.ok) {
           saveQueue([]);
        }
      } catch (e) {
        console.error("Sync flush failed", e);
      } finally {
        setSyncing(false);
      }
    };

    const intervalId = setInterval(flushQueue, 5000);
    return () => clearInterval(intervalId);
  }, [getQueue, saveQueue, syncEndpoint, syncing]);

  return { queueMutation, isOffline, syncing };
}
