import { useOfflineSyncStore } from './offlineSyncStore';

export class SyncManager {
  private static syncInterval: NodeJS.Timeout | null = null;
  private static MAX_RETRIES = 3;

  static start() {
    if (this.syncInterval) return;

    // Listen for online events to trigger immediate sync
    if (typeof window !== 'undefined') {
      window.addEventListener('online', this.sync);
    }

    // Periodic sync check
    this.syncInterval = setInterval(() => {
      this.sync();
    }, 15000); // Check every 15 seconds
  }

  static stop() {
    if (this.syncInterval) {
      clearInterval(this.syncInterval);
      this.syncInterval = null;
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('online', this.sync);
    }
  }

  static async sync() {
    const store = useOfflineSyncStore.getState();

    if (store.isSyncing || !navigator.onLine || store.queue.length === 0) {
      return;
    }

    store.setSyncing(true);

    const queue = [...store.queue]; // Copy queue

    for (const event of queue) {
      if (event.retryCount >= this.MAX_RETRIES) {
        // Drop it or move to dead letter queue. Here we just drop.
        store.dequeueMutation(event.id);
        continue;
      }

      try {
        const response = await fetch(event.endpoint, {
          method: event.method,
          headers: {
            'Content-Type': 'application/json',
            'X-Idempotency-Key': event.id, // Ensure idempotency
          },
          body: JSON.stringify({
            ...event.payload,
            _offlineId: event.id,
            _timestamp: event.timestamp
          }),
        });

        if (response.ok) {
          store.dequeueMutation(event.id);
        } else if (response.status >= 400 && response.status < 500 && response.status !== 429) {
          // Client error (e.g. bad request, unauthorized), won't succeed on retry
          // Should log this or alert user in a real app
          console.error("Unrecoverable error during sync:", await response.text());
          store.dequeueMutation(event.id);
        } else {
          // Server error or rate limit, retry later
          store.incrementRetry(event.id);
        }
      } catch (error) {
        console.error('Failed to sync event', event.id, error);
        store.incrementRetry(event.id);
        // If fetch throws, we are likely offline or network error, stop syncing current batch
        break;
      }
    }

    store.setSyncing(false);
  }
}
