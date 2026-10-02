import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { MutationService } from './MutationService';
import { readQueueOwner } from './queueIdentity';
import type { StoredQueueRow } from './queueStorage';
const storage = vi.hoisted(() => ({ getAll: vi.fn(), writeTransaction: vi.fn() }));
const notifications = vi.hoisted(() => ({ getInstance: vi.fn() }));
vi.mock('../powersync/db', () => ({ getPowerSyncDB: async () => storage }));
vi.mock('./SyncManager', () => ({ SyncManager: { getInstance: notifications.getInstance } }));
vi.mock('./queueIdentity', async original => ({ ...await original<object>(), readQueueOwner: vi.fn() }));
let stored: Map<string, StoredQueueRow>;
let failSecond: boolean;
beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear(); stored = new Map(); failSecond = false;
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'a', tenantId: 't' });
  vi.spyOn(navigator.locks, 'request').mockImplementation((async (_name: string, work: () => Promise<unknown>) => work()) as never);
  notifications.getInstance.mockReturnValue({ sync: vi.fn(async () => {}) });
  vi.stubGlobal('indexedDB', undefined);
  storage.getAll.mockImplementation(async () => [...stored.values()]);
  storage.writeTransaction.mockImplementation(async (work: (tx: unknown) => Promise<unknown>) => {
    const staged = new Map(stored); let count = 0;
    const result = await work({ getAll: async () => [...staged.values()], execute: async (_sql: string, values: unknown[]) => {
      count += 1; if (failSecond && count === 2) throw new Error('Second item write failed');
      staged.set(String(values[0]), { id: String(values[0]), type: String(values[1]), payload: values[2], timestamp: Number(values[3]) });
    } });
    stored = staged; return result;
  });
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
it('propagates a real enqueue adapter failure and rolls back the optimistic view', async () => {
  storage.writeTransaction.mockRejectedValue(new Error('PowerSync storage unavailable'));
  const optimistic = vi.fn(); const rollback = vi.fn();
  await expect(MutationService.getInstance().executeMutation('cash_sale', { amount_cents: 5000 }, optimistic, rollback)).rejects.toThrow('storage unavailable');
  expect(optimistic).toHaveBeenCalledOnce(); expect(rollback).toHaveBeenCalledOnce(); expect(notifications.getInstance).not.toHaveBeenCalled();
});
it('persists a two-item sale atomically and retries only a failed pre-commit write without duplication', async () => {
  failSecond = true;
  const optimistic = vi.fn(); const rollback = vi.fn();
  const payloads = [{ product_id: 'a', amount_cents: 1000 }, { product_id: 'b', amount_cents: 2000 }];
  const service = MutationService.getInstance();
  await expect(service.executeMutationBatch('cash_sale', payloads, optimistic, rollback)).rejects.toThrow('Second item write failed');
  expect(stored.size).toBe(0); expect(rollback).toHaveBeenCalledOnce(); expect(notifications.getInstance).not.toHaveBeenCalled();
  failSecond = false;
  await service.executeMutationBatch('cash_sale', payloads, optimistic, rollback);
  expect(stored.size).toBe(2);
  expect([...stored.values()].map(row => JSON.parse(String(row.payload)).action.payload.product_id)).toEqual(['a', 'b']);
  expect(storage.writeTransaction).toHaveBeenCalledTimes(2); expect(optimistic).toHaveBeenCalledTimes(2); expect(rollback).toHaveBeenCalledOnce();
});
it('does not report a committed sale as failed when sync notification throws', async () => {
  notifications.getInstance.mockImplementation(() => { throw new Error('Notification unavailable'); });
  vi.spyOn(console, 'error').mockImplementation(() => {});
  const rollback = vi.fn();
  await expect(MutationService.getInstance().executeMutation('cash_sale', { amount_cents: 5000 }, vi.fn(), rollback)).resolves.toBeUndefined();
  expect(stored.size).toBe(1); expect(rollback).not.toHaveBeenCalled();
});
