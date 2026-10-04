import { queueTransaction, readOtherAdapter, selectedQueueAdapter, type QueueAdapter, type StoredQueueRow } from '../../lib/sync/queueStorage';
import { readQueueOwner, sameOwner, currentVerifiedQueueOwner, queueIdentityGeneration, QUEUE_IDENTITY_EPOCH_KEY, type QueueOwner } from '../../lib/sync/queueIdentity';
import { captureRouteContext, planRoutes, validRoutePlan, isHistoricalClock, STAFF_CLOCK_TYPE, TIMECARD_ROUTE, readOutcome, type RouteContext, type RoutePlan, type OutcomeStatus } from '../../lib/sync/queueRoutes';
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
  /** Frozen local handoff context. Never inferred from a later appointment snapshot. */
  field_completion?: { job_id: string; customer_id: string; notes: string };
  field_completion_parent_id?: string;
}

export type RouteState = { plan: RoutePlan; status: 'pending' | 'inflight' | OutcomeStatus; attempts: number; attemptToken?: string; reason?: string };
type Envelope = { version: 2; adapter: QueueAdapter; owner: QueueOwner; action: OfflineAction; context: RouteContext; routes: RouteState[] };
export type ActionClaim = { action: OfflineAction; owner: QueueOwner; adapter: QueueAdapter; route: RoutePlan; attemptToken: string };
export type QueueSummary = { pending: number; needsAttention: number; reconciliation: number; legacyHeld: number; storageUnavailable: boolean };

/** Only the acknowledged appointment event can release its captured follow-ups. */
function fieldCompletionFollowups(action: OfflineAction): OfflineAction[] {
  const context = action.field_completion;
  if (context === undefined) return [];
  const event = action.payload;
  const payload = event?.payload;
  if (!context || typeof context !== 'object' || Array.isArray(context) || action.type !== 'sync_event'
    || event?.entity_type !== 'appointment' || event?.action_type !== 'UpdateStatus'
    || typeof context.job_id !== 'string' || !context.job_id || context.job_id !== event.entity_id
    || typeof context.customer_id !== 'string' || !context.customer_id || typeof context.notes !== 'string'
    || !payload || typeof payload !== 'object' || Array.isArray(payload)
    || (payload as Record<string, unknown>).status !== 'Completed'
    || typeof (payload as Record<string, unknown>).expected_updated_at !== 'string'
    || !(payload as Record<string, unknown>).expected_updated_at
    || ((payload as Record<string, unknown>).notes ?? '') !== context.notes) {
    throw new Error('Invalid captured field completion handoff');
  }
  const common = { timestamp: action.timestamp, field_completion_parent_id: action.id };
  const quoteNotes = `Follow up quote requested by field op for job ${context.job_id}. Notes: ${context.notes}`;
  return [
    { ...common, id: `field-completion-${action.id}-invoice`, type: 'generate_invoice', payload: { job_id: context.job_id, customer_id: context.customer_id } },
    ...(context.notes ? [{ ...common, id: `field-completion-${action.id}-quote`, type: 'draft_quote', notes: quoteNotes, payload: { notes: quoteNotes } }] : []),
  ];
}

