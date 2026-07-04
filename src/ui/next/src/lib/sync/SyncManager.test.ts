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
