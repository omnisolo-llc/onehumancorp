import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { MutationService } from './MutationService';

type StoredIntent = { id: string; payload: string };
type Transaction = { execute: (sql: string, values: unknown[]) => Promise<void> };
const storage = vi.hoisted(() => ({ execute: vi.fn(), writeTransaction: vi.fn() }));
const notifications = vi.hoisted(() => ({ getInstance: vi.fn() }));
vi.mock('../powersync/db', () => ({ getPowerSyncDB: async () => storage }));
vi.mock('./SyncManager', () => ({ SyncManager: { getInstance: notifications.getInstance } }));

beforeEach(() => {
  vi.clearAllMocks();
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  notifications.getInstance.mockReturnValue({ sync: vi.fn(async () => {}) });
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

it('propagates a real enqueue adapter failure and rolls back the optimistic view', async () => {
  storage.execute.mockRejectedValue(new Error('PowerSync unavailable'));
  vi.stubGlobal('indexedDB', undefined);
  const optimistic = vi.fn();
  const rollback = vi.fn();
  await expect(MutationService.getInstance().executeMutation('cash_sale', { amount_cents: 5000 }, optimistic, rollback)).rejects.toThrow(/storage|IndexedDB/);
  expect(optimistic).toHaveBeenCalledOnce();
  expect(rollback).toHaveBeenCalledOnce();
  expect(notifications.getInstance).not.toHaveBeenCalled();
});

it('persists a two-item sale atomically and retries after partial write failure without duplication', async () => {
  let stored = new Map<string, StoredIntent>();
  let failSecond = true;
  vi.stubGlobal('indexedDB', undefined);
  storage.writeTransaction.mockImplementation(async (write: (tx: Transaction) => Promise<void>) => {
    const staged = new Map(stored);
    let count = 0;
    await write({ execute: async (_sql, values) => {
      count += 1;
      if (failSecond && count === 2) throw new Error('Second item write failed');
      staged.set(String(values[0]), { id: String(values[0]), payload: String(values[2]) });
    } });
    stored = staged;
  });
  const optimistic = vi.fn();
  const rollback = vi.fn();
  const payloads = [{ product_id: 'a', amount_cents: 1000 }, { product_id: 'b', amount_cents: 2000 }];
  const service = MutationService.getInstance();
  await expect(service.executeMutationBatch('cash_sale', payloads, optimistic, rollback)).rejects.toThrow(/storage|IndexedDB/);
  expect(stored.size).toBe(0);
  expect(rollback).toHaveBeenCalledOnce();
  expect(notifications.getInstance).not.toHaveBeenCalled();
  failSecond = false;
  await service.executeMutationBatch('cash_sale', payloads, optimistic, rollback);
  expect(stored.size).toBe(2);
  expect([...stored.values()].map(row => JSON.parse(row.payload).product_id)).toEqual(['a', 'b']);
  expect(storage.writeTransaction).toHaveBeenCalledTimes(2);
  expect(optimistic).toHaveBeenCalledTimes(2);
  expect(rollback).toHaveBeenCalledOnce();
});

it('does not report a committed sale as failed when sync notification throws', async () => {
  storage.execute.mockResolvedValue(undefined);
  notifications.getInstance.mockImplementation(() => { throw new Error('Notification unavailable'); });
  vi.spyOn(console, 'error').mockImplementation(() => {});
  const rollback = vi.fn();
  await expect(MutationService.getInstance().executeMutation('cash_sale', { amount_cents: 5000 }, vi.fn(), rollback)).resolves.toBeUndefined();
  expect(rollback).not.toHaveBeenCalled();
});
