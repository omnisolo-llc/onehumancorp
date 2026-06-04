"use client";
import { useEffect, useCallback, useRef } from 'react';
import { useSyncStore, QueuedMutation } from './syncStore';

const MAX_RETRIES = 3;

export const useSyncManager = () => {
  const {
    queue,
    isOnline,
    isSyncing,
    setOnlineStatus,
    setSyncingStatus,
    getPendingMutations,
    updateMutationStatus,
    removeMutation,
    incrementRetry
  } = useSyncStore();

  const syncIntervalRef = useRef<NodeJS.Timeout | null>(null);

  const handleOnline = useCallback(() => {
    setOnlineStatus(true);
    processQueue();
  }, [setOnlineStatus]);

  const handleOffline = useCallback(() => {
    setOnlineStatus(false);
  }, [setOnlineStatus]);

  // Set up network listeners
  useEffect(() => {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);

      // Initial check
      setOnlineStatus(navigator.onLine);

      if (navigator.onLine) {
        processQueue();
      }

      // Periodic check
      syncIntervalRef.current = setInterval(() => {
        if (navigator.onLine && !isSyncing) {
          processQueue();
        }
      }, 30000); // Check every 30 seconds
    }

    return () => {
      if (typeof window !== 'undefined') {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      }
      if (syncIntervalRef.current) {
        clearInterval(syncIntervalRef.current);
      }
    };
  }, []);

  const processQueue = async () => {
    if (!navigator.onLine || isSyncing) return;

    const pendingItems = getPendingMutations();
    if (pendingItems.length === 0) return;

    setSyncingStatus(true);

    for (const item of pendingItems) {
      if (item.retryCount >= MAX_RETRIES) {
        // Skip items that have failed too many times, maybe flag for manual intervention
        continue;
      }

      await syncItem(item);
    }

    setSyncingStatus(false);
  };

  const syncItem = async (item: QueuedMutation) => {
    updateMutationStatus(item.id, 'syncing');
    incrementRetry(item.id);

    try {
      // In a real app, you'd route this to the correct API endpoint based on item.type
      // For this implementation, we simulate an API call or use the endpoint if provided
      const endpoint = item.endpoint || '/api/sync';

      const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Idempotency-Key': item.idempotencyKey
        },
        body: JSON.stringify(item)
      });

      if (response.ok || response.status === 409) { // 409 Conflict might mean it was already processed
        // Success or already processed, remove from queue
        removeMutation(item.id);
      } else {
        // Failed but might be retryable
        updateMutationStatus(item.id, 'failed', `Status: ${response.status}`);
      }
    } catch (error) {
      console.error('Sync failed for item', item.id, error);
      updateMutationStatus(item.id, 'failed', error instanceof Error ? error.message : String(error));
    }
  };

  return {
    processQueue,
    forceSync: processQueue,
    pendingCount: getPendingMutations().length
  };
};

// Component to handle sync logic in the background
export const SyncProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  useSyncManager(); // Initialize the manager
  return children as React.ReactElement; // Or just return <>{children}</>
};
