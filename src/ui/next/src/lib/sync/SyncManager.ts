import { useEffect } from 'react';
import { useSyncStore, OfflineMutation } from './syncStore';

export const SyncManager = () => {
  const { queue, isOnline, isSyncing, setOnlineStatus, setSyncingStatus, updateMutationStatus, removeMutation } = useSyncStore();

  useEffect(() => {
    const handleOnline = () => setOnlineStatus(true);
    const handleOffline = () => setOnlineStatus(false);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    // Initial check
    setOnlineStatus(navigator.onLine);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, [setOnlineStatus]);

  useEffect(() => {
    const processQueue = async () => {
      if (!isOnline || isSyncing || queue.length === 0) return;

      setSyncingStatus(true);

      const pendingMutations = queue.filter(m => m.status === 'pending' || m.status === 'failed');

      for (const mutation of pendingMutations) {
        try {
          updateMutationStatus(mutation.id, 'syncing');

          let endpoint = '';
          let method = 'POST';

          if (mutation.type === 'tap_to_pay') {
             endpoint = '/api/checkout/offline-sync';
          } else if (mutation.type === 'cash_order') {
             endpoint = '/api/checkout/offline-sync';
          }

          const response = await fetch(endpoint, {
            method,
            headers: {
              'Content-Type': 'application/json',
              'Idempotency-Key': mutation.idempotency_key
            },
            body: JSON.stringify(mutation.payload)
          });

          if (response.ok) {
            removeMutation(mutation.id);
          } else {
            // Handle specific errors (e.g., conflict) or mark as failed
            updateMutationStatus(mutation.id, 'failed');
          }
        } catch (error) {
          console.error(`Failed to sync mutation ${mutation.id}`, error);
          updateMutationStatus(mutation.id, 'failed');
        }
      }

      setSyncingStatus(false);
    };

    // Attempt to process queue when coming back online or periodically
    const intervalId = setInterval(() => {
       if(navigator.onLine) {
         processQueue();
       }
    }, 5000); // Check every 5 seconds

    // Also trigger on state change (isOnline becoming true)
    if (isOnline) {
       processQueue();
    }

    return () => clearInterval(intervalId);
  }, [queue, isOnline, isSyncing, setSyncingStatus, updateMutationStatus, removeMutation]);

  return null; // This is a logic-only component
};
