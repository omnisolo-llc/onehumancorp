import { getActions, removeAction } from '../../app/utils/offlineQueue';

export class OfflineBackgroundWorker {
  private static instance: OfflineBackgroundWorker;
  private isSyncing = false;
  private retryCount = 0;
  private baseDelay = 1000;
  private maxRetries = 5;

  private constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.drainPendingActions());
    }
  }

  public static getInstance(): OfflineBackgroundWorker {
    if (!OfflineBackgroundWorker.instance) {
      OfflineBackgroundWorker.instance = new OfflineBackgroundWorker();
    }
    return OfflineBackgroundWorker.instance;
  }

  public async drainPendingActions(): Promise<void> {
    if (typeof window === 'undefined' || !navigator.onLine || this.isSyncing) {
      return;
    }

    this.isSyncing = true;

    try {
      while (true) {
        const actions = await getActions();
        if (actions.length === 0) {
          this.retryCount = 0; // reset on successful empty queue
          this.isSyncing = false;
          break;
        }

        const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
        const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

        const res = await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ mutations: actions })
        });

        if (!res.ok) {
          throw new Error(`Sync failed with status: ${res.status}`);
        }

        // Successful sync, remove all actions
        for (const action of actions) {
          await removeAction(action.id);
        }

        // Reset retry state
        this.retryCount = 0;

        if (typeof window !== 'undefined') {
          window.dispatchEvent(new Event('ohc_queue_updated'));
        }
      }
    } catch (error) {
      console.error('Background sync worker failed:', error);
      if (this.retryCount < this.maxRetries) {
        const backoffDelay = this.baseDelay * Math.pow(2, this.retryCount);
        this.retryCount++;
        setTimeout(() => {
          this.isSyncing = false;
          this.drainPendingActions();
        }, backoffDelay);
      } else {
        this.isSyncing = false;
      }
    }
  }
}
