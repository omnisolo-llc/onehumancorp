export class LocalDB {
  private dbName = 'ohc_local_db';
  private dbVersion = 1;
  private db: IDBDatabase | null = null;

  public async init(): Promise<void> {
    if (typeof window === 'undefined') return;

    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, this.dbVersion);

      request.onerror = () => reject(request.error);

      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };

      request.onupgradeneeded = (event: IDBVersionChangeEvent) => {
        const db = (event.target as IDBOpenDBRequest).result;

        if (!db.objectStoreNames.contains('mutation_queue')) {
          db.createObjectStore('mutation_queue', { keyPath: 'id' });
        }

        if (!db.objectStoreNames.contains('appointments')) {
          db.createObjectStore('appointments', { keyPath: 'id' });
        }
      };
    });
  }

  // Generic Operations
  private async getStore(storeName: string, mode: IDBTransactionMode): Promise<IDBObjectStore> {
    if (!this.db) {
      await this.init();
    }
    const transaction = this.db!.transaction(storeName, mode);
    return transaction.objectStore(storeName);
  }

  // Mutation Queue Operations
  public async getMutations(): Promise<any[]> {
    if (typeof window === 'undefined') return [];
    try {
      const store = await this.getStore('mutation_queue', 'readonly');
      return new Promise((resolve, reject) => {
        const request = store.getAll();
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
      });
    } catch {
      return [];
    }
  }

  public async addMutation(mutation: any): Promise<void> {
    if (typeof window === 'undefined') return;
    try {
      const store = await this.getStore('mutation_queue', 'readwrite');
      return new Promise((resolve, reject) => {
        const request = store.put(mutation);
        request.onsuccess = () => resolve();
        request.onerror = () => reject(request.error);
      });
    } catch (e) {
      console.error('Failed to add mutation to IndexedDB', e);
    }
  }

  public async clearMutations(): Promise<void> {
    if (typeof window === 'undefined') return;
    try {
      const store = await this.getStore('mutation_queue', 'readwrite');
      return new Promise((resolve, reject) => {
        const request = store.clear();
        request.onsuccess = () => resolve();
        request.onerror = () => reject(request.error);
      });
    } catch (e) {
      console.error('Failed to clear mutations from IndexedDB', e);
    }
  }

  public async removeMutations(ids: string[]): Promise<void> {
    if (typeof window === 'undefined') return;
    try {
      const store = await this.getStore('mutation_queue', 'readwrite');
      return new Promise((resolve, reject) => {
        let count = 0;
        let hasError = false;

        if (ids.length === 0) {
            resolve();
            return;
        }

        for (const id of ids) {
          const request = store.delete(id);
          request.onsuccess = () => {
            count++;
            if (count === ids.length && !hasError) resolve();
          };
          request.onerror = () => {
            hasError = true;
            reject(request.error);
          };
        }
      });
    } catch (e) {
        console.error('Failed to remove mutations from IndexedDB', e);
    }
  }

  // Appointment Operations
  public async setAppointments(appointments: any[]): Promise<void> {
    if (typeof window === 'undefined') return;
    try {
      const store = await this.getStore('appointments', 'readwrite');
      return new Promise((resolve, reject) => {
        store.clear(); // Clear existing
        let count = 0;
        let hasError = false;

        if (appointments.length === 0) {
            resolve();
            return;
        }

        for (const apt of appointments) {
          const request = store.put(apt);
          request.onsuccess = () => {
            count++;
            if (count === appointments.length && !hasError) resolve();
          };
          request.onerror = () => {
            hasError = true;
            reject(request.error);
          };
        }
      });
    } catch (e) {
      console.error('Failed to set appointments in IndexedDB', e);
    }
  }

  public async getAppointments(): Promise<any[]> {
    if (typeof window === 'undefined') return [];
    try {
      const store = await this.getStore('appointments', 'readonly');
      return new Promise((resolve, reject) => {
        const request = store.getAll();
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
      });
    } catch {
      return [];
    }
  }
}

export const localDB = new LocalDB();
