// robust IndexedDB helper for Offline Sync and Conflicts

export interface ConflictRecord {
  id: string;
  type: string;
  payload: any;
  timestamp: string;
  errorMsg: string;
  serverState?: any;
}

const CONFLICT_DB_NAME = "OHC_Sync_Conflicts";
const CONFLICT_STORE_NAME = "conflicts";
const CONFLICT_DB_VERSION = 1;

function getConflictDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    if (typeof window === "undefined" || !window.indexedDB) {
      reject(new Error("IndexedDB not available"));
      return;
    }
    const request = window.indexedDB.open(CONFLICT_DB_NAME, CONFLICT_DB_VERSION);
    request.onerror = (event) => {
      reject(request.error);
    };
    request.onsuccess = (event) => {
      resolve((event.target as IDBOpenDBRequest).result);
    };
    request.onupgradeneeded = (event) => {
      const db = (event.target as IDBOpenDBRequest).result;
      if (!db.objectStoreNames.contains(CONFLICT_STORE_NAME)) {
        db.createObjectStore(CONFLICT_STORE_NAME, { keyPath: "id" });
      }
    };
  });
}

export async function storeConflict(conflict: ConflictRecord): Promise<void> {
  try {
    const db = await getConflictDB();
    return new Promise((resolve, reject) => {
      const tx = db.transaction([CONFLICT_STORE_NAME], "readwrite");
      const store = tx.objectStore(CONFLICT_STORE_NAME);
      const req = store.put(conflict);
      req.onsuccess = () => {
          if (typeof window !== 'undefined') {
              window.dispatchEvent(new Event('ohc_sync_conflict_detected'));
          }
          resolve();
      };
      req.onerror = () => reject(req.error);
    });
  } catch (e) {
    console.error("Failed to store conflict", e);
  }
}

export async function getConflicts(): Promise<ConflictRecord[]> {
  try {
    const db = await getConflictDB();
    return new Promise((resolve, reject) => {
      const tx = db.transaction([CONFLICT_STORE_NAME], "readonly");
      const store = tx.objectStore(CONFLICT_STORE_NAME);
      const req = store.getAll();
      req.onsuccess = () => resolve(req.result as ConflictRecord[]);
      req.onerror = () => reject(req.error);
    });
  } catch (e) {
    console.error("Failed to get conflicts", e);
    return [];
  }
}

export async function removeConflict(id: string): Promise<void> {
  try {
    const db = await getConflictDB();
    return new Promise((resolve, reject) => {
      const tx = db.transaction([CONFLICT_STORE_NAME], "readwrite");
      const store = tx.objectStore(CONFLICT_STORE_NAME);
      const req = store.delete(id);
      req.onsuccess = () => resolve();
      req.onerror = () => reject(req.error);
    });
  } catch (e) {
    console.error("Failed to remove conflict", e);
  }
}
