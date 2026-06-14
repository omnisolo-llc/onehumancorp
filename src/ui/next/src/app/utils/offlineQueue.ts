export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

const DB_NAME = "OHC_Offline_Queue";
const STORE_NAME = "actions";
const JOBS_STORE_NAME = "jobs";
const DB_VERSION = 2;

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
      if (!db.objectStoreNames.contains(JOBS_STORE_NAME)) {
        db.createObjectStore(JOBS_STORE_NAME, { keyPath: "id" });
      }
    };
  });
}

export async function cacheJobs(jobs: any[]): Promise<void> {
  if (typeof window === "undefined" || !window.indexedDB) return;
  try {
    const db = await getDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([JOBS_STORE_NAME], "readwrite");
      const store = transaction.objectStore(JOBS_STORE_NAME);

      // Clear existing jobs first, to ensure no deleted jobs stay around
      store.clear().onsuccess = () => {
        let count = 0;
        if (jobs.length === 0) return resolve();

        jobs.forEach(job => {
          const req = store.put(job);
          req.onsuccess = () => {
            count++;
            if (count === jobs.length) resolve();
          };
          req.onerror = () => reject(req.error);
        });
      };

      transaction.onerror = () => reject(transaction.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to cache jobs", err);
    }
  }
}

export async function getCachedJobs(): Promise<any[]> {
  if (typeof window === "undefined" || !window.indexedDB) return [];
  try {
    const db = await getDB();
    return new Promise((resolve, reject) => {
      const transaction = db.transaction([JOBS_STORE_NAME], "readonly");
      const store = transaction.objectStore(JOBS_STORE_NAME);
      const request = store.getAll();

      request.onsuccess = () => {
        resolve(request.result || []);
      };
      request.onerror = () => reject(request.error);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to get cached jobs", err);
    }
    return [];
  }
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
