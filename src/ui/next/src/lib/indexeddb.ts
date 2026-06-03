export function openDB(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open("ohc_offline_db", 1);

    request.onupgradeneeded = (event: any) => {
      const db = event.target.result;
      if (!db.objectStoreNames.contains("offline_queue")) {
        db.createObjectStore("offline_queue", { keyPath: "id" });
      }
    };

    request.onsuccess = (event: any) => resolve(event.target.result);
    request.onerror = (event: any) => reject(event.target.error);
  });
}

export async function addToOfflineQueue(item: any): Promise<void> {
  const db = await openDB();
  return new Promise((resolve, reject) => {
    const transaction = db.transaction(["offline_queue"], "readwrite");
    const store = transaction.objectStore("offline_queue");
    const request = store.add(item);

    request.onsuccess = () => resolve();
    request.onerror = (event: any) => reject(event.target.error);
  });
}

export async function getOfflineQueueCount(): Promise<number> {
    try {
      const db = await openDB();
      return new Promise((resolve, reject) => {
        const transaction = db.transaction(["offline_queue"], "readonly");
        const store = transaction.objectStore("offline_queue");
        const request = store.count();

        request.onsuccess = (event: any) => resolve(event.target.result);
        request.onerror = (event: any) => reject(event.target.error);
      });
    } catch (e) {
        return 0;
    }
}
