import { useEffect, useCallback, useState } from 'react';
import { offlineSyncService, OfflineMutation } from '@/lib/offline-sync';

const SYNC_INTERVAL = 10000; // 10 seconds

export function useOfflineSyncEngine(apiEndpoint = '/api/v1/sync/mutations') {
  const [isOnline, setIsOnline] = useState<boolean>(true);
  const [pendingCount, setPendingCount] = useState(0);

  const checkPending = useCallback(async () => {
    try {
      const pending = await offlineSyncService.getPendingMutations();
      setPendingCount(pending.length);
    } catch (e) {
      console.error("Failed to check pending mutations", e);
    }
  }, []);

  const sync = useCallback(async () => {
    if (!isOnline) return;

    const mutations = await offlineSyncService.getPendingMutations();
    if (mutations.length === 0) return;

    for (const m of mutations) {
      await offlineSyncService.markAsSyncing(m.idempotency_key);
    }

    try {
      const res = await fetch(apiEndpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ mutations }),
      });

      if (res.ok) {
        // Assume success, clear mutations
        for (const m of mutations) {
          await offlineSyncService.removeMutation(m.idempotency_key);
        }
      } else {
        // Mark as failed
        for (const m of mutations) {
          await offlineSyncService.markAsFailed(m.idempotency_key);
        }
      }
    } catch (error) {
      // Network error, mark as failed to retry
      for (const m of mutations) {
        await offlineSyncService.markAsFailed(m.idempotency_key);
      }
    }

    await checkPending();
  }, [isOnline, apiEndpoint, checkPending]);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsOnline(navigator.onLine);

      const handleOnline = () => {
        setIsOnline(true);
        sync();
      };
      const handleOffline = () => setIsOnline(false);

      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);

      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      };
    }
  }, [sync]);

  useEffect(() => {
    const timer = setInterval(() => {
      sync();
      checkPending();
    }, SYNC_INTERVAL);

    // Initial check
    checkPending();

    return () => clearInterval(timer);
  }, [sync, checkPending]);

  const addOfflineMutation = useCallback(async (mutation: Omit<OfflineMutation, 'status' | 'created_at' | 'idempotency_key'>) => {
    const entry = await offlineSyncService.addMutation(mutation);
    await checkPending();
    // Try to sync immediately if online
    if (isOnline) {
      sync();
    }
    return entry;
  }, [isOnline, sync, checkPending]);

  return { isOnline, pendingCount, sync, addOfflineMutation };
}
