/// <reference types="node" />
import { getPowerSyncDB } from '../../lib/powersync/db';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

const DB_NAME = "OHC_Offline_Queue";
const STORE_NAME = "actions";
const DB_VERSION = 1;

// Fallback to IndexedDB if PowerSync (SQLite) fails or isn't supported
function getIndexedDB(): Promise<IDBDatabase> {
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

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = await getPowerSyncDB();
    // Using local_transaction_queue mimicking the backend LOCAL_TRANSACTION_QUEUE
    await db.execute(
      'INSERT OR REPLACE INTO local_transaction_queue (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
      [action.id, action.type, JSON.stringify(action.payload), action.timestamp]
    );
    return;
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.warn("PowerSync SQLite unavailable, falling back to IndexedDB", err);
    }
  }

  // Fallback
  if (!window.indexedDB) return;
  try {
    const db = await getIndexedDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([STORE_NAME], "readwrite");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.put(action);
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action to fallback IndexedDB", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined") return [];
  try {
    const db = await getPowerSyncDB();
    const result = await db.getAll('SELECT * FROM local_transaction_queue ORDER BY timestamp ASC');
    return result.map((row: any) => ({
      id: row.id,
      type: row.type,
      payload: JSON.parse(row.payload),
      timestamp: row.timestamp
    }));
  } catch (err) {
     if (process.env.NODE_ENV !== 'test') {
        console.warn("PowerSync SQLite unavailable for getActions, falling back to IndexedDB", err);
     }
  }

  if (!window.indexedDB) return [];
  try {
    const db = await getIndexedDB();
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
      console.error("Failed to get actions from fallback IndexedDB", err);
    }
    return [];
  }
}

export async function removeAction(id: string): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = await getPowerSyncDB();
    await db.execute('DELETE FROM local_transaction_queue WHERE id = ?', [id]);
    return;
  } catch (err) {
     if (process.env.NODE_ENV !== 'test') {
        console.warn("PowerSync SQLite unavailable for removeAction, falling back to IndexedDB", err);
     }
  }

  if (!window.indexedDB) return;
  try {
    const db = await getIndexedDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([STORE_NAME], "readwrite");
      const store = transaction.objectStore(STORE_NAME);
      const request = store.delete(id);
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action from fallback IndexedDB", err);
    }
  }
}
