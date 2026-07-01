export interface OfflineMutation {
  idempotency_key: string;
  entity_type: string;
  entity_id: string;
  action: string;
  payload: any;
  status: 'pending' | 'syncing' | 'failed';
  created_at: string;
}

class IDBStorage {
  private dbName = 'ohc-offline-store';
  private storeName = 'mutations';
  private db: IDBDatabase | null = null;

  async init(): Promise<void> {
    if (this.db) return;
    return new Promise((resolve, reject) => {
      if (typeof window === 'undefined') return resolve(); // SSR fallback
      const request = indexedDB.open(this.dbName, 1);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };
      request.onupgradeneeded = (event: any) => {
        const db = event.target.result;
        if (!db.objectStoreNames.contains(this.storeName)) {
          db.createObjectStore(this.storeName, { keyPath: 'idempotency_key' });
        }
      };
    });
  }

  private async getStore(mode: IDBTransactionMode = 'readonly'): Promise<IDBObjectStore> {
    await this.init();
    if (!this.db) throw new Error("IndexedDB not available");
    const tx = this.db.transaction(this.storeName, mode);
    return tx.objectStore(this.storeName);
  }

  async setItem(key: string, value: OfflineMutation): Promise<void> {
    const store = await this.getStore('readwrite');
    return new Promise((resolve, reject) => {
      const req = store.put(value);
      req.onsuccess = () => resolve();
      req.onerror = () => reject(req.error);
    });
  }

  async getItem(key: string): Promise<OfflineMutation | null> {
    const store = await this.getStore('readonly');
    return new Promise((resolve, reject) => {
      const req = store.get(key);
      req.onsuccess = () => resolve(req.result || null);
      req.onerror = () => reject(req.error);
    });
  }

  async removeItem(key: string): Promise<void> {
    const store = await this.getStore('readwrite');
    return new Promise((resolve, reject) => {
      const req = store.delete(key);
      req.onsuccess = () => resolve();
      req.onerror = () => reject(req.error);
    });
  }

  async getAll(): Promise<OfflineMutation[]> {
    const store = await this.getStore('readonly');
    return new Promise((resolve, reject) => {
      const req = store.getAll();
      req.onsuccess = () => resolve(req.result || []);
      req.onerror = () => reject(req.error);
    });
  }

  async clear(): Promise<void> {
    const store = await this.getStore('readwrite');
    return new Promise((resolve, reject) => {
      const req = store.clear();
      req.onsuccess = () => resolve();
      req.onerror = () => reject(req.error);
    });
  }
}

const storage = new IDBStorage();

export const offlineSyncService = {
  async addMutation(mutation: Omit<OfflineMutation, 'status' | 'created_at' | 'idempotency_key'>) {
    const key = typeof crypto !== 'undefined' && crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(2);
    const entry: OfflineMutation = {
      ...mutation,
      idempotency_key: key,
      status: 'pending',
      created_at: new Date().toISOString(),
    };
    await storage.setItem(key, entry);
    return entry;
  },

  async getPendingMutations(): Promise<OfflineMutation[]> {
    const all = await storage.getAll();
    const mutations = all.filter(m => m.status === 'pending' || m.status === 'failed');
    return mutations.sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
  },

  async markAsSyncing(key: string) {
    const entry = await storage.getItem(key);
    if (entry) {
      entry.status = 'syncing';
      await storage.setItem(key, entry);
    }
  },

  async markAsFailed(key: string) {
    const entry = await storage.getItem(key);
    if (entry) {
      entry.status = 'failed';
      await storage.setItem(key, entry);
    }
  },

  async removeMutation(key: string) {
    await storage.removeItem(key);
  },

  async clearAll() {
    await storage.clear();
  }
};
