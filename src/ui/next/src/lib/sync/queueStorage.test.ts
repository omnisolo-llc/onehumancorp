import { afterEach, expect, it, vi } from 'vitest';
import { queueTransaction, selectedQueueAdapter } from './queueStorage';
import { getPowerSyncDB } from '../powersync/db';
vi.mock('../powersync/db', () => ({ getPowerSyncDB: vi.fn() }));
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); localStorage.clear(); });
function database() {
  const request = { result: [], onsuccess: null as null | (() => void), onerror: null as null | (() => void) };
  const writes: unknown[] = [];
  const close = vi.fn();
  const tx = { oncomplete: null as null | (() => void), onabort: null as null | (() => void), onerror: null as null | (() => void), error: new Error('Commit aborted'), abort: vi.fn(() => queueMicrotask(() => tx.onabort?.())), objectStore: () => ({ getAll: () => request, put: (row: unknown) => { writes.push(row); } }) };
  const db = { close, transaction: () => tx };
  vi.stubGlobal('indexedDB', { open: () => {
    const opening = { result: db, onsuccess: null as null | (() => void) };
    queueMicrotask(() => opening.onsuccess?.()); return opening;
  } });
  return { request, tx, writes, close };
}
it('waits for IndexedDB transaction commit rather than successful individual requests', async () => {
  const db = database(); let settled = false;
  const pending = queueTransaction('indexeddb', () => ({ writes: [{ id: 'a', type: 'test', payload: 'saved', timestamp: 1 }], result: 'committed' })).then(value => { settled = true; return value; });
  await vi.waitFor(() => expect(db.request.onsuccess).not.toBeNull());
  db.request.onsuccess!(); await Promise.resolve();
  expect(db.writes).toHaveLength(1); expect(settled).toBe(false);
  db.tx.oncomplete!(); expect(await pending).toBe('committed'); expect(db.close).toHaveBeenCalledOnce();
});
it('rejects IndexedDB commit abort after individual writes succeed', async () => {
  const db = database();
  const pending = queueTransaction('indexeddb', () => ({ writes: [{ id: 'a', type: 'test', payload: 'saved', timestamp: 1 }], result: undefined }));
  await vi.waitFor(() => expect(db.request.onsuccess).not.toBeNull()); db.request.onsuccess!(); db.tx.onabort!();
  await expect(pending).rejects.toThrow('Commit aborted'); expect(db.close).toHaveBeenCalledOnce();
});
it('pins the selected adapter across a later PowerSync failure', async () => {
  vi.spyOn(navigator.locks, 'request').mockImplementation((async (_name: string, work: () => Promise<unknown>) => work()) as never);
  vi.mocked(getPowerSyncDB).mockResolvedValue({} as never);
  expect(await selectedQueueAdapter()).toBe('powersync');
  vi.mocked(getPowerSyncDB).mockRejectedValue(new Error('Lost storage'));
  expect(await selectedQueueAdapter()).toBe('powersync');
  await expect(queueTransaction('powersync', rows => ({ result: rows }))).rejects.toThrow('Lost storage');
});
