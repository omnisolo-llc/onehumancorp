import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { enqueueAction, enqueueActions, getActions, claimAction, completeAction, getQueueSummary } from './offlineQueue';
import { readQueueOwner } from '../../lib/sync/queueIdentity';
import { getPowerSyncDB } from '../../lib/powersync/db';
import type { OfflineAction } from './offlineQueue';

vi.mock('../../lib/powersync/db', () => ({ getPowerSyncDB: vi.fn() }));
vi.mock('../../lib/sync/queueIdentity', async importOriginal => ({ ...await importOriginal<object>(), readQueueOwner: vi.fn() }));
let rows: Map<string, Record<string, unknown>>;
let failCommit: boolean;
let tail: Promise<unknown>;
const owner = { userId: 'a', tenantId: 't' };
const action: OfflineAction = { id: 'one', type: 'update_quote', payload: { title: 'Saved' }, quoteId: 'quote', notes: 'terms', amount: 500, currency: 'eur', device_signature: 'signature', timestamp: 1 };

beforeEach(() => {
  localStorage.clear(); rows = new Map(); failCommit = false; tail = Promise.resolve();
  vi.mocked(readQueueOwner).mockResolvedValue(owner);
  vi.spyOn(navigator.locks, 'request').mockImplementation((async (_name: string, work: () => Promise<unknown>) => work()) as never);
  const transaction = async (work: (tx: object) => Promise<unknown>) => {
    const run = tail.then(async () => {
      const staged = new Map(rows);
      const result = await work({
        getAll: async () => [...staged.values()],
        execute: async (_sql: string, args: unknown[]) => { staged.set(args[0] as string, { id: args[0], type: args[1], payload: args[2], timestamp: args[3] }); },
      });
      if (failCommit) throw new Error('Commit aborted');
      rows = staged; return result;
    });
    tail = run.catch(() => {}); return run;
  };
  vi.mocked(getPowerSyncDB).mockResolvedValue({ getAll: async () => [...rows.values()], writeTransaction: transaction } as never);
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });

describe('durable owner-scoped queue', () => {
  it('persists the full action and frozen route without replacing an immutable ID', async () => {
    await enqueueAction(action);
    expect(await getActions()).toEqual([action]);
    await enqueueAction({ ...action });
    await expect(enqueueAction({ ...action, amount: 999 })).rejects.toThrow('collision');
    expect(await getActions()).toEqual([action]);
  });
  it('atomically claims once across independent concurrent consumers and survives reload', async () => {
    await enqueueAction(action);
    const results = await Promise.all([claimAction('one', '/api/v1/quotes?id=quote'), claimAction('one', '/api/v1/quotes?id=quote')]);
    expect(results.filter(Boolean)).toHaveLength(1);
    expect(await claimAction('one', '/api/v1/quotes?id=quote')).toBeNull();
    expect((await getQueueSummary()).reconciliation).toBe(1);
  });
  it('keeps acknowledgement durable, does not reclaim it, and ignores foreign attempt tokens', async () => {
    await enqueueAction(action);
    const claim = (await claimAction('one', '/api/v1/quotes?id=quote'))!;
    await expect(completeAction({ ...claim, attemptToken: 'foreign' }, 'acknowledged')).rejects.toThrow('claim');
    await completeAction(claim, 'acknowledged');
    expect(await getActions()).toEqual([]);
    expect(await claimAction('one', claim.route.id)).toBeNull();
    expect(rows.size).toBe(1); // An acknowledged tombstone prevents accidental ID reuse.
  });
  it('never falls back after a selected adapter fails or a commit aborts', async () => {
    await enqueueAction(action);
    failCommit = true;
    await expect(claimAction('one', '/api/v1/quotes?id=quote')).rejects.toThrow('Commit aborted');
    failCommit = false;
    const claim = await claimAction('one', '/api/v1/quotes?id=quote');
    expect(claim).not.toBeNull();
    vi.mocked(getPowerSyncDB).mockRejectedValue(new Error('Storage unavailable'));
    await expect(completeAction(claim!, 'acknowledged')).rejects.toThrow('Storage unavailable');
    expect(localStorage.getItem('omnisolo_queue_adapter_v2')).toBe('powersync');
  });
  it('keeps all items when an atomic batch commit aborts', async () => {
    failCommit = true;
    await expect(enqueueActions([action, { ...action, id: 'two' }])).rejects.toThrow('Commit aborted');
    expect(rows.size).toBe(0);
  });
  it('holds legacy rows without exposing their payload to the current account', async () => {
    rows.set('legacy', { id: 'legacy', type: 'cash_sale', payload: '{"private":"old owner"}', timestamp: 1 });
    expect(await getActions()).toEqual([]);
    expect((await getQueueSummary()).legacyHeld).toBe(1);
    await expect(enqueueAction({ ...action, id: 'legacy' })).rejects.toThrow('collision');
  });
  it('hides another owner and commits an in-flight response to its original row after account switch', async () => {
    await enqueueAction(action);
    const claim = (await claimAction('one', '/api/v1/quotes?id=quote'))!;
    vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'b', tenantId: 'other' });
    expect(await getActions()).toEqual([]);
    expect(await claimAction('one', claim.route.id)).toBeNull();
    await completeAction(claim, 'acknowledged');
    expect(await getActions()).toEqual([]);
    vi.mocked(readQueueOwner).mockResolvedValue(owner);
    expect(await getActions()).toEqual([]);
  });
});

