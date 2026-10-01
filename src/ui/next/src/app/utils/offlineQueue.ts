import { queueTransaction, readOtherAdapter, selectedQueueAdapter, type QueueAdapter, type StoredQueueRow } from '../../lib/sync/queueStorage';
import { readQueueOwner, sameOwner, type QueueOwner } from '../../lib/sync/queueIdentity';
import { captureRouteContext, planRoutes, validRoutePlan, type RouteContext, type RoutePlan, type OutcomeStatus } from '../../lib/sync/queueRoutes';
/** Payload fields consumed by the existing queue adapters. Unknown extension
 * fields remain opaque JSON and cannot be read without narrowing. */
export interface MutationPayload {
  [key: string]: unknown;
  id?: string;
  amount_cents?: number;
  order_id?: string;
  item_id?: string;
  entity_id?: string;
  data?: unknown;
  approved?: boolean;
  modified_content?: string;
  event_source?: string;
  action?: string;
}

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload?: MutationPayload;
  url?: string;
  notes?: string;
  quoteId?: string;
  amount?: number;
  currency?: string;
  product_id?: string;
  quantity?: number;
  device_signature?: string;
  timestamp: number;
}

export type RouteState = { plan: RoutePlan; status: 'pending' | 'inflight' | OutcomeStatus; attempts: number; attemptToken?: string; reason?: string };
type Envelope = { version: 2; adapter: QueueAdapter; owner: QueueOwner; action: OfflineAction; context: RouteContext; routes: RouteState[] };
export type ActionClaim = { action: OfflineAction; owner: QueueOwner; adapter: QueueAdapter; route: RoutePlan; attemptToken: string };
export type QueueSummary = { pending: number; needsAttention: number; reconciliation: number; legacyHeld: number; storageUnavailable: boolean };

