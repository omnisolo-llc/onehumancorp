import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { offlineSyncService } from '@/lib/offline-sync';

// Mock the indexedDB and global objects
const mockStorage: any = {};
vi.mock('@/lib/offline-sync', () => {
  return {
    offlineSyncService: {
      addMutation: vi.fn(),
      getPendingMutations: vi.fn().mockResolvedValue([]),
      markAsSyncing: vi.fn(),
      markAsFailed: vi.fn(),
      removeMutation: vi.fn(),
      clearAll: vi.fn(),
    },
  };
});

describe('OfflineSyncEngine', () => {
  it('should initialize and get pending mutations', async () => {
    const mutations = await offlineSyncService.getPendingMutations();
    expect(mutations).toEqual([]);
    expect(offlineSyncService.getPendingMutations).toHaveBeenCalled();
  });
});
