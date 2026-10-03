import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { enqueueAction, claimAction, completeAction, getActions, getQueueSummary, type OfflineAction } from '@/app/utils/offlineQueue';
import { SyncManager } from './SyncManager';
import { readQueueOwner } from './queueIdentity';
import { getPowerSyncDB } from '../powersync/db';

// Only I/O boundaries are fixtures: the queue envelopes, plans, claims, receipts,
// follow-up transaction, and new-manager recovery all use production code.
vi.mock('../powersync/db', () => ({ getPowerSyncDB: vi.fn() }));
vi.mock('./queueIdentity', async original => ({ ...await original<object>(), readQueueOwner: vi.fn() }));
const owner = { userId: 'field-owner', tenantId: 'field-tenant' };
const completion: OfflineAction = {
  id: 'offline-completion', type: 'sync_event', timestamp: 100,
  payload: { entity_type: 'appointment', entity_id: 'job-1', action_type: 'UpdateStatus', base_version: 0,
    payload: { status: 'Completed', expected_status: 'In-Progress', expected_updated_at: '2026-10-03T12:00:00Z', notes: 'Requested estimate' } },
  field_completion: { job_id: 'job-1', customer_id: 'customer-1', notes: 'Requested estimate' },
};
let rows: Map<string, Record<string, unknown>>;
let tail: Promise<unknown>;
beforeEach(() => {
  vi.clearAllMocks(); localStorage.clear(); rows = new Map(); tail = Promise.resolve();
  Reflect.set(SyncManager, 'instance', undefined);
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
  vi.mocked(readQueueOwner).mockResolvedValue(owner);
  vi.spyOn(navigator.locks, 'request').mockImplementation((async (_name: string, work: () => Promise<unknown>) => work()) as never);
  const transaction = async (work: (tx: object) => Promise<unknown>) => {
    const run = tail.then(async () => {
      const staged = new Map(rows);
      const result = await work({ getAll: async () => [...staged.values()], execute: async (_sql: string, args: unknown[]) => { staged.set(args[0] as string, { id: args[0], type: args[1], payload: args[2], timestamp: args[3] }); } });
      rows = staged; return result;
    });
    tail = run.catch(() => undefined); return run;
  };
  vi.mocked(getPowerSyncDB).mockResolvedValue({ getAll: async () => [...rows.values()], writeTransaction: transaction } as never);
  vi.stubGlobal('fetch', vi.fn(async (url: string, options?: RequestInit) => Response.json({ outcomes: [{ id: new Headers(options?.headers).get('Idempotency-Key'), route: url, status: 'acknowledged' }] })));
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
async function acknowledgedCompletion() {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  await completeAction(claim, 'acknowledged');
}
it('fresh manager drains committed children after hard navigation without a jobs component or parent replay', async () => {
  await acknowledgedCompletion();
  expect((await getQueueSummary()).pending).toBe(2);
  // A new manager has no view state or captured closure from before navigation.
  Reflect.set(SyncManager, 'instance', undefined);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect(await getActions()).toEqual([]));
  expect(fetch).toHaveBeenCalledTimes(3); // Invoice once and both frozen quote routes once.
  expect(vi.mocked(fetch).mock.calls.some(([url]) => url === '/api/v1/sync/events')).toBe(false);
  Reflect.set(SyncManager, 'instance', undefined);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).toHaveBeenCalledTimes(3);
  expect(rows.size).toBe(3); // Durable parent and child tombstones survive recovery.
});
it('hard navigation before the receipt still releases and drains follow-ups only after explicit acknowledgement', async () => {
  await enqueueAction(completion, owner);
  Reflect.set(SyncManager, 'instance', undefined);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect(await getActions()).toEqual([]));
  expect(vi.mocked(fetch).mock.calls.map(([url]) => url)).toEqual(['/api/v1/sync/events', '/api/v1/invoices/generate', '/api/v1/sync/operation-intents', '/api/v1/sync/offline']);
});
it('restores only the original owner after a session transition and never sends its children as another user', async () => {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'other-user', tenantId: 'other-tenant' });
  await completeAction(claim, 'acknowledged');
  await SyncManager.getInstance().resumeFieldCompletionWork(); expect(fetch).not.toHaveBeenCalled();
  Reflect.set(SyncManager, 'instance', undefined); vi.mocked(readQueueOwner).mockResolvedValue(owner);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect(await getActions()).toEqual([]));
  for (const [, options] of vi.mocked(fetch).mock.calls) {
    expect(new Headers(options?.headers).get('x-ohc-expected-user')).toBe(owner.userId);
    expect(new Headers(options?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  }
});
it.each(['network', 'wrong-receipt', 'http-409'])('does not reconstruct or replay a completion with %s uncertainty after reload', async kind => {
  await enqueueAction(completion, owner);
  vi.mocked(fetch).mockImplementation(async () => {
    if (kind === 'network') throw new Error('Lost acknowledgement');
    if (kind === 'http-409') return new Response('', { status: 409 });
    return Response.json({ success: true, outcomes: [{ id: 'wrong-id', route: '/api/v1/sync/events', status: 'acknowledged' }] });
  });
  await SyncManager.getInstance().resumeFieldCompletionWork();
  Reflect.set(SyncManager, 'instance', undefined);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).toHaveBeenCalledTimes(1); expect(rows.size).toBe(1);
  expect((await getActions()).map(action => action.type)).toEqual(['sync_event']);
});
it('holds an ambiguous child across hard reload while preserving the parent acknowledgement', async () => {
  await acknowledgedCompletion();
  const confirmed = vi.mocked(fetch).getMockImplementation()!;
  vi.mocked(fetch).mockImplementation(async (url, options) => { if (url === '/api/v1/invoices/generate') throw new Error('Lost invoice response'); return confirmed(url, options); });
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect((await getQueueSummary()).reconciliation).toBe(1));
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(3));
  Reflect.set(SyncManager, 'instance', undefined); await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).toHaveBeenCalledTimes(3); expect(await claimAction(completion.id, '/api/v1/sync/events')).toBeNull();
});
it('keeps receipt-triggered recovery passes field-only when unrelated work is queued beside the parent', async () => {
  await enqueueAction({ id: 'unrelated-triage', type: 'triage_action', timestamp: 99, payload: { id: 'other-card', approved: true } }, owner);
  await enqueueAction(completion, owner);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect((await getActions()).map(action => action.id)).toEqual(['unrelated-triage']));
  expect(vi.mocked(fetch).mock.calls.map(([url]) => url)).toEqual(['/api/v1/sync/events', '/api/v1/invoices/generate', '/api/v1/sync/operation-intents', '/api/v1/sync/offline']);
});

