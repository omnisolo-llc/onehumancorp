type QueuedAction = {
  id: string;
  endpoint: string;
  method: string;
  headers: Record<string, string>;
  body: string;
  timestamp: number;
};

class OfflineQueueManager {
  private queueKey = 'ohc_offline_action_queue';
  private syncInProgress = false;
  private listeners: (() => void)[] = [];

  constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.syncQueue());
    }
  }

  public subscribe(listener: () => void) {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  private notify() {
    this.listeners.forEach(l => l());
  }

  public getQueueLength(): number {
    if (typeof window === 'undefined') return 0;
    try {
      const queue = JSON.parse(localStorage.getItem(this.queueKey) || '[]');
      return Array.isArray(queue) ? queue.length : 0;
    } catch {
      return 0;
    }
  }

  public enqueueAction(action: Omit<QueuedAction, 'timestamp'>) {
    if (typeof window === 'undefined') return;

    try {
      const currentQueue: QueuedAction[] = JSON.parse(localStorage.getItem(this.queueKey) || '[]');
      // Avoid duplicate actions for the same ID/endpoint combination
      const filteredQueue = currentQueue.filter(a => !(a.id === action.id && a.endpoint === action.endpoint));

      const queuedAction: QueuedAction = { ...action, timestamp: Date.now() };
      filteredQueue.push(queuedAction);
      localStorage.setItem(this.queueKey, JSON.stringify(filteredQueue));
      this.notify();

      if (navigator.onLine) {
        this.syncQueue();
      }
    } catch (error) {
      console.error('Failed to enqueue offline action:', error);
    }
  }

  public async syncQueue() {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    this.syncInProgress = true;
    try {
      const queue: QueuedAction[] = JSON.parse(localStorage.getItem(this.queueKey) || '[]');
      if (!Array.isArray(queue) || queue.length === 0) return;

      const newQueue = [...queue];
      for (const action of queue) {
        if (!navigator.onLine) break;

        try {
          const res = await fetch(action.endpoint, {
            method: action.method,
            headers: action.headers,
            body: action.body,
          });

          if (res.ok) {
            // Remove from queue upon success
            const idx = newQueue.findIndex(a => a.id === action.id && a.endpoint === action.endpoint);
            if (idx !== -1) newQueue.splice(idx, 1);
            localStorage.setItem(this.queueKey, JSON.stringify(newQueue));
            this.notify();
          } else {
             console.error(`Failed to sync action ${action.id}, status: ${res.status}`);
             // If bad request (4xx), might be unrecoverable, we could remove it, but for now we'll let it retry later
             if (res.status >= 400 && res.status < 500 && res.status !== 429) {
                 const idx = newQueue.findIndex(a => a.id === action.id && a.endpoint === action.endpoint);
                 if (idx !== -1) newQueue.splice(idx, 1);
                 localStorage.setItem(this.queueKey, JSON.stringify(newQueue));
                 this.notify();
             }
          }
        } catch (err) {
          console.error(`Network error syncing action ${action.id}:`, err);
        }
      }
    } catch (error) {
      console.error('Error during offline queue sync:', error);
    } finally {
      this.syncInProgress = false;
    }
  }
}

export const offlineQueueManager = new OfflineQueueManager();
