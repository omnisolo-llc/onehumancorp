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