function envelope(row: StoredQueueRow): Envelope | null {
  try {
    const data = typeof row.payload === 'string' ? JSON.parse(row.payload) : row.payload;
    if (data?.version !== 2 || !['powersync', 'indexeddb'].includes(data.adapter) ||
        typeof data.owner?.userId !== 'string' || typeof data.owner?.tenantId !== 'string' ||
        data.action?.id !== row.id || !Array.isArray(data.routes) || !data.routes.length ||
        data.routes.some((route: RouteState) => !route || !validRoutePlan(route.plan) || !['pending', 'inflight', 'acknowledged', 'blocked', 'reconciliation'].includes(route.status) || !Number.isInteger(route.attempts) || route.attempts < 0 || route.attempts > route.plan.maxAttempts)) return null;
    if (!data.context || typeof data.context !== 'object' || Array.isArray(data.context) ||
        canonical(data.routes.map((route: RouteState) => route.plan)) !== canonical(planRoutes(data.action, data.context))) return null;
    fieldCompletionFollowups(data.action);
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

export async function enqueueAction(action: OfflineAction, expectedOwner?: QueueOwner): Promise<void> { return enqueueActions([action], expectedOwner); }
export async function enqueueActions(actions: OfflineAction[], expectedOwner?: QueueOwner): Promise<void> {
  if (!actions.length) return;
  const intendedOwner = expectedOwner ? { ...expectedOwner } : undefined;
  // Clone before the first await: the caller cannot change a queued financial action.
  const immutable = clone(actions).map(action => { fieldCompletionFollowups(action); const context = captureRouteContext(action); return { action, context, plans: planRoutes(action, context) }; });
  const owner = await readQueueOwner();
  if (intendedOwner && !sameOwner(owner, intendedOwner)) throw new Error('Queued action owner does not match the current view.');
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

async function ownedEnvelopes(includeAcknowledged = false): Promise<{ values: Envelope[]; summary: QueueSummary }> {
  const owner = await readQueueOwner();
  const adapter = await selectedQueueAdapter();
  const other = await readOtherAdapter(adapter);
  checkOtherRows(other.rows);
  const rows = await queueTransaction(adapter, rows => ({ result: rows }));
  const values = rows.map(envelope).filter((value): value is Envelope => value !== null && sameOwner(value.owner, owner) && value.adapter === adapter && (includeAcknowledged || !acknowledged(value)));
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
    if (!value || value.adapter !== adapter || !sameOwner(value.owner, owner) || isHistoricalClock(value.action)) return { result: null };
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
    const writes = [stored(value)];
    if (acknowledged(value)) {
      // Parent receipt and children share one commit. A crash cannot leave an
      // acknowledged completion without its follow-ups, or release them early.
      for (const action of fieldCompletionFollowups(value.action)) {
        const previous = rows.find(row => row.id === action.id);
        const existing = previous && envelope(previous);
        if (previous && (!existing || !sameOwner(existing.owner, value.owner) || canonical(existing.action) !== canonical(action))) throw new Error('Field completion follow-up ID collision requires reconciliation');
        if (!previous) {
          const context = captureRouteContext(action);
          writes.push(stored({ version: 2, adapter: value.adapter, owner: value.owner, action, context,
            routes: planRoutes(action, context).map(plan => ({ plan, status: 'pending', attempts: 0 })) }));
        }
      }
    }
    return { writes, result: undefined };
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


export type ClockQueueSummary = { confirmed: number; unconfirmed: number; legacyHeld: number };
export async function getClockQueueSummary(): Promise<ClockQueueSummary> {
  const { values } = await ownedEnvelopes(true);
  return {
    confirmed: values.filter(value => value.action.type === STAFF_CLOCK_TYPE && acknowledged(value)).length,
    unconfirmed: values.filter(value => value.action.type === STAFF_CLOCK_TYPE && !acknowledged(value)).length,
    legacyHeld: values.filter(value => isHistoricalClock(value.action)).length,
  };
}

export type ClockReceiptCandidate = Readonly<{ action: OfflineAction; owner: QueueOwner; adapter: QueueAdapter; route: RouteState }>;
type IdentityFence = { generation: number; storageEpoch: string | null };
const clockReceiptSnapshots = new WeakMap<ClockReceiptCandidate, { value: Envelope; fence: IdentityFence }>();
function identityFence(): IdentityFence { return { generation: queueIdentityGeneration(), storageEpoch: localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) }; }
function sameFence(fence: IdentityFence): boolean {
  try { return fence.generation === queueIdentityGeneration() && fence.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY); }
  catch { return false; }
}
function freeze<T>(value: T): T {
  if (value && typeof value === 'object') { for (const nested of Object.values(value)) freeze(nested); Object.freeze(value); }
  return value;
}
function attemptedClock(value: Envelope): boolean {
  const route = value.routes[0];
  return value.action.type === STAFF_CLOCK_TYPE && value.routes.length === 1 && route.plan.id === TIMECARD_ROUTE
    && route.attempts === 1 && typeof route.attemptToken === 'string' && !!route.attemptToken
    && ['inflight', 'blocked', 'reconciliation'].includes(route.status);
}
/** Fresh copied snapshots only; no claim, rewrite, migration or retry is performed. */
export async function getClockReceiptCandidates(): Promise<ClockReceiptCandidate[]> {
  const fence = identityFence();
  const { values } = await ownedEnvelopes();
  const owner = currentVerifiedQueueOwner();
  if (!sameFence(fence) || !owner) return [];
  return values.filter(value => sameOwner(owner, value.owner) && attemptedClock(value)).map(value => {
    const candidate = freeze(clone({ action: value.action, owner: value.owner, adapter: value.adapter, route: value.routes[0] }));
    clockReceiptSnapshots.set(candidate, { value: clone(value), fence });
    return candidate;
  });
}
/** Fence receipt access and the original-row CAS; this is not commit-wide cancellation. */
export function isCurrentClockReceiptCandidate(candidate: ClockReceiptCandidate): boolean {
  const snapshot = clockReceiptSnapshots.get(candidate);
  const current = currentVerifiedQueueOwner();
  return !!snapshot && sameFence(snapshot.fence) && !!current && sameOwner(current, snapshot.value.owner);
}
/** Exact receipt plus compare-and-set of the original envelope; never resets an attempt. */
export async function acknowledgeClockReceipt(candidate: ClockReceiptCandidate, httpStatus: number, body: unknown): Promise<boolean> {
  const snapshot = clockReceiptSnapshots.get(candidate);
  if (!snapshot || !isCurrentClockReceiptCandidate(candidate) || !attemptedClock(snapshot.value)) return false;
  const { action, owner } = snapshot.value;
  const result = body && typeof body === 'object' && !Array.isArray(body) ? body as Record<string, unknown> : {};
  const expected = { version: 1, actor_id: owner.userId, id: action.id, staff_id: action.payload?.staff_id,
    event_type: action.payload?.event_type, offline_timestamp: new Date(action.timestamp).toISOString() };
  if (httpStatus !== 200 || result.success !== true || readOutcome(action.id, TIMECARD_ROUTE, httpStatus, result).status !== 'acknowledged'
    || canonical(result.receipt) !== canonical(expected)) return false;
  if (!sameOwner(owner, await readQueueOwner()) || !isCurrentClockReceiptCandidate(candidate)) return false;
  const adapter = await selectedQueueAdapter();
  if (adapter !== snapshot.value.adapter) return false;
  checkOtherRows((await readOtherAdapter(adapter)).rows);
  return queueTransaction(adapter, rows => {
    const row = rows.find(row => row.id === action.id);
    const current = row && envelope(row);
    if (!isCurrentClockReceiptCandidate(candidate) || !current || canonical(current) !== canonical(snapshot.value)) return { result: false };
    // This is the local acknowledgment linearization point. The exact server
    // effect already exists. As with completeAction, a later logout may allow
    // this original-owner commit to finish, but cannot acknowledge a replacement
    // envelope, mutate another account, or authorize a server write/replay.
    current.routes[0].status = 'acknowledged'; delete current.routes[0].reason;
    return { writes: [stored(current)], result: true };
  });
}
