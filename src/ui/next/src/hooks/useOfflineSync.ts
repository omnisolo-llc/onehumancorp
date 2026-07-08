import { useState, useEffect } from 'react';
import { OfflineSyncManager, OfflineAction } from '@/lib/offline-sync';

export function useOfflineSync() {
  const [queue, setQueue] = useState<OfflineAction[]>([]);
  const [isOnline, setIsOnline] = useState(true);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsOnline(navigator.onLine);
      setQueue(OfflineSyncManager.getQueue());

      const handleOnline = () => setIsOnline(true);
      const handleOffline = () => setIsOnline(false);

      const handleSyncUpdate = (e: any) => {
          setQueue(e.detail.queue);
      };

      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      window.addEventListener('offline-sync-updated', handleSyncUpdate);

      OfflineSyncManager.initNetworkListeners();

      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
        window.removeEventListener('offline-sync-updated', handleSyncUpdate);
      };
    }
  }, []);

  const dispatchAction = (action_type: string, payload: any) => {
    OfflineSyncManager.queueAction(action_type, payload);
  };

  return { queue, isOnline, dispatchAction };
}
