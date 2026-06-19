import { powerSyncDb } from '../../lib/powersync/db';
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
  if (typeof window === "undefined") return;

  if (powerSyncDb && powerSyncDb.database) {
    try {
      const payloadStr = typeof action.payload === 'string' ? action.payload : JSON.stringify(action.payload);
      await powerSyncDb.execute(
        'INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
        [action.id, action.type, payloadStr, action.timestamp.toString()]
      );
      return;
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to enqueue action to PowerSync", err);
      }
    }
  }

  if (!window.indexedDB) return;
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
  if (typeof window === "undefined") return [];

  if (powerSyncDb && powerSyncDb.database) {
    try {
      const result = await powerSyncDb.execute('SELECT * FROM pending_actions');
      const actions: OfflineAction[] = [];
      for (let i = 0; i < result.rows?.length; i++) {
        const row = result.rows?.item(i);
        let payload = row.payload;
        try {
           payload = JSON.parse(row.payload);
        } catch (e) {}
        actions.push({
          id: row.id,
          type: row.type,
          payload: payload,
          timestamp: parseInt(row.timestamp, 10)
        });
      }
      return actions;
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to get actions from PowerSync", err);
      }
    }
  }

  if (!window.indexedDB) return [];
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
  if (typeof window === "undefined") return;

  if (powerSyncDb && powerSyncDb.database) {
    try {
      await powerSyncDb.execute(
        'DELETE FROM pending_actions WHERE id = ?',
        [id]
      );
      return;
    } catch (err) {
      if (process.env.NODE_ENV !== 'test') {
        console.error("Failed to remove action from PowerSync", err);
      }
    }
  }

  if (!window.indexedDB) return;
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
