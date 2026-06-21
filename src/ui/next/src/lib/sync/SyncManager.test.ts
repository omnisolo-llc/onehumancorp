import { describe, it, expect, beforeEach, vi } from 'vitest';
import { SyncManager } from './SyncManager';

describe('SyncManager', () => {
  beforeEach(() => {
    // Reset singleton instance between tests
    (SyncManager as any).instance = undefined;
    vi.clearAllMocks();
  });

  it('is a singleton', () => {
    const instance1 = SyncManager.getInstance();
    const instance2 = SyncManager.getInstance();
    expect(instance1).toBe(instance2);
  });

  it('initializes with default properties', () => {
    const instance = SyncManager.getInstance();
    expect(instance).toBeDefined();
    // @ts-ignore - accessing private properties for testing
    expect(instance.syncInProgress).toBe(false);
  });
});
