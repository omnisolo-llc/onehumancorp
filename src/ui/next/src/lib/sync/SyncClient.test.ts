import { describe, it, expect, vi, beforeEach } from 'vitest';
import { SyncClient } from './SyncClient';
import { getPowerSyncDB } from '../powersync/db';

vi.mock('../powersync/db', () => ({
  getPowerSyncDB: vi.fn()
}));

describe('SyncClient', () => {
  const mockDb = {
    execute: vi.fn(),
    getAll: vi.fn()
  };

  beforeEach(() => {
    vi.clearAllMocks();
    (getPowerSyncDB as any).mockResolvedValue(mockDb);
    global.fetch = vi.fn();
  });

  it('queues mutation locally', async () => {
    const payload = {
      tableName: 'tasks',
      operation: 'INSERT' as const,
      payloadJson: '{"title":"Fix sink"}'
    };

    Object.defineProperty(window, 'navigator', {
        value: { onLine: false },
        writable: true
    });

    const res = await SyncClient.queueMutation(payload);
    expect(res.id).toBeDefined();
    expect(res.idempotencyKey).toBeDefined();
    expect(mockDb.execute).toHaveBeenCalled();
  });

  it('syncs queued mutations', async () => {
    mockDb.getAll.mockResolvedValue([
      {
        id: '123',
        type: 'LOCAL_MUTATION',
        payload: JSON.stringify({
            id: '123',
            tableName: 'tasks',
            operation: 'INSERT',
            payloadJson: '{}',
            timestamp: 1000,
            idempotencyKey: 'key1'
        })
      }
    ]);

    (global.fetch as any).mockResolvedValueOnce({ ok: true }); // token
    (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ acked_ids: ['123'], failed_ids: [] })
    });

    await SyncClient.syncNow();
    expect(global.fetch).toHaveBeenCalledTimes(2);
    expect(mockDb.execute).toHaveBeenCalledWith('DELETE FROM pending_actions WHERE id = ?', ['123']);
  });
});
