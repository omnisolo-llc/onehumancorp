import { describe, it, expect, vi, beforeEach } from 'vitest';
import { SyncManager } from './SyncManager';
import { MutationService } from './MutationService';
import * as offlineQueue from '../../app/utils/offlineQueue';

vi.mock('../../app/utils/offlineQueue', () => ({
  enqueueAction: vi.fn(),
  getActions: vi.fn(),
  removeAction: vi.fn()
}));

vi.mock('uuid', () => ({
  v4: () => 'fake-uuid'
}));

// Mock fetch
const originalFetch = global.fetch;
beforeEach(() => {
  vi.resetAllMocks();
  (offlineQueue.enqueueAction as any).mockResolvedValue();
  (offlineQueue.getActions as any).mockResolvedValue([]);
  (offlineQueue.removeAction as any).mockResolvedValue();

  global.fetch = vi.fn().mockResolvedValue({
    ok: true,
    status: 200,
    json: async () => ({})
  });

  // mock navigator online
  Object.defineProperty(navigator, 'onLine', {
     value: true,
     configurable: true
  });
});


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

  it('maps general mutations correctly', () => {
    const instance = SyncManager.getInstance();

    const inventoryAction = {
      id: 'e2e-product-cake-123',
      type: 'inventory_toggle',
      timestamp: 1690000000000
    };
    const mappedInventory = instance.mapGeneralMutation(inventoryAction);
    expect(mappedInventory.product_id).toBe('cake-123');
    expect(mappedInventory.quantity_deducted).toBe(1);
    expect(mappedInventory.transaction_id).toBe('e2e-product-cake-123');

    const agentAction = {
      id: 'agent-1',
      type: 'agent_intent',
      payload: { test: 'value' },
      timestamp: 1690000000000
    };
    const mappedAgent = instance.mapGeneralMutation(agentAction);
    expect(mappedAgent.mutation_type).toBe('agent_intent');
    expect(mappedAgent.payload).toBe('{"test":"value"}');

    const keepAction = {
      id: 'triage-1',
      type: 'triage_action',
      payload: { field: 1 }
    };
    const mappedKeep = instance.mapGeneralMutation(keepAction);
    expect(mappedKeep).toBe(keepAction); // returns the same object
  });
});

describe('Offline-First MutationService and SyncManager', () => {
  it('queues a mutation when offline and syncs when online', async () => {
    // 1. Simulate offline
    Object.defineProperty(navigator, 'onLine', {
       value: false,
       configurable: true
    });

    let optimisticCalled = false;
    let rollbackCalled = false;
    const optFn = () => { optimisticCalled = true; };
    const rollFn = () => { rollbackCalled = true; };

    const service = MutationService.getInstance();

    await service.executeMutation(
      'UPDATE_ORDER_STATUS',
      { order_id: '123', status: 'completed' },
      optFn,
      rollFn
    );

    expect(optimisticCalled).toBe(true);
    expect(rollbackCalled).toBe(false);
    expect(offlineQueue.enqueueAction).toHaveBeenCalledOnce();

    const queuedIntent = vi.mocked(offlineQueue.enqueueAction).mock.calls[0][0];
    expect(queuedIntent.type).toBe('UPDATE_ORDER_STATUS');
    expect(queuedIntent.payload.order_id).toBe('123');

    // fetch should not be called since we are offline
    expect(global.fetch).not.toHaveBeenCalled();

    // 2. Simulate coming back online
    Object.defineProperty(navigator, 'onLine', {
       value: true,
       configurable: true
    });

    const syncManager = SyncManager.getInstance();

    // Fake queue having the item
    (offlineQueue.getActions as any).mockResolvedValue([queuedIntent]);

    await syncManager.sync();

    // Expect fetch to have been called to flush the queue
    expect(global.fetch).toHaveBeenCalled();

    // Expect queue item to be removed after successful sync
    expect(offlineQueue.removeAction).toHaveBeenCalledWith(queuedIntent.id);
  });
});
