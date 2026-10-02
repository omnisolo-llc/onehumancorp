import { describe, expect, it } from 'vitest';
import { planRoutes, readOutcome } from './queueRoutes';

const action = { id: 'payment-1', type: 'cash_sale', timestamp: 1, amount: 1500, currency: 'eur', device_signature: 'signed' };
const route = '/api/v1/payments/terminal/sync_offline';
describe('frozen route plans', () => {
  it('freezes financial terminal metadata and carries a complete action', () => {
    localStorage.setItem('omnisolo_pos_device_id', 'device-a');
    localStorage.setItem('omnisolo_active_terminal_session_id', 'session-a');
    const [plan] = planRoutes(action);
    localStorage.setItem('omnisolo_pos_device_id', 'device-b');
    expect(plan.id).toBe(route);
    expect(plan.body).toEqual({ session_id: 'session-a', transactions: [expect.objectContaining({ id: action.id, client_id: 'device-a', amount_cents: 1500, currency: 'eur', device_signature: 'signed' })] });
    expect(plan.maxAttempts).toBe(1);
  });
  it('freezes every legacy route and normalizes raw event IDs', () => {
    expect(planRoutes({ id: 'i', type: 'agent_intent', timestamp: 1, payload: { goal: 'saved' } }).map(plan => plan.id)).toEqual(['/api/v1/sync/operation-intents', '/api/v1/sync/offline']);
    expect(planRoutes({ id: 'e', type: 'sync_event', timestamp: 1, payload: { id: 'mismatch', entity_type: 'order' } })[0].body).toEqual({ events: [expect.objectContaining({ id: 'e', entity_type: 'order', base_version: 0 })] });
  });
});
describe('explicit per-ID outcomes', () => {
  it('acknowledges only the matching ID and route', () => {
    expect(readOutcome(action.id, route, 200, { outcomes: [{ id: action.id, route, status: 'acknowledged' }, { id: 'other', route, status: 'blocked' }] }).status).toBe('acknowledged');
  });
  it.each([
    { success: true, processed: 1 },
    { outcomes: [{ id: 'other', route, status: 'acknowledged' }] },
    { outcomes: [{ id: action.id, route: '/different', status: 'acknowledged' }] },
    { outcomes: [{ id: action.id, route, status: 'acknowledged' }, { id: action.id, route, status: 'blocked' }] },
    { success: false, outcomes: [{ id: action.id, route, status: 'acknowledged' }] },
    { pending_reconciliation: [action.id], outcomes: [{ id: action.id, route, status: 'acknowledged' }] },
  ])('holds count-only, foreign, duplicate or contradictory results', body => {
    expect(readOutcome(action.id, route, 200, body).status).toBe('reconciliation');
  });
  it.each(['blocked', 'reconciliation'] as const)('retains a definite %s outcome even when the aggregate success flag is false', status => {
    expect(readOutcome(action.id, route, 200, { success: false, outcomes: [{ id: action.id, route, status, reason: 'specific server reason' }] })).toEqual({ status, reason: 'specific server reason' });
  });
  it('blocks 4xx without accepting an acknowledgement body', () => {
    expect(readOutcome(action.id, route, 409, { outcomes: [{ id: action.id, route, status: 'acknowledged' }] }).status).toBe('blocked');
  });
  it('retains an explicit blocked or reconciliation outcome', () => {
    expect(readOutcome(action.id, route, 200, { outcomes: [{ id: action.id, route, status: 'blocked', reason: 'revoked' }] })).toEqual({ status: 'blocked', reason: 'revoked' });
  });
});
it('does not permit dot segments to change an intended route destination', () => {
  expect(() => planRoutes({ id: 'bad', type: 'approve_quote', quoteId: '..', timestamp: 1 })).toThrow('identifier');
});
it('preserves an observed entity version and timestamp instead of rebasing queued state', () => {
  const plan = planRoutes({ id: 'v', type: 'UPDATE_ORDER_STATUS', timestamp: 1, payload: { order_id: 'o', status: 'Ready', expected_status: 'Preparing', base_version: 7, expected_updated_at: '2026-09-30T10:00:00Z' } })[0];
  expect(plan.body).toEqual({ events: [expect.objectContaining({ base_version: 7, payload: expect.objectContaining({ expected_updated_at: '2026-09-30T10:00:00Z' }) })] });
});
