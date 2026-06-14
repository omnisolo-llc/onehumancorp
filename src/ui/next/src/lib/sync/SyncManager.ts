export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed', 'tap_to_pay', 'inventory_toggle'
  payload: any;
  timestamp: number;
  // Extra fields that were previously flat
  amount?: number;
  currency?: string;
  product_id?: string;
  quantity?: number;
  notes?: string;
  client_id?: string;
}

const DB_NAME = "OHC_Offline_Queue";
const STORE_NAME = "actions";
const DB_VERSION = 1;

function getDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = window.indexedDB.open(DB_NAME, DB_VERSION);

    request.onerror = (event) => {
      if (process.env.NODE_ENV !== 'test') {
        console.error("IndexedDB error", event);
      }
      reject(request.error);
    };

    request.onsuccess = (event) => {
      resolve((event.target as IDBOpenDBRequest).result);
    };

    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(STORE_NAME)) {
        db.createObjectStore(STORE_NAME, { keyPath: "id" });
      }
    };
  });
}

export class SyncManager {
  private static instance: SyncManager;
  private syncInProgress = false;
  private retryDelayMs = 1000;
  private maxRetries = 5;

  private constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.sync());
      (window as any).__getOfflineQueue = async () => {
        return await this.getQueue();
      };
      (window as any).__clearOfflineQueue = async () => {
        const db = await getDB();
        return new Promise<void>((resolve, reject) => {
          const transaction = db.transaction([STORE_NAME], "readwrite");
          const store = transaction.objectStore(STORE_NAME);
          const request = store.clear();
          request.onsuccess = () => resolve();
          request.onerror = () => reject(request.error);
        });
      };
      (window as any).__enqueueOfflineAction = async (action: any) => {
        return await this.enqueue(action);
      };
    }
  }

  public static getInstance(): SyncManager {
    if (!SyncManager.instance) {
      SyncManager.instance = new SyncManager();
    }
    return SyncManager.instance;
  }

  public async enqueue(mutation: OfflineAction): Promise<void> {
    if (typeof window === 'undefined' || !window.indexedDB) return;
    try {
      const db = await getDB();
      await new Promise<void>((resolve, reject) => {
        const transaction = db.transaction([STORE_NAME], "readwrite");
        const store = transaction.objectStore(STORE_NAME);
        const request = store.put(mutation);

        request.onsuccess = () => resolve();
        request.onerror = () => reject(request.error);
      });
      this.notifyListeners();

      if (navigator.onLine) {
        this.sync();
      }
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to enqueue action", err);
      }
    }
  }

  public async getQueueLength(): Promise<number> {
    const queue = await this.getQueue();
    return queue.length;
  }

  public async getQueue(): Promise<OfflineAction[]> {
    if (typeof window === 'undefined' || !window.indexedDB) return [];
    try {
      const db = await getDB();
      return new Promise((resolve, reject) => {
        const transaction = db.transaction([STORE_NAME], "readonly");
        const store = transaction.objectStore(STORE_NAME);
        const request = store.getAll();

        request.onsuccess = () => {
          resolve(request.result as OfflineAction[]);
        };
        request.onerror = () => reject(request.error);
      });
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to get actions", err);
      }
      return [];
    }
  }

  public async removeAction(id: string): Promise<void> {
    if (typeof window === "undefined" || !window.indexedDB) return;
    try {
      const db = await getDB();
      await new Promise<void>((resolve, reject) => {
        const transaction = db.transaction([STORE_NAME], "readwrite");
        const store = transaction.objectStore(STORE_NAME);
        const request = store.delete(id);

        request.onsuccess = () => resolve();
        request.onerror = () => reject(request.error);
      });
      this.notifyListeners();
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to remove action", err);
      }
    }
  }

  public async removeActions(ids: string[]): Promise<void> {
      if (typeof window === "undefined" || !window.indexedDB) return;
      try {
        const db = await getDB();
        await new Promise<void>((resolve, reject) => {
          const transaction = db.transaction([STORE_NAME], "readwrite");
          const store = transaction.objectStore(STORE_NAME);
          ids.forEach(id => store.delete(id));
          transaction.oncomplete = () => resolve();
          transaction.onerror = () => reject(transaction.error);
        });
        this.notifyListeners();
      } catch (err) {
        if (process.env.NODE_ENV !== 'test') {
          console.error("Failed to remove actions", err);
        }
      }
  }

  private notifyListeners() {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('ohc_queue_updated'));
      window.dispatchEvent(new Event('storage')); // trigger fallback storage listeners
    }
  }

  public async sync(retryCount = 0): Promise<void> {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = await this.getQueue();
    if (queue.length === 0) return;

    this.syncInProgress = true;

    try {
      // Separate POS transactions from general offline sync
      const posTransactions = queue.filter(m => m.type === 'tap_to_pay').map(m => {
        return {
          id: m.id,
          client_id: m.client_id || 'terminal_client',
          amount_cents: m.amount ? Math.round(m.amount) : 0,
          currency: m.currency || 'usd',
          payload: m.payload || JSON.stringify([{ product_id: m.product_id, quantity: m.quantity || 1 }]),
          timestamp: new Date(m.timestamp).toISOString()
        };
      });

      const generalMutations = queue.filter(m => m.type !== 'tap_to_pay' && m.type !== 'approve_agent_feed').map(m => {
        if (m.type === 'inventory_toggle') {
           return {
              transaction_id: m.id,
              product_id: m.id.replace('e2e-product-', ''),
              quantity_deducted: 1, // Assume 1 for E2E logic
              amount: null,
              payment_method: null,
              payment_intent_id: null,
              currency: null
           };
        } else if (m.type === 'draft_quote') {
          return {
             transaction_id: m.id,
             product_id: 'draft_quote',
             quantity_deducted: 0,
             amount: null,
             payment_method: null,
             payment_intent_id: null,
             currency: 'usd',
             mutation_type: 'draft_quote',
             payload: m.notes
          };
        }
        return m;
      });

      const approveFeedMutations = queue.filter(m => m.type === 'approve_agent_feed');

      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      let allOk = true;
      let syncedIds: string[] = [];

      // Sync POS transactions
      if (posTransactions.length > 0) {
        const sessionId = localStorage.getItem('ohc_active_terminal_session_id');
        const resPos = await fetch('/api/v1/payments/terminal/sync_offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({
            session_id: sessionId || undefined,
            transactions: posTransactions
          })
        });
        if (!resPos.ok) {
          allOk = false;
          console.error(`POS Sync failed with status ${resPos.status}`);
        } else {
            syncedIds.push(...queue.filter(m => m.type === 'tap_to_pay').map(m => m.id));
        }
      }

      // Sync general mutations
      if (generalMutations.length > 0) {
        const resGen = await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ mutations: generalMutations })
        });
        if (!resGen.ok) {
          allOk = false;
          console.error(`General Sync failed with status ${resGen.status}`);
        } else {
           syncedIds.push(...queue.filter(m => m.type !== 'tap_to_pay' && m.type !== 'approve_agent_feed').map(m => m.id));
        }
      }

      // Sync approve feed mutations
      if (approveFeedMutations.length > 0) {
         for (const action of approveFeedMutations) {
            try {
               const res = await fetch("/api/agent-feed", {
                  method: "POST",
                  headers: { "Content-Type": "application/json" },
                  body: JSON.stringify({
                    tenant_id: tenantId,
                    decision_id: action.payload.id,
                    approved: action.payload.approved,
                  }),
               });
               if (res.ok) {
                  syncedIds.push(action.id);
               } else {
                  allOk = false;
               }
            } catch (err) {
               allOk = false;
            }
         }
      }

      if (syncedIds.length > 0) {
        await this.removeActions(syncedIds);
      }

      if (allOk) {
        this.retryDelayMs = 1000; // Reset delay on success
      }
    } catch (e) {
      console.error('Failed to sync offline queue:', e);
      if (retryCount < this.maxRetries) {
        const delay = this.retryDelayMs * Math.pow(2, retryCount);
        setTimeout(() => {
          this.syncInProgress = false;
          this.sync(retryCount + 1);
        }, delay);
        return; // Don't unset syncInProgress yet
      }
    } finally {
      this.syncInProgress = false;
    }
  }
}
