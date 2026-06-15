/// <reference types="node" />
export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

const DB_NAME = "OHC_Offline_Queue";
const STORE_NAME = "actions";
const DB_VERSION = 1;

function getDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {

    const request = window.indexedDB.open(DB_NAME, DB_VERSION);

    request.onerror = (event) => {
      // Suppress error log if IndexedDB is intentionally unavailable in tests
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

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined" || !window.indexedDB) return;
  try {
    const db = await getDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([STORE_NAME], "readwrite");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.put(action);

      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined" || !window.indexedDB) return [];
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

export async function removeAction(id: string): Promise<void> {
  if (typeof window === "undefined" || !window.indexedDB) return;
  try {
    const db = await getDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([STORE_NAME], "readwrite");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.delete(id);

      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}

export class SyncEngine {
  private isSyncing = false;
  private syncInterval: NodeJS.Timeout | null = null;
  private tenantId: string = "";

  constructor(tenantId: string) {
    this.tenantId = tenantId;
    if (typeof window !== 'undefined') {
      window.addEventListener('online', this.handleOnline.bind(this));
    }
  }

  public startPeriodicSync(intervalMs: number = 30000) {
    if (this.syncInterval) clearInterval(this.syncInterval);
    this.syncInterval = setInterval(() => {
      if (typeof navigator !== 'undefined' && navigator.onLine) {
        this.sync();
      }
    }, intervalMs);
  }

  public stopPeriodicSync() {
    if (this.syncInterval) {
      clearInterval(this.syncInterval);
      this.syncInterval = null;
    }
  }

  private handleOnline() {
    this.sync();
  }

  public async sync(): Promise<void> {
    if (this.isSyncing) return;
    this.isSyncing = true;

    try {
      // First try to sync localStorage queue from Tauri POS/Dashboard
      let localQueueStr = "[]";
      if (typeof localStorage !== "undefined") {
        localQueueStr = localStorage.getItem("ohc_offline_queue") || "[]";
      }
      let localQueue = JSON.parse(localQueueStr);

      let allActions = await getActions();

      // Merge localStorage mock queue to real indexedDB actions if any
      if (localQueue && Array.isArray(localQueue)) {
        for (const item of localQueue) {
          const id = item.id || `local-${Date.now()}-${Math.random()}`;
          const action = {
            id,
            type: item.type || "InventoryUpdate",
            payload: JSON.stringify(item),
            timestamp: item.timestamp || Date.now()
          };
          allActions.push(action);
        }
      }

      if (allActions.length === 0) {
        this.isSyncing = false;
        return;
      }

      const batchId = `batch-${Date.now()}`;
      const events = allActions.map(a => ({
        id: a.id,
        action_type: a.type,
        payload: typeof a.payload === 'string' ? a.payload : JSON.stringify(a.payload),
        timestamp_ms: a.timestamp
      }));

      const response = await fetch('/api/v1/sync/events', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          tenant_id: this.tenantId,
          batch_id: batchId,
          events
        })
      });

      if (response.ok) {
        // Clear synced actions from IndexedDB
        for (const action of allActions.filter(a => !a.id.startsWith('local-'))) {
            // we remove all actions except those that caused conflicts which we might retry or drop depending on the policy.
            // For now we drop them so they don't block the queue. The conflict resolution is server-side.
            await removeAction(action.id);
        }
        // Clear local storage queue
        if (typeof localStorage !== "undefined") {
          localStorage.setItem("ohc_offline_queue", "[]");
        }
      }
    } catch (error) {
      console.error("Sync Engine failed to sync", error);
    } finally {
      this.isSyncing = false;
    }
  }
}
