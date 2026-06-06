"use client";

/**
 * Robust SyncManager for Offline-First resilience.
 * Handles queuing mutations locally and syncing with the Go backend.
 */

export interface OfflineMutation {
  mutation_id: string;
  mutation_type: 'INVENTORY_DEDUCT' | 'TOGGLE_SOLD_OUT' | 'UPDATE_ORDER_STATUS';
  product_id?: string;
  quantity_deducted?: number;
  amount?: number;
  payment_method?: string;
  payment_intent_id?: string;
  currency?: string;
  order_id?: string;
  status?: string;
  timestamp: string;
  metadata?: any;
}

const STORAGE_KEY = 'ohc_offline_mutation_queue';

class SyncManager {
  private queue: OfflineMutation[] = [];
  private isSyncing = false;
  private listeners: ((status: { pendingCount: number; isSyncing: boolean }) => void)[] = [];

  constructor() {
    if (typeof window !== 'undefined') {
      this.loadQueue();
      this.setupNetworkListeners();
      this.startSyncLoop();
    }
  }

  private loadQueue() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      this.queue = stored ? JSON.parse(stored) : [];
      this.notify();
    } catch (e) {
      console.error('Failed to load sync queue', e);
      this.queue = [];
    }
  }

  private saveQueue() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(this.queue));
      this.notify();
    } catch (e) {
      console.error('Failed to save sync queue', e);
    }
  }

  private setupNetworkListeners() {
    window.addEventListener('online', () => this.sync());
  }

  private startSyncLoop() {
    // Attempt sync every 10 seconds if online
    setInterval(() => {
      if (this.queue.length > 0 && navigator.onLine && !this.isSyncing) {
        this.sync();
      }
    }, 10000);
  }

  public async enqueue(mutation: Omit<OfflineMutation, 'mutation_id' | 'timestamp'>) {
    const fullMutation: OfflineMutation = {
      ...mutation,
      mutation_id: crypto.randomUUID(),
      timestamp: new Date().toISOString(),
    };

    this.queue.push(fullMutation);
    this.saveQueue();

    if (navigator.onLine) {
      this.sync();
    }
  }

  public async sync() {
    if (this.isSyncing || this.queue.length === 0 || !navigator.onLine) return;

    this.isSyncing = true;
    this.notify();

    const mutationsToSync = [...this.queue];

    try {
      const response = await fetch('/api/v1/sync/offline', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'tenant-id': localStorage.getItem('tenant_id') || 'default',
        },
        body: JSON.stringify({ mutations: mutationsToSync }),
      });

      if (response.ok) {
        const result = await response.json();
        const processedIds = new Set(result.processed_ids || []);

        // Remove processed mutations from queue
        this.queue = this.queue.filter(m => !processedIds.has(m.mutation_id));
        this.saveQueue();
      }
    } catch (e) {
      console.error('Sync failed', e);
    } finally {
      this.isSyncing = false;
      this.notify();
    }
  }

  public subscribe(listener: (status: { pendingCount: number; isSyncing: boolean }) => void) {
    this.listeners.push(listener);
    listener({ pendingCount: this.queue.length, isSyncing: this.isSyncing });
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  private notify() {
    const status = { pendingCount: this.queue.length, isSyncing: this.isSyncing };
    this.listeners.forEach(l => l(status));
  }

  public getStatus() {
    return { pendingCount: this.queue.length, isSyncing: this.isSyncing };
  }
}

// Singleton instance
export const syncManager = new SyncManager();
