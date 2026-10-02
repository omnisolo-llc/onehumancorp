import { getPowerSyncDB } from '../powersync/db';
export type QueueAdapter = 'powersync' | 'indexeddb';
export type StoredQueueRow = { id: string; type: string; payload: unknown; timestamp: number };
type Change<T> = { writes?: StoredQueueRow[]; result: T };
const ADAPTER_KEY = 'omnisolo_queue_adapter_v2';

function openIndexedDB(): Promise<IDBDatabase> {
  if (typeof window === 'undefined' || !window.indexedDB) return Promise.reject(new Error('Offline storage unavailable: IndexedDB is not supported'));
  return new Promise((resolve, reject) => {
    const request = window.indexedDB.open('OMNISOLO_Offline_Queue', 1);
    request.onerror = () => reject(request.error ?? new Error('Offline storage open failed'));
    request.onblocked = () => reject(new Error('Offline storage upgrade blocked'));
    request.onupgradeneeded = () => { if (!request.result.objectStoreNames.contains('actions')) request.result.createObjectStore('actions', { keyPath: 'id' }); };
    request.onsuccess = () => resolve(request.result);
  });
}

/** A durable adapter owner is selected once under an origin-wide lock. */
export async function selectedQueueAdapter(): Promise<QueueAdapter> {
  if (typeof window === 'undefined' || !navigator.locks) throw new Error('Offline storage requires cross-tab locks');
  return navigator.locks.request('omnisolo_queue_adapter_v2', async () => {
    const selected = localStorage.getItem(ADAPTER_KEY);
    if (selected === 'powersync' || selected === 'indexeddb') return selected;
    if (selected !== null) throw new Error('Invalid offline storage owner');
    let adapter: QueueAdapter;
    try { await getPowerSyncDB(); adapter = 'powersync'; }
    catch { const db = await openIndexedDB(); db.close(); adapter = 'indexeddb'; }
    localStorage.setItem(ADAPTER_KEY, adapter);
    if (localStorage.getItem(ADAPTER_KEY) !== adapter) throw new Error('Offline storage owner could not be persisted');
    return adapter;
  });
}

/** Callback is synchronous so a read-modify-write stays inside one storage transaction. */
export async function queueTransaction<T>(adapter: QueueAdapter, work: (rows: StoredQueueRow[]) => Change<T>): Promise<T> {
  if (adapter === 'powersync') {
    const db = await getPowerSyncDB();
    return db.writeTransaction(async tx => {
      const rows = await tx.getAll<StoredQueueRow>('SELECT * FROM local_pending_actions ORDER BY timestamp ASC');
      const change = work(rows);
      for (const row of change.writes ?? []) await tx.execute('INSERT OR REPLACE INTO local_pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)', [row.id, row.type, row.payload, row.timestamp]);
      return change.result;
    });
  }
  const db = await openIndexedDB();
  return new Promise<T>((resolve, reject) => {
    let failure: unknown;
    let result: T;
    try {
      const tx = db.transaction(['actions'], 'readwrite');
      tx.oncomplete = () => { db.close(); resolve(result); };
      tx.onabort = () => { db.close(); reject(failure ?? tx.error ?? new Error('Offline storage transaction aborted')); };
      tx.onerror = () => { failure ??= tx.error; };
      const store = tx.objectStore('actions');
      const request = store.getAll();
      request.onerror = () => { failure ??= request.error; };
      request.onsuccess = () => {
        try {
          const change = work(request.result as StoredQueueRow[]);
          result = change.result;
          for (const row of change.writes ?? []) store.put(row);
        } catch (error) { failure = error; tx.abort(); }
      };
    } catch (error) { db.close(); reject(error); }
  });
}

export async function readOtherAdapter(selected: QueueAdapter): Promise<{ rows: StoredQueueRow[]; unavailable: boolean }> {
  const other = selected === 'powersync' ? 'indexeddb' : 'powersync';
  try { return { rows: await queueTransaction(other, rows => ({ result: rows })), unavailable: false }; }
  catch { return { rows: [], unavailable: true }; }
}
