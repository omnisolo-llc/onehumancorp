import type { OfflineAction } from '../../app/utils/offlineQueue';
import { isQuoteVersion } from '../quoteVersion';
import { recordOrEmpty } from '../records';

export const STAFF_CLOCK_TYPE = 'staff_clock_event_v1';
export const TIMECARD_ROUTE = '/api/v1/staff/timecard';
export function isSafeClockId(value: unknown): value is string { return typeof value === 'string' && /^[A-Za-z0-9_-]{1,128}$/.test(value); }
export function isHistoricalClock(action: OfflineAction): boolean { return action.type === 'CLOCK_IN' || action.type === 'CLOCK_OUT'; }

export type OutcomeStatus = 'acknowledged' | 'blocked' | 'reconciliation';
export type RouteContext = { terminalId?: string; sessionId?: string };
export type RoutePlan = { id: string; method: 'POST' | 'PATCH' | 'PUT'; body?: unknown; maxAttempts: number };
export type RouteOutcome = { status: OutcomeStatus; reason?: string };

export function mapGeneralMutation(action: OfflineAction) {
  const common = { timestamp: new Date(action.timestamp).toISOString(), transaction_id: action.id, quantity_deducted: 0, amount: null, payment_method: null, payment_intent_id: null, currency: 'usd' };
  if (action.type === 'inventory_toggle') return { ...common, product_id: action.id.replace('e2e-product-', ''), quantity_deducted: 1, currency: null };
  if (action.type === 'draft_quote') return { ...common, product_id: 'draft_quote', mutation_type: 'draft_quote', payload: action.notes };
  if (action.type === 'agent_intent') return { ...common, product_id: 'agent_intent', mutation_type: 'agent_intent', payload: JSON.stringify(action.payload) };
  return action;
}

export function captureRouteContext(action: OfflineAction): RouteContext {
  if (action.type !== 'cash_sale' && action.type !== 'tap_to_pay') return {};
  return { terminalId: localStorage.getItem('omnisolo_pos_device_id') || 'terminal_client', sessionId: localStorage.getItem('omnisolo_active_terminal_session_id') || undefined };
}
function pathId(value: unknown): string {
  const id = String(value ?? '');
  if (!id || id === '.' || id === '..') throw new Error('Invalid offline route identifier');
  return encodeURIComponent(id);
}
function clockTimestamp(value: number): string {
  if (!Number.isSafeInteger(value) || !Number.isFinite(new Date(value).getTime())) throw new Error('Invalid clock timestamp');
  const timestamp = new Date(value).toISOString();
  // The timecard endpoint accepts four-digit RFC3339 years. Never silently
  // truncate fractions or freeze an expanded-year body it cannot persist.
  if (!/^[0-9]{4}-/.test(timestamp)) throw new Error('Invalid clock timestamp');
  return timestamp;
}
/** All URLs, payloads and terminal metadata are captured before the first send. */
export function planRoutes(action: OfflineAction, context = captureRouteContext(action)): RoutePlan[] {
  const payload = recordOrEmpty(action.payload);
  const timestamp = action.type === STAFF_CLOCK_TYPE ? clockTimestamp(action.timestamp) : new Date(action.timestamp).toISOString();
  const post = (id: string, body?: unknown): RoutePlan => ({ id, method: 'POST', body, maxAttempts: 1 });
  if (action.type === STAFF_CLOCK_TYPE) {
    if (!isSafeClockId(action.id) || typeof payload.staff_id !== 'string' || !payload.staff_id.trim()
      || typeof payload.event_type !== 'string' || !['CLOCK_IN', 'CLOCK_OUT'].includes(payload.event_type)
      || Object.keys(payload).some(key => key !== 'staff_id' && key !== 'event_type')) throw new Error('Invalid clock event');
    return [post(TIMECARD_ROUTE, { events: [{ id: action.id, staff_id: payload.staff_id, event_type: payload.event_type, offline_timestamp: timestamp }] })];
  }
  if (action.type === 'cash_sale' || action.type === 'tap_to_pay') {
    const device = context.terminalId;
    if (!device) throw new Error('Missing frozen terminal identifier');
    return [post('/api/v1/payments/terminal/sync_offline', {
      session_id: context.sessionId,
      transactions: [{ id: action.id, client_id: device, amount_cents: Math.round(action.payload?.amount_cents ?? action.amount ?? 0), currency: action.currency || 'usd',
        payload: JSON.stringify(action.payload ?? [{ product_id: action.product_id, quantity: action.quantity ?? 1 }]), timestamp,
        device_signature: action.device_signature || `sig_offline_${device}_${action.id}`, mutation_type: action.type, terminal_id: device }],
    })];
  }
  if (action.type === 'UPDATE_ORDER_STATUS' || action.type === 'TOGGLE_SOLD_OUT') {
    const order = action.type === 'UPDATE_ORDER_STATUS';
    return [post('/api/v1/sync/events', { events: [{ id: action.id, entity_type: order ? 'order' : 'product', entity_id: order ? payload.order_id : payload.item_id, action_type: order ? 'UpdateStatus' : 'ToggleSoldOut', payload, base_version: payload.base_version ?? payload.version ?? 0, timestamp }] })];
  }
  if (action.type === 'sync_event') return [post('/api/v1/sync/events', { events: [{ ...payload, id: action.id, base_version: payload.base_version ?? payload.version ?? 0, timestamp }] })];
  if (action.type === 'CRDT_MUTATION') return [post('/api/v1/sync/mcp-deltas', { deltas: [{ id: action.id, entity_id: payload.entity_id || 'unknown', data: typeof payload.data === 'string' ? payload.data : JSON.stringify(payload.data ?? {}), updated_at: timestamp }] })];
  if (action.type === 'update_quote' || action.type === 'approve_quote') {
    const id = pathId(action.quoteId);
    if (!isQuoteVersion(payload.expected_updated_at)) throw new Error('Missing reviewed quote version');
    // Freeze the observed token. Old/versionless envelopes remain held rather
    // than being rebased to a later quote when connectivity returns.
    return action.type === 'update_quote' ? [post(`/api/v1/quotes?id=${id}`, action.payload)]
      : [{ id: `/api/v1/quotes/${id}/approve`, method: 'PATCH', body: { expected_updated_at: payload.expected_updated_at }, maxAttempts: 1 }];
  }
  if (action.type === 'approve_agent_feed') {
    const id = String(payload.id ?? '');
    if (payload.event_source === 'review') return [post('/api/v1/reviews/action', { action: payload.approved ? 'approve' : 'dismiss', responseId: id, content: payload.modified_content })];
    if (['triage', 'task', 'order'].includes(String(payload.event_source))) return [post('/api/v1/triage/action', { triage_item_id: id, approved: payload.approved, edited_payload: payload.modified_content })];
    return [{ id: `/api/v1/agent-feed/${pathId(id)}`, method: 'PUT', body: { state: payload.approved ? 'APPROVED' : 'DISMISSED', modified_content: payload.modified_content }, maxAttempts: 1 }];
  }
  if (action.type === 'triage_action') return [post('/api/v1/ui/triage/action', action.payload)];
  if (action.type === 'advisory_action') return [post(`/api/v1/agents/approvals/${pathId(payload.id)}`, { approved: payload.approved })];
  if (action.type === 'field_ops_status') return [post('/api/v1/field-ops/appointments', action.payload)];
  if (action.type === 'fulfillment_action') return [post(`/api/v1/fulfillment/execute/${pathId(payload.id)}`, { action: payload.action })];
  if (action.type === 'generate_invoice') return [post('/api/v1/invoices/generate', action.payload)];
  return [
    post('/api/v1/sync/operation-intents', { intents: [{ id: action.id, action_type: action.type, payload: action.payload, timestamp }] }),
    post('/api/v1/sync/offline', { mutations: [mapGeneralMutation(action)] }),
  ];
}