it('freezes terminal route metadata synchronously before identity/storage awaits', async () => {
  let resolve!: (value: typeof owner) => void;
  vi.mocked(readQueueOwner).mockReturnValueOnce(new Promise(r => { resolve = r; }));
  localStorage.setItem('omnisolo_pos_device_id', 'original-device');
  const saving = enqueueAction({ id: 'terminal', type: 'cash_sale', timestamp: 1, amount: 1500 });
  localStorage.setItem('omnisolo_pos_device_id', 'new-device');
  resolve(owner); await saving;
  const saved = JSON.parse(String(rows.get('terminal')!.payload));
  expect(saved.routes[0].plan.body.transactions[0].client_id).toBe('original-device');
});
it('retains a multi-route action until every frozen route is explicitly acknowledged', async () => {
  await enqueueAction({ id: 'multiple', type: 'agent_intent', timestamp: 1, payload: { goal: 'test' } });
  const first = (await claimAction('multiple', '/api/v1/sync/operation-intents'))!;
  await completeAction(first, 'acknowledged');
  expect(await getActions()).toHaveLength(1);
  const second = (await claimAction('multiple', '/api/v1/sync/offline'))!;
  await completeAction(second, 'reconciliation');
  expect((await getQueueSummary()).reconciliation).toBe(1);
  expect(await claimAction('multiple', first.route.id)).toBeNull();
  expect(await claimAction('multiple', second.route.id)).toBeNull();
});
it('does not lose the durable in-flight marker when acknowledgement commit aborts', async () => {
  await enqueueAction(action);
  const claim = (await claimAction(action.id, '/api/v1/quotes?id=quote'))!;
  failCommit = true;
  await expect(completeAction(claim, 'acknowledged')).rejects.toThrow('Commit aborted');
  failCommit = false;
  expect((await getQueueSummary()).reconciliation).toBe(1);
  expect(await claimAction(action.id, claim.route.id)).toBeNull();
});
it('quarantines a corrupted persisted route instead of sending queue data outside the API', async () => {
  await enqueueAction(action);
  const row = rows.get(action.id)!;
  const saved = JSON.parse(String(row.payload));
  saved.routes[0].plan.id = 'https://foreign.example/upload';
  rows.set(action.id, { ...row, payload: JSON.stringify(saved) });
  expect(await getActions()).toEqual([]);
  expect((await getQueueSummary()).legacyHeld).toBe(1);
  expect(await claimAction(action.id, 'https://foreign.example/upload')).toBeNull();
});
it.each(['destination', 'method', 'body'])('quarantines a persisted %s inconsistent with its declared immutable action', async field => {
  await enqueueAction(action);
  const row = rows.get(action.id)!;
  const saved = JSON.parse(String(row.payload));
  if (field === 'destination') saved.routes[0].plan = { id: '/api/v1/payments/terminal/sync_offline', method: 'POST', maxAttempts: 1, body: { transactions: [{ id: action.id, amount_cents: 9999 }] } };
  else if (field === 'method') saved.routes[0].plan.method = 'PUT';
  else saved.routes[0].plan.body = { title: 'Changed after enqueue' };
  rows.set(action.id, { ...row, payload: JSON.stringify(saved) });
  expect(await getActions()).toEqual([]);
  expect(await claimAction(action.id, saved.routes[0].plan.id)).toBeNull();
});
it('round-trips omitted optional financial fields when validating the frozen plan', async () => {
  const cash = { id: 'cash', type: 'cash_sale', timestamp: 1, amount: 100 };
  await enqueueAction(cash);
  expect(await getActions()).toEqual([cash]);
  expect(await claimAction(cash.id, '/api/v1/payments/terminal/sync_offline')).not.toBeNull();
});

it('refuses to append a prior view action after the verified account changes', async () => {
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'b', tenantId: 'other' });
  const storageReads = vi.mocked(getPowerSyncDB).mock.calls.length;
  await expect(enqueueAction(action, owner)).rejects.toThrow('owner does not match');
  expect(rows.size).toBe(0);
  expect(getPowerSyncDB).toHaveBeenCalledTimes(storageReads);
});
it('freezes the intended view owner before awaiting identity verification', async () => {
  const intended = { ...owner };
  const storageReads = vi.mocked(getPowerSyncDB).mock.calls.length;
  let finish!: (value: typeof owner) => void;
  vi.mocked(readQueueOwner).mockReturnValueOnce(new Promise(resolve => { finish = resolve; }));
  const saving = enqueueAction(action, intended);
  intended.userId = 'b'; intended.tenantId = 'other';
  const rejected = expect(saving).rejects.toThrow('owner does not match');
  finish({ userId: 'b', tenantId: 'other' });
  await rejected;
  expect(rows.size).toBe(0);
  expect(getPowerSyncDB).toHaveBeenCalledTimes(storageReads);
});

