import { describe, it, expect, beforeEach } from 'vitest';
import { useOfflineSyncStore } from './offlineSyncStore';

describe('offlineSyncStore', () => {
  beforeEach(() => {
    useOfflineSyncStore.getState().clearQueue();
  });

  it('should enqueue a mutation', () => {
    const store = useOfflineSyncStore.getState();
    store.enqueueMutation('/api/test', 'POST', 'TEST_EVENT', { data: 123 });

    const queue = useOfflineSyncStore.getState().queue;
    expect(queue.length).toBe(1);
    expect(queue[0].endpoint).toBe('/api/test');
    expect(queue[0].method).toBe('POST');
    expect(queue[0].type).toBe('TEST_EVENT');
    expect(queue[0].payload).toEqual({ data: 123 });
    expect(queue[0].retryCount).toBe(0);
    expect(queue[0].id).toBeDefined();
    expect(queue[0].timestamp).toBeDefined();
  });

  it('should dequeue a mutation', () => {
    const store = useOfflineSyncStore.getState();
    store.enqueueMutation('/api/test', 'POST', 'TEST_EVENT', { data: 123 });
    const id = useOfflineSyncStore.getState().queue[0].id;

    useOfflineSyncStore.getState().dequeueMutation(id);
    expect(useOfflineSyncStore.getState().queue.length).toBe(0);
  });

  it('should increment retry count', () => {
    const store = useOfflineSyncStore.getState();
    store.enqueueMutation('/api/test', 'POST', 'TEST_EVENT', { data: 123 });
    const id = useOfflineSyncStore.getState().queue[0].id;

    useOfflineSyncStore.getState().incrementRetry(id);
    expect(useOfflineSyncStore.getState().queue[0].retryCount).toBe(1);
  });
});
