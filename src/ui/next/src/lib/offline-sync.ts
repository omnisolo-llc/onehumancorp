import { v4 as uuidv4 } from 'uuid';

export interface OfflineAction {
  id: string;
  action_type: string;
  payload: any;
  status: 'pending' | 'syncing' | 'completed' | 'failed';
  timestamp: number;
  idempotency_key: string;
}

const STORAGE_KEY = 'ohc_offline_actions_queue';

export class OfflineSyncManager {

  static getQueue(): OfflineAction[] {
    if (typeof window === 'undefined') return [];
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return [];
    try {
      return JSON.parse(stored);
    } catch (e) {
      console.error('Failed to parse offline queue', e);
      return [];
    }
  }

  static saveQueue(queue: OfflineAction[]) {
    if (typeof window === 'undefined') return;
    localStorage.setItem(STORAGE_KEY, JSON.stringify(queue));
  }

  static queueAction(action_type: string, payload: any): OfflineAction {
    const action: OfflineAction = {
      id: uuidv4(),
      action_type,
      payload,
      status: 'pending',
      timestamp: Date.now(),
      idempotency_key: uuidv4(),
    };

    const queue = this.getQueue();
    queue.push(action);
    this.saveQueue(queue);

    // Attempt sync immediately if online
    if (typeof navigator !== 'undefined' && navigator.onLine) {
        this.processQueue();
    }

    return action;
  }

  static async processQueue() {
    if (typeof window === 'undefined') return;

    const queue = this.getQueue();
    const pendingActions = queue.filter(a => a.status === 'pending' || a.status === 'failed');

    if (pendingActions.length === 0) return;

    for (const action of pendingActions) {
      try {
        // Mark as syncing to avoid duplicate attempts
        this.updateActionStatus(action.id, 'syncing');

        const response = await fetch('/api/offline-sync', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            idempotency_key: action.idempotency_key,
            action_type: action.action_type,
            payload: action.payload,
          }),
        });

        if (response.ok) {
           this.updateActionStatus(action.id, 'completed');
           // Optionally remove completed actions from queue
           this.removeAction(action.id);
        } else {
           console.error('Failed to sync action', action);
           this.updateActionStatus(action.id, 'failed');
        }
      } catch (e) {
        console.error('Network error during sync', e);
        this.updateActionStatus(action.id, 'failed');
      }
    }
  }

  static updateActionStatus(id: string, status: OfflineAction['status']) {
    const queue = this.getQueue();
    const index = queue.findIndex(a => a.id === id);
    if (index !== -1) {
      queue[index].status = status;
      this.saveQueue(queue);
    }

    // Dispatch event so UI can reactively update
    if (typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent('offline-sync-updated', { detail: { queue: this.getQueue() } }));
    }
  }

  static removeAction(id: string) {
      const queue = this.getQueue();
      const newQueue = queue.filter(a => a.id !== id);
      this.saveQueue(newQueue);
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent('offline-sync-updated', { detail: { queue: newQueue } }));
      }
  }

  static initNetworkListeners() {
      if (typeof window !== 'undefined') {
          window.addEventListener('online', () => {
              console.log('Network online, processing offline queue...');
              this.processQueue();
          });
      }
  }
}