function envelope(row: StoredQueueRow): Envelope | null {
  try {
    const data = typeof row.payload === 'string' ? JSON.parse(row.payload) : row.payload;
    if (data?.version !== 2 || !['powersync', 'indexeddb'].includes(data.adapter) ||
        typeof data.owner?.userId !== 'string' || typeof data.owner?.tenantId !== 'string' ||
        data.action?.id !== row.id || !Array.isArray(data.routes) || !data.routes.length ||
        data.routes.some((route: RouteState) => !route || !validRoutePlan(route.plan) || !['pending', 'inflight', 'acknowledged', 'blocked', 'reconciliation'].includes(route.status) || !Number.isInteger(route.attempts) || route.attempts < 0 || route.attempts > route.plan.maxAttempts)) return null;
    if (!data.context || typeof data.context !== 'object' || Array.isArray(data.context) ||
        canonical(data.routes.map((route: RouteState) => route.plan)) !== canonical(planRoutes(data.action, data.context))) return null;
    return data as Envelope;
  } catch { return null; }
}
function stored(value: Envelope): StoredQueueRow {
  return { id: value.action.id, type: value.action.type, timestamp: value.action.timestamp, payload: JSON.stringify(value) };
}
function acknowledged(value: Envelope): boolean { return value.routes.every(route => route.status === 'acknowledged'); }
function clone<T>(value: T): T { return JSON.parse(JSON.stringify(value)) as T; }
function canonical(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.entries(value).filter(([, nested]) => nested !== undefined).sort(([a], [b]) => a.localeCompare(b)).map(([key, nested]) => `${JSON.stringify(key)}:${canonical(nested)}`).join(',')}}`;
  return JSON.stringify(value);
}
function checkOtherRows(rows: StoredQueueRow[]): void {
  if (rows.some(row => envelope(row))) throw new Error('Conflicting queue adapters require reconciliation');
}

export async function enqueueAction(action: OfflineAction): Promise<void> { return enqueueActions([action]); }
export async function enqueueActions(actions: OfflineAction[]): Promise<void> {
  if (!actions.length) return;
  // Clone before the first await: the caller cannot change a queued financial action.
  const immutable = clone(actions).map(action => { const context = captureRouteContext(action); return { action, context, plans: planRoutes(action, context) }; });
  const owner = await readQueueOwner();
  const adapter = await selectedQueueAdapter();
  const other = await readOtherAdapter(adapter);
  checkOtherRows(other.rows);
  const newEnvelopes = immutable.map(({ action, context, plans }) => {
    if (!action.id || !action.type || !Number.isFinite(action.timestamp)) throw new Error('Invalid offline action');
    return { version: 2 as const, adapter, owner, action, context, routes: plans.map(plan => ({ plan, status: 'pending' as const, attempts: 0 })) };
  });
  await queueTransaction(adapter, rows => {
    const byId = new Map(rows.map(row => [row.id, row]));
    const writes: StoredQueueRow[] = [];
    for (const next of newEnvelopes) {
      const previous = byId.get(next.action.id);
      const existing = previous && envelope(previous);
      if (other.rows.some(row => row.id === next.action.id) || previous && (!existing || !sameOwner(existing.owner, owner) || canonical(existing.action) !== canonical(next.action))) {
        throw new Error('Immutable offline action ID collision requires reconciliation');
      }
      if (!previous) { const row = stored(next); writes.push(row); byId.set(row.id, row); }
    }
    return { writes, result: undefined };
  });
}

async function ownedEnvelopes(): Promise<{ values: Envelope[]; summary: QueueSummary }> {
  const owner = await readQueueOwner();
  const adapter = await selectedQueueAdapter();
  const other = await readOtherAdapter(adapter);
  checkOtherRows(other.rows);
  const rows = await queueTransaction(adapter, rows => ({ result: rows }));
  const values = rows.map(envelope).filter((value): value is Envelope => value !== null && sameOwner(value.owner, owner) && value.adapter === adapter && !acknowledged(value));
  return { values, summary: {
    pending: values.filter(value => value.routes.some(route => route.status === 'pending')).length,
    needsAttention: values.filter(value => value.routes.some(route => route.status === 'blocked')).length,
    reconciliation: values.filter(value => value.routes.some(route => route.status === 'reconciliation' || route.status === 'inflight')).length,
    legacyHeld: [...rows, ...other.rows].filter(row => !envelope(row)).length,
    storageUnavailable: other.unavailable,
  } };
}
export async function getActions(): Promise<OfflineAction[]> { return (await ownedEnvelopes()).values.map(value => clone(value.action)); }
export async function getQueueSummary(): Promise<QueueSummary> { return (await ownedEnvelopes()).summary; }
export async function getActionRoutes(id: string): Promise<RoutePlan[]> {
  return (await ownedEnvelopes()).values.find(value => value.action.id === id)?.routes.filter(route => route.status === 'pending').map(route => clone(route.plan)) ?? [];
}

/** No lease expiry: interrupted attempts are reconciliation, never automatic replay. */
export async function claimAction(id: string, routeId: string): Promise<ActionClaim | null> {
  const owner = await readQueueOwner();
  const adapter = await selectedQueueAdapter();
  checkOtherRows((await readOtherAdapter(adapter)).rows);
  return queueTransaction(adapter, rows => {
    const row = rows.find(row => row.id === id);
    const value = row && envelope(row);
    if (!value || value.adapter !== adapter || !sameOwner(value.owner, owner)) return { result: null };
    const route = value.routes.find(route => route.plan.id === routeId);
    if (!route || route.status !== 'pending' || route.attempts !== 0) return { result: null };
    route.status = 'inflight'; route.attempts += 1; route.attemptToken = crypto.randomUUID();
    return { writes: [stored(value)], result: { action: clone(value.action), owner: clone(owner), adapter, route: clone(route.plan), attemptToken: route.attemptToken } };
  });
}

/** Complete the original owner's row even if the login changed while the request ran. */
export async function completeAction(claim: ActionClaim, status: OutcomeStatus, reason?: string): Promise<void> {
  await queueTransaction(claim.adapter, rows => {
    const row = rows.find(row => row.id === claim.action.id);
    const value = row && envelope(row);
    const route = value?.routes.find(route => route.plan.id === claim.route.id);
    if (!value || value.adapter !== claim.adapter || !sameOwner(value.owner, claim.owner) || !route || route.status !== 'inflight' || route.attemptToken !== claim.attemptToken) throw new Error('Offline claim no longer matches');
    route.status = status; route.reason = reason;
    return { writes: [stored(value)], result: undefined };
  });
}

/** Retain acknowledged tombstones so cleanup cannot resurrect or overwrite an ID. */
export async function removeAction(id: string): Promise<void> {
  const owner = await readQueueOwner();
  const adapter = await selectedQueueAdapter();
  await queueTransaction(adapter, rows => {
    const row = rows.find(row => row.id === id);
    const value = row && envelope(row);
    if (row && (!value || !sameOwner(value.owner, owner) || !acknowledged(value))) throw new Error('Unacknowledged offline actions cannot be removed');
    return { result: undefined };
  });
}
