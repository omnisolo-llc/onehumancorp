import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { syncManager } from './syncManager';

describe('SyncManager', () => {
  beforeEach(() => {
    localStorage.clear();
    (syncManager as any).queue = [];
    (syncManager as any).isSyncing = false;
    (syncManager as any).listeners = [];
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('should enqueue a mutation and save it to localStorage', async () => {
    const mutation = {
      mutation_type: 'INVENTORY_DEDUCT' as const,
      product_id: 'test-prod',
      quantity_deducted: 5,
    };

    vi.stubGlobal('navigator', { onLine: false });

    await syncManager.enqueue(mutation);

    const stored = JSON.parse(localStorage.getItem('ohc_offline_mutation_queue') || '[]');
    expect(stored).toHaveLength(1);
    expect(stored[0]).toMatchObject(mutation);
    expect(stored[0].mutation_id).toBeDefined();
    expect(stored[0].timestamp).toBeDefined();
  });

  it('should notify subscribers when the queue changes', async () => {
    const listener = vi.fn();
    syncManager.subscribe(listener);

    const mutation = {
      mutation_type: 'TOGGLE_SOLD_OUT' as const,
      product_id: 'test-prod',
      metadata: { is_sold_out: true },
    };

    vi.stubGlobal('navigator', { onLine: false });

    await syncManager.enqueue(mutation);

    expect(listener).toHaveBeenCalledTimes(2);
    expect(listener).toHaveBeenLastCalledWith({ pendingCount: 1, isSyncing: false });
  });

  it('should attempt to sync when online', async () => {
    const fetchSpy = vi.spyOn(global, 'fetch').mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ success: true, processed_ids: [] }),
      } as Response)
    );

    vi.stubGlobal('navigator', { onLine: true });

    const mutation = {
      mutation_type: 'UPDATE_ORDER_STATUS' as const,
      order_id: 'order-1',
      status: 'Ready',
    };

    await syncManager.enqueue(mutation);

    expect(fetchSpy).toHaveBeenCalled();
  });
});
