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

it.each(['update_quote', 'approve_quote'])('freezes the exact observed quote version for %s', type => {
  const token = '2026-10-04T00:00:00.123456+00:00';
  const plan = planRoutes({ id: 'versioned', type, quoteId: 'quote', timestamp: 1, payload: { expected_updated_at: token, line_items: [] } })[0];
  expect(plan.body).toMatchObject({ expected_updated_at: token });
});
it.each(['update_quote', 'approve_quote'])('refuses to dispatch %s without a reviewed version', type => {
  expect(() => planRoutes({ id: 'unreviewed', type, quoteId: 'quote', timestamp: 1, payload: {} })).toThrow('reviewed quote version');
});

const clock = { id: 'clock-safe_1', type: 'staff_clock_event_v1', timestamp: 1, payload: { staff_id: 'a', event_type: 'CLOCK_IN' } };
it('plans each new clock event as one exact timecard mutation using only the action timestamp', () => {
  expect(planRoutes(clock)).toEqual([{ id: '/api/v1/staff/timecard', method: 'POST', maxAttempts: 1,
    body: { events: [{ id: clock.id, staff_id: 'a', event_type: 'CLOCK_IN', offline_timestamp: '1970-01-01T00:00:00.001Z' }] } }]);
});
it.each(['', '.', '..', 'a/b', 'a%2Fb', 'a b', 'é', 'a'.repeat(129)])('rejects an unsafe clock receipt identifier %j', id => {
  expect(() => planRoutes({ ...clock, id })).toThrow(/clock/i);
});
it.each([{}, { staff_id: '', event_type: 'CLOCK_IN' }, { staff_id: 'a', event_type: 'BREAK' }, { staff_id: 4, event_type: 'CLOCK_OUT' }, { staff_id: 'a', event_type: 'CLOCK_IN', timestamp: '2000-01-01T00:00:00Z' }])('rejects incomplete or ambiguous clock payload %j', payload => {
  expect(() => planRoutes({ ...clock, payload })).toThrow(/clock/i);
});
it.each(['CLOCK_IN', 'CLOCK_OUT'])('preserves the historical %s plan without transforming its envelope', type => {
  const old = { id: 'old', type, timestamp: 1, payload: { staff_id: 'a', timestamp: '1970-01-01T00:00:00.001Z' } };
  expect(planRoutes(old)).toEqual([
    { id: '/api/v1/sync/operation-intents', method: 'POST', maxAttempts: 1, body: { intents: [{ id: 'old', action_type: type, payload: old.payload, timestamp: '1970-01-01T00:00:00.001Z' }] } },
    { id: '/api/v1/sync/offline', method: 'POST', maxAttempts: 1, body: { mutations: [old] } },
  ]);
});
it('whitelists only the exact timecard POST route', async () => {
  const { validRoutePlan } = await import('./queueRoutes');
  expect(validRoutePlan(planRoutes(clock)[0])).toBe(true);
  for (const id of ['/api/v1/staff/timecard/other', '/api/v1/staff/timecard?x=1', '/api/v1/staff/timecard/receipts/clock-safe_1']) {
    expect(validRoutePlan({ id, method: 'POST', maxAttempts: 1 })).toBe(false);
  }
  expect(validRoutePlan({ id: '/api/v1/staff/timecard', method: 'PUT', maxAttempts: 1 })).toBe(false);
});

it.each(['1970-01-01T00:00:00.001Z', NaN, Infinity, 1.25, Number.MAX_SAFE_INTEGER, 8_640_000_000_000_001, Date.UTC(10000, 0, 1), Date.UTC(-1, 0, 1)])('rejects corrupt new-clock millisecond timestamps without coercion: %s', timestamp => {
  expect(() => planRoutes({ ...clock, timestamp: timestamp as number })).toThrow(/clock timestamp/i);
});