const completion: OfflineAction = {
  id: 'completion-1', type: 'sync_event', timestamp: 123,
  payload: { entity_type: 'appointment', entity_id: 'job-1', action_type: 'UpdateStatus', base_version: 0,
    payload: { status: 'Completed', expected_status: 'In-Progress', expected_updated_at: '2026-10-03T10:00:00Z', notes: 'Requested estimate' } },
  field_completion: { job_id: 'job-1', customer_id: 'customer-1', notes: 'Requested estimate' },
};
it('atomically appends captured field follow-ups only with a durable completion acknowledgement', async () => {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  expect((await getActions()).map(action => action.type)).toEqual(['sync_event']);
  await completeAction(claim, 'acknowledged');
  const pending = await getActions();
  expect(pending).toEqual([
    { id: 'field-completion-completion-1-invoice', type: 'generate_invoice', timestamp: 123, payload: { job_id: 'job-1', customer_id: 'customer-1' }, field_completion_parent_id: 'completion-1' },
    { id: 'field-completion-completion-1-quote', type: 'draft_quote', timestamp: 123, notes: 'Follow up quote requested by field op for job job-1. Notes: Requested estimate', payload: { notes: 'Follow up quote requested by field op for job job-1. Notes: Requested estimate' }, field_completion_parent_id: 'completion-1' },
  ]);
  expect(rows.size).toBe(3);
  await enqueueAction(completion, owner);
  expect(await claimAction(completion.id, '/api/v1/sync/events')).toBeNull();
  expect(await getActions()).toEqual(pending);
});
it.each(['blocked', 'reconciliation'] as const)('never generates follow-ups from a %s completion receipt', async status => {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  await completeAction(claim, status);
  expect(rows.size).toBe(1); expect((await getActions()).map(action => action.type)).toEqual(['sync_event']);
});
it('keeps the parent inflight and creates no children if their shared receipt transaction aborts', async () => {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  failCommit = true;
  await expect(completeAction(claim, 'acknowledged')).rejects.toThrow('Commit aborted');
  failCommit = false;
  expect(rows.size).toBe(1); expect((await getQueueSummary()).reconciliation).toBe(1);
  expect(await claimAction(completion.id, '/api/v1/sync/events')).toBeNull();
});
it('keeps acknowledged completion children under the original owner after a session switch', async () => {
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'b', tenantId: 'other' });
  await completeAction(claim, 'acknowledged');
  expect(await getActions()).toEqual([]);
  for (const row of rows.values()) expect(JSON.parse(String(row.payload)).owner).toEqual(owner);
  vi.mocked(readQueueOwner).mockResolvedValue(owner);
  expect((await getActions()).map(action => action.type)).toEqual(['generate_invoice', 'draft_quote']);
});
it('cannot turn unrelated or inconsistent mutations into a field completion handoff', async () => {
  for (const bad of [{ ...completion, type: 'update_quote' }, { ...completion, field_completion: { ...completion.field_completion!, job_id: 'other' } }, { ...completion, field_completion: { ...completion.field_completion!, notes: 'not the saved notes' } }, { ...completion, payload: { ...completion.payload, payload: { status: 'Scheduled', notes: 'Requested estimate' } } }]) {
    await expect(enqueueAction(bad, owner)).rejects.toThrow(/completion/i);
  }
  expect(rows.size).toBe(0);
});
it('does not emit a quote follow-up when the captured completion has no notes', async () => {
  const noNotes = { ...completion, payload: { ...completion.payload, payload: { status: 'Completed', expected_updated_at: '2026-10-03T10:00:00Z', notes: '' } }, field_completion: { ...completion.field_completion!, notes: '' } };
  await enqueueAction(noNotes, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  await completeAction(claim, 'acknowledged');
  expect((await getActions()).map(action => action.type)).toEqual(['generate_invoice']);
});
it('holds a conflicting child ID instead of acknowledging the parent or replacing an existing action', async () => {
  await enqueueAction({ id: 'field-completion-completion-1-invoice', type: 'generate_invoice', timestamp: 123, payload: { job_id: 'other', customer_id: 'other' } }, owner);
  await enqueueAction(completion, owner);
  const claim = (await claimAction(completion.id, '/api/v1/sync/events'))!;
  await expect(completeAction(claim, 'acknowledged')).rejects.toThrow(/collision/i);
  expect(rows.size).toBe(2); expect((await getQueueSummary()).reconciliation).toBe(1);
});
