import { renderHook, act } from '@testing-library/react';
import { useSyncManager } from '../syncManager';
import { useSyncStore } from '../syncStore';
import { describe, it, expect, beforeEach, vi } from 'vitest';

// Mock fetch
global.fetch = vi.fn();

describe('useSyncManager', () => {
  beforeEach(() => {
    useSyncStore.getState().clearQueue();
    vi.resetAllMocks();
  });

  it('processes queue items correctly', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: true, status: 200 });

    const store = useSyncStore.getState();
    const item = store.enqueueMutation({
      type: 'test_mutation',
      payload: { data: 'test' }
    });

    expect(useSyncStore.getState().queue.length).toBe(1);

    const { result } = renderHook(() => useSyncManager());

    await act(async () => {
      await result.current.forceSync();
    });

    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(useSyncStore.getState().queue.length).toBe(0);
  });

  it('handles failed sync items', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: false, status: 500 });

    const store = useSyncStore.getState();
    const item = store.enqueueMutation({
      type: 'test_mutation',
      payload: { data: 'test' }
    });

    const { result } = renderHook(() => useSyncManager());

    await act(async () => {
      await result.current.forceSync();
    });

    expect(global.fetch).toHaveBeenCalledTimes(1);
    const queue = useSyncStore.getState().queue;
    expect(queue.length).toBe(1);
    expect(queue[0].status).toBe('failed');
    expect(queue[0].retryCount).toBe(1);
  });
});