it('serializes the real field quote child with the required object intent payload and unchanged raw offline notes', async () => {
  await acknowledgedCompletion();
  await SyncManager.getInstance().resumeFieldCompletionWork();
  await vi.waitFor(async () => expect(await getActions()).toEqual([]));
  const id = 'field-completion-offline-completion-quote';
  const notes = 'Follow up quote requested by field op for job job-1. Notes: Requested estimate';
  const timestamp = new Date(completion.timestamp).toISOString();
  const intent = vi.mocked(fetch).mock.calls.find(([url]) => url === '/api/v1/sync/operation-intents')!;
  const offline = vi.mocked(fetch).mock.calls.find(([url]) => url === '/api/v1/sync/offline')!;
  expect(JSON.parse(String(intent[1]?.body))).toEqual({ intents: [{ id, action_type: 'draft_quote', payload: { notes }, timestamp }] });
  expect(JSON.parse(String(offline[1]?.body))).toEqual({ mutations: [{ timestamp, transaction_id: id, quantity_deducted: 0, amount: null, payment_method: null, payment_intent_id: null, currency: 'usd', product_id: 'draft_quote', mutation_type: 'draft_quote', payload: notes }] });
  expect(new Headers(intent[1]?.headers).get('Idempotency-Key')).toBe(id);
  expect(new Headers(offline[1]?.headers).get('Idempotency-Key')).toBe(id);
});
it('retains a pre-fix blocked quote envelope without rewriting its request identity or replaying the rejected intent', async () => {
  const legacy: OfflineAction = { id: 'previously-rejected-quote', type: 'draft_quote', timestamp: 99, notes: 'Original saved notes', field_completion_parent_id: 'old-completion' };
  await enqueueAction(legacy, owner);
  const intent = (await claimAction(legacy.id, '/api/v1/sync/operation-intents'))!;
  await completeAction(intent, 'blocked', 'HTTP 422');
  const before = String(rows.get(legacy.id)!.payload);
  // The other already-frozen route is explicitly held too; neither may be replayed.
  const offline = (await claimAction(legacy.id, '/api/v1/sync/offline'))!;
  await completeAction(offline, 'reconciliation', 'Unknown existing result');
  const held = String(rows.get(legacy.id)!.payload);
  expect(JSON.parse(before).action).toEqual(legacy);
  await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).not.toHaveBeenCalled(); expect(String(rows.get(legacy.id)!.payload)).toBe(held);
  expect(await getActions()).toEqual([legacy]);
});
