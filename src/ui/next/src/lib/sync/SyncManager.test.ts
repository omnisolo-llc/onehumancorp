import { describe, it, expect, vi, beforeEach } from 'vitest';
import { SyncManager } from './SyncManager';

describe('SyncManager', () => {
  beforeEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
    // Reset singleton instance
    (SyncManager as any).instance = null;
    vi.useRealTimers();
  });

  it('should enqueue a mutation to localStorage', () => {
    const manager = SyncManager.getInstance();
    const mutation = { id: 'test-1', type: 'test' };

    manager.enqueue(mutation);

    const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    expect(queue).toHaveLength(1);
    expect(queue[0]).toEqual(mutation);
  });

  it('should notify listeners on enqueue', () => {
    const manager = SyncManager.getInstance();
    const dispatchSpy = vi.spyOn(window, 'dispatchEvent');

    manager.enqueue({ id: 'test-2' });

    expect(dispatchSpy).toHaveBeenCalled();
    const lastEvent = dispatchSpy.mock.calls.find(call => (call[0] as CustomEvent).type === 'ohc_sync_status');
    expect(lastEvent).toBeDefined();
  });

  it('should attempt sync when online and enqueued', async () => {
    const manager = SyncManager.getInstance();
    const fetchSpy = vi.fn().mockResolvedValue({ ok: true });
    global.fetch = fetchSpy;

    // Mock online
    Object.defineProperty(navigator, 'onLine', {
      configurable: true,
      value: true,
    });

    manager.enqueue({ id: 'test-3', product_id: 'p1', amount: 10 });

    await vi.waitFor(() => {
      expect(localStorage.getItem('ohc_offline_queue')).toBe('[]');
    });

    expect(fetchSpy).toHaveBeenCalledWith('/api/v1/sync/offline', expect.objectContaining({
      method: 'POST',
      body: expect.stringContaining('"transactions":')
    }));

    const body = JSON.parse(fetchSpy.mock.calls[0][1].body);
    expect(body.transactions[0]).toMatchObject({
      id: 'test-3',
      amount_cents: 1000,
      product_id: 'p1'
    });
  });

  it('should handle sync failure with retries', async () => {
    const manager = SyncManager.getInstance();
    const fetchSpy = vi.fn().mockRejectedValue(new Error('Network error'));
    global.fetch = fetchSpy;

    Object.defineProperty(navigator, 'onLine', {
      configurable: true,
      value: true,
    });

    manager.enqueue({ id: 'test-4' });

    // Should retry after ~1s. We use waitFor with a generous timeout.
    await vi.waitFor(() => {
      expect(fetchSpy).toHaveBeenCalledTimes(2);
    }, { timeout: 5000 });
  });
});
