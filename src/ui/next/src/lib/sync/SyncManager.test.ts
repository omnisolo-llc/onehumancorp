import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { SyncManager } from './SyncManager';
import { enqueueAction, getActions, removeAction } from '../../app/utils/offlineQueue';

vi.mock('../../app/utils/offlineQueue', () => ({
  enqueueAction: vi.fn().mockResolvedValue(undefined),
  getActions: vi.fn().mockResolvedValue([]),
  removeAction: vi.fn().mockResolvedValue(undefined),
}));

afterEach(() => {
  vi.clearAllTimers();
  vi.useRealTimers();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe('SyncManager', () => {
  beforeEach(() => {
    // Reset singleton instance between tests
    Reflect.set(SyncManager, 'instance', undefined);
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
    expect(instance).toHaveProperty('syncInProgress', false);
  });

  it('normalizes ISO queue timestamps and rejects invalid ones without persisting', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    const manager = SyncManager.getInstance();
    await manager.enqueue({ id: 'order-1', type: 'UPDATE_ORDER_STATUS', payload: { order_id: 'order-1' }, timestamp: '2026-09-19T10:00:00Z' });
    expect(enqueueAction).toHaveBeenCalledWith(expect.objectContaining({ timestamp: Date.parse('2026-09-19T10:00:00Z') }));
    vi.mocked(enqueueAction).mockClear();
    await expect(manager.enqueue({ id: 'bad', type: 'UPDATE_ORDER_STATUS', timestamp: 'not-a-date' })).rejects.toThrow('timestamp');
    expect(enqueueAction).not.toHaveBeenCalled();
  });

  it.each(['triage_action', 'advisory_action', 'field_ops_status', 'fulfillment_action', 'generate_invoice'])('retains %s when the server rate-limits the operation', async (type) => {
    vi.useFakeTimers();
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
    vi.spyOn(console, 'error').mockImplementation(() => {});
    vi.mocked(getActions).mockResolvedValue([{ id: 'pending', type, timestamp: 1, payload: { id: 'pending', order_id: 'order', action: 'approve' } }]);
    const fetchMock = vi.fn().mockResolvedValue(new Response('{}', { status: 429, headers: { 'Retry-After': '1' } }));
    vi.stubGlobal('fetch', fetchMock);
    await SyncManager.getInstance().sync();
    expect(fetchMock).toHaveBeenCalled();
    expect(removeAction).not.toHaveBeenCalled();
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
