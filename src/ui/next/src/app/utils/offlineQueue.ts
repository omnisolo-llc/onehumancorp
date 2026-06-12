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
<<<<<<< HEAD
=======
    if (typeof window === "undefined" || !window.indexedDB) {
      return reject(new Error("IndexedDB not available"));
    }
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))

    const request = window.indexedDB.open(DB_NAME, DB_VERSION);

    request.onerror = (event) => {
<<<<<<< HEAD
      // Suppress error log if IndexedDB is intentionally unavailable in tests
      if (process.env.NODE_ENV !== 'test') {
        console.error("IndexedDB error", event);
      }
=======
      console.error("IndexedDB error", event);
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
<<<<<<< HEAD
  if (typeof window === "undefined" || !window.indexedDB) return;
=======
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
<<<<<<< HEAD
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
=======
    console.error("Failed to enqueue action", err);
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
  }
}

export async function getActions(): Promise<OfflineAction[]> {
<<<<<<< HEAD
  if (typeof window === "undefined" || !window.indexedDB) return [];
=======
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
<<<<<<< HEAD
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to get actions", err);
    }
=======
    console.error("Failed to get actions", err);
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    return [];
  }
}

export async function removeAction(id: string): Promise<void> {
<<<<<<< HEAD
  if (typeof window === "undefined" || !window.indexedDB) return;
=======
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
<<<<<<< HEAD
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
=======
    console.error("Failed to remove action", err);
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
  }
}
