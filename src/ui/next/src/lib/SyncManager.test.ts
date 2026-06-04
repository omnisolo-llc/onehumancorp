import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { SyncManager } from './SyncManager';
import { useOfflineSyncStore } from './offlineSyncStore';

// Mock navigator.onLine
Object.defineProperty(navigator, 'onLine', {
  value: true,
  writable: true
});

global.fetch = vi.fn();

describe('SyncManager', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    useOfflineSyncStore.getState().clearQueue();
    useOfflineSyncStore.getState().setSyncing(false);
    (global.fetch as any).mockClear();
    navigator.onLine = true;
  });

  afterEach(() => {
    SyncManager.stop();
    vi.useRealTimers();
  });

  it('should not sync if offline', async () => {
    navigator.onLine = false;
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    await SyncManager.sync();

    expect(global.fetch).not.toHaveBeenCalled();
    expect(useOfflineSyncStore.getState().queue.length).toBe(1);
  });

  it('should not sync if already syncing', async () => {
    useOfflineSyncStore.getState().setSyncing(true);
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    await SyncManager.sync();

    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('should sync successfully and dequeue', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: true, status: 200 });
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    await SyncManager.sync();

    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(useOfflineSyncStore.getState().queue.length).toBe(0);
  });

  it('should increment retry count on network error', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    await SyncManager.sync();

    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(useOfflineSyncStore.getState().queue.length).toBe(1);
    expect(useOfflineSyncStore.getState().queue[0].retryCount).toBe(1);
  });

  it('should dequeue on unrecoverable client error', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 400,
      text: () => Promise.resolve('Bad Request')
    });
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    await SyncManager.sync();

    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(useOfflineSyncStore.getState().queue.length).toBe(0);
  });

  it('should drop after max retries', async () => {
    (global.fetch as any).mockRejectedValue(new Error('Network error'));
    useOfflineSyncStore.getState().enqueueMutation('/api/test', 'POST', 'TEST', { a: 1 });

    const eventId = useOfflineSyncStore.getState().queue[0].id;
    useOfflineSyncStore.getState().incrementRetry(eventId); // 1
    useOfflineSyncStore.getState().incrementRetry(eventId); // 2
    useOfflineSyncStore.getState().incrementRetry(eventId); // 3 (MAX_RETRIES)

    await SyncManager.sync();

    // Should not fetch because retryCount >= 3
    expect(global.fetch).not.toHaveBeenCalled();
    // Should dequeue
    expect(useOfflineSyncStore.getState().queue.length).toBe(0);
  });
});