/** Transport success/counts cannot prove a particular effect was committed. */
export function readOutcome(id: string, route: string, httpStatus: number, body: unknown): RouteOutcome {
  if (httpStatus >= 400 && httpStatus < 500) return { status: 'blocked', reason: `HTTP ${httpStatus}` };
  if (httpStatus < 200 || httpStatus >= 300) return { status: 'reconciliation', reason: `HTTP ${httpStatus}` };
  const result = recordOrEmpty(body);
  const outcomes = Array.isArray(result.outcomes) ? result.outcomes.map(recordOrEmpty) : [];
  const matching = outcomes.filter(outcome => outcome.id === id && outcome.route === route);
  const reconciliation = Array.isArray(result.pending_reconciliation) ? result.pending_reconciliation : [];
  if (matching.length !== 1 || (result.success === false && matching[0].status === 'acknowledged') || result.error != null || reconciliation.some(item => item === id || recordOrEmpty(item).id === id)) {
    return { status: 'reconciliation', reason: 'Missing or contradictory per-action acknowledgement' };
  }
  const outcome = matching[0];
  if (!['acknowledged', 'blocked', 'reconciliation'].includes(String(outcome.status))) return { status: 'reconciliation', reason: 'Unknown outcome status' };
  return { status: outcome.status as OutcomeStatus, ...(typeof outcome.reason === 'string' ? { reason: outcome.reason } : {}) };
}

export function validRoutePlan(value: unknown): value is RoutePlan {
  const plan = recordOrEmpty(value);
  if (typeof plan.id !== 'string' || !['POST', 'PUT', 'PATCH'].includes(String(plan.method)) || plan.maxAttempts !== 1) return false;
  if (plan.id === TIMECARD_ROUTE) return plan.method === 'POST';
  const exact = new Set(['/api/v1/payments/terminal/sync_offline', '/api/v1/sync/events', '/api/v1/sync/mcp-deltas', '/api/v1/sync/operation-intents', '/api/v1/sync/offline', '/api/v1/ui/triage/action', '/api/v1/triage/action', '/api/v1/reviews/action', '/api/v1/field-ops/appointments', '/api/v1/invoices/generate']);
  return exact.has(plan.id) || /^\/api\/v1\/quotes\?id=[A-Za-z0-9%_.!~*'()-]*$/.test(plan.id) || /^\/api\/v1\/(?:quotes\/[A-Za-z0-9%_.!~*'()-]*\/approve|agents\/approvals\/[A-Za-z0-9%_.!~*'()-]*|fulfillment\/execute\/[A-Za-z0-9%_.!~*'()-]*|agent-feed\/[A-Za-z0-9%_.!~*'()-]*)$/.test(plan.id);
}
