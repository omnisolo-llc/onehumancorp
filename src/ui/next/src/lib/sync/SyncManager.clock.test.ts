import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { StoredQueueRow } from './queueStorage';
import type { OfflineAction } from '../../app/utils/offlineQueue';

vi.mock('../powersync/db', () => ({ getPowerSyncDB: vi.fn() }));
describe.each(['powersync', 'indexeddb'] as const)('%s clock journal', adapter => {
  let rows: Map<string, StoredQueueRow>;
  let queue: typeof import('../../app/utils/offlineQueue');
  let identity: typeof import('./queueIdentity');
  let manager: import('./SyncManager').SyncManager;
  let owner: { userId: string; tenantId: string };
  let transport: ReturnType<typeof vi.fn<typeof fetch>>;
  let tail: Promise<unknown>;
  let failCommit: boolean;
  let delayTransaction: (() => Promise<void>) | undefined;
  let delayWrite: (() => Promise<void>) | undefined;
  let delayCommit: (() => Promise<void>) | undefined;
  let authenticated: boolean;
  let removeListeners: (() => void)[];
  const route = '/api/v1/staff/timecard';
  const clock: OfflineAction = { id: 'clock-1', type: 'staff_clock_event_v1', timestamp: 1, payload: { staff_id: 'a', event_type: 'CLOCK_IN' } };
  const ack = (id = clock.id) => ({ success: true, outcomes: [{ id, route, status: 'acknowledged' }] });
  const receipt = () => ({ ...ack(), receipt: { version: 1, actor_id: 'a', id: clock.id, staff_id: 'a', event_type: 'CLOCK_IN', offline_timestamp: '1970-01-01T00:00:00.001Z' } });
  const rowBytes = () => JSON.stringify([...rows.values()]);
  const pending = <T,>() => { let resolve!: (value: T) => void; const promise = new Promise<T>(yes => { resolve = yes; }); return { resolve, promise }; };
  async function restart() {
    for (const remove of removeListeners) remove(); removeListeners = [];
    vi.resetModules();
    const { getPowerSyncDB } = await import('../powersync/db');
    vi.mocked(getPowerSyncDB).mockResolvedValue({ writeTransaction: async (work: (tx: object) => Promise<unknown>) => {
      const run = tail.then(async () => {
        if (delayTransaction) await delayTransaction();
        const staged = new Map(adapter === 'powersync' ? rows : []);
        let wrote = false;
        const result = await work({ getAll: async () => [...staged.values()], execute: async (_sql: string, args: unknown[]) => {
          staged.set(String(args[0]), { id: String(args[0]), type: String(args[1]), payload: args[2], timestamp: Number(args[3]) });
          wrote = true; if (delayWrite && adapter === 'powersync') await delayWrite();
        } });
        if (wrote && delayCommit && adapter === 'powersync') await delayCommit();
        if (failCommit) throw new Error('Commit aborted');
        if (adapter === 'powersync') rows = staged; return result;
      });
      tail = run.catch(() => {}); return run;
    } } as never);
    queue = await import('../../app/utils/offlineQueue');
    identity = await import('./queueIdentity');
    manager = (await import('./SyncManager')).SyncManager.getInstance();
  }
  // Exercise the actual IndexedDB adapter's request/transaction commit boundary.
  function installIndexedDB() {
    vi.stubGlobal('indexedDB', { open: () => {
      const db = { close: () => {}, transaction: () => {
        let aborted = false;
        let wrote = false;
        let staged = new Map<string, StoredQueueRow>();
        const request = { result: [] as StoredQueueRow[], onsuccess: null as null | (() => void), onerror: null as null | (() => void) };
        const tx = { oncomplete: null as null | (() => void), onabort: null as null | (() => void), onerror: null as null | (() => void), error: new Error('Commit aborted'),
          abort: () => { aborted = true; }, objectStore: () => ({ getAll: () => request, put: (row: StoredQueueRow) => { wrote = true; staged.set(row.id, row); } }) };
        const run = tail.then(async () => {
          if (delayTransaction && adapter === 'indexeddb') await delayTransaction();
          staged = new Map(adapter === 'indexeddb' ? rows : []);
          request.result = [...staged.values()]; request.onsuccess?.();
          if (wrote && delayWrite && adapter === 'indexeddb') await delayWrite();
          await Promise.resolve();
          if (wrote && delayCommit && adapter === 'indexeddb') await delayCommit();
          if (aborted || failCommit) tx.onabort?.();
          else { if (adapter === 'indexeddb') rows = staged; tx.oncomplete?.(); }
        });
        tail = run.catch(() => {}); return tx;
      } };
      const opening = { result: db, onsuccess: null as null | (() => void) };
      queueMicrotask(() => opening.onsuccess?.()); return opening;
    } });
  }
  beforeEach(async () => {
    rows = new Map(); owner = { userId: 'a', tenantId: 't' }; tail = Promise.resolve(); failCommit = false; delayTransaction = undefined; delayWrite = undefined; delayCommit = undefined; authenticated = true; removeListeners = [];
    localStorage.clear(); localStorage.setItem('omnisolo_queue_adapter_v2', adapter);
    installIndexedDB();
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
    vi.spyOn(navigator.locks, 'request').mockImplementation((async (_name: string, work: () => Promise<unknown>) => work()) as never);
    const add = window.addEventListener.bind(window);
    vi.spyOn(window, 'addEventListener').mockImplementation((type, listener, options) => {
      add(type, listener, options); removeListeners.push(() => window.removeEventListener(type, listener, options));
    });
    vi.spyOn(console, 'error').mockImplementation(() => {});
    transport = vi.fn<typeof fetch>(async (input, init) => {
      if (String(input) === '/api/v1/auth/session-identity') return authenticated ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : Response.json({ error: 'logged out' }, { status: 401 });
      if (String(input) === route && init?.method === 'POST') return Response.json(ack());
      if (String(input) === `${route}/receipts/${clock.id}` && init?.method === 'GET') return Response.json(receipt());
      throw new Error(`Unexpected test request ${input}`);
    });
    vi.stubGlobal('fetch', transport); await restart();
  });
  afterEach(async () => {
    await tail; for (const remove of removeListeners) remove(); identity.invalidateQueueOwner();
    vi.restoreAllMocks(); vi.unstubAllGlobals();
  });
  const businessCalls = () => transport.mock.calls.filter(([url]) => !String(url).includes('session-identity'));
  async function attempted(status: 'reconciliation' | 'blocked' | 'inflight' = 'reconciliation') {
    await queue.enqueueAction(clock, owner);
    const claim = await queue.claimAction(clock.id, route);
    expect(claim).not.toBeNull();
    if (status !== 'inflight') await queue.completeAction(claim!, status);
  }

  it.each(['pending', 'acknowledged'] as const)('never sends or rewrites historical clocks with %s/pending routes across reload, online and unrelated enqueue', async first => {
    const { planRoutes } = await import('./queueRoutes');
    for (const type of ['CLOCK_IN', 'CLOCK_OUT']) {
      const action = { id: type, type, timestamp: 1, payload: { staff_id: 'a', timestamp: '1970-01-01T00:00:00.001Z' } };
      rows.set(type, { id: type, type, timestamp: 1, payload: JSON.stringify({ version: 2, adapter, owner, action, context: {},
        routes: planRoutes(action).map((plan, i) => ({ plan, status: i === 0 ? first : 'pending', attempts: i === 0 && first === 'acknowledged' ? 1 : 0 })) }) });
    }
    const original = rowBytes();
    await restart(); await manager.sync();
    window.dispatchEvent(new Event('online')); await vi.waitFor(() => expect(Reflect.get(manager, 'syncInProgress')).toBe(false));
    expect(businessCalls()).toEqual([]); expect(rowBytes()).toBe(original);
    transport.mockImplementation(async input => String(input).includes('session-identity') ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : Response.json({ outcomes: [{ id: 'other', route: '/api/v1/ui/triage/action', status: 'acknowledged' }] }));
    await manager.enqueue({ id: 'other', type: 'triage_action', timestamp: 1, payload: { approved: true } });
    await vi.waitFor(() => expect(Reflect.get(manager, 'syncInProgress')).toBe(false));
    expect(businessCalls().map(([url]) => url)).toEqual(['/api/v1/ui/triage/action']);
    expect(JSON.stringify([...rows.values()].filter(row => row.id !== 'other'))).toBe(original);
    expect(await queue.getClockQueueSummary()).toEqual({ confirmed: 0, unconfirmed: 0, legacyHeld: 2 });
    expect(await queue.claimAction('CLOCK_IN', '/api/v1/sync/offline')).toBeNull();
    expect(JSON.stringify([...rows.values()].filter(row => row.id !== 'other'))).toBe(original);
  });
  it('sends one new clock event to only its frozen timecard endpoint and requires a per-ID outcome', async () => {
    await queue.enqueueAction(clock, owner); await manager.sync();
    expect(await queue.getActions()).toEqual([]);
    expect(businessCalls()).toHaveLength(1);
    expect(businessCalls()[0]).toEqual([route, expect.objectContaining({ method: 'POST', body: JSON.stringify({ events: [{ id: clock.id, staff_id: 'a', event_type: 'CLOCK_IN', offline_timestamp: '1970-01-01T00:00:00.001Z' }] }) })]);
    expect(await queue.getClockQueueSummary()).toEqual({ confirmed: 1, unconfirmed: 0, legacyHeld: 0 });
  });
  it('holds success:true without a receipt and never automatically replays the POST', async () => {
    transport.mockImplementation(async input => String(input).includes('session-identity') ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 }) : Response.json({ success: true }));
    await queue.enqueueAction(clock, owner); await manager.sync(); await manager.sync();
    expect(await queue.getActions()).toEqual([clock]); expect(businessCalls()).toHaveLength(1);
  });
  it('recovers a lost response after reload using receipt GET only and never drains unrelated work', async () => {
    transport.mockImplementationOnce(async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 }));
    await queue.enqueueAction(clock, owner);
    const defaultTransport = transport.getMockImplementation()!;
    transport.mockImplementation(async (input, init) => String(input) === route ? Promise.reject(new Error('Lost POST response')) : defaultTransport(input, init));
    await manager.sync(); await restart();
    await queue.enqueueAction({ id: 'unrelated', type: 'triage_action', timestamp: 1 });
    await manager.reconcileClockReceipts(); await manager.reconcileClockReceipts();
    expect((await queue.getActions()).map(a => a.id)).toEqual(['unrelated']);
    expect(businessCalls().map(([url, init]) => [url, init?.method])).toEqual([[route, 'POST'], [`${route}/receipts/${clock.id}`, 'GET']]);
    expect(businessCalls()[1][1]).toMatchObject({ credentials: 'same-origin', cache: 'no-store', redirect: 'error', headers: { 'x-ohc-expected-user': 'a', 'x-ohc-expected-tenant': 't' } });
  });
  it.each(['reconciliation', 'blocked', 'inflight'] as const)('reconciles a matching receipt for an attempted %s event without replay', async status => {
    await attempted(status); await manager.reconcileClockReceipts();
    expect(await queue.getActions()).toEqual([]); expect(businessCalls().map(([url]) => url)).toEqual([`${route}/receipts/${clock.id}`]);
  });
  it.each(['id', 'actor_id', 'staff_id', 'event_type', 'offline_timestamp', 'version', 'outcome-id', 'outcome-route', 'success-only', 'missing', 'unavailable', 'contradictory'])('holds %s receipt evidence byte-for-byte', async field => {
    await attempted(); const original = rowBytes(); const body = receipt();
    if (field === 'outcome-id') body.outcomes[0].id = 'other';
    else if (field === 'outcome-route') body.outcomes[0].route = '/api/v1/sync/operation-intents';
    else if (field === 'contradictory') body.success = false;
    else if (field in body.receipt) Object.assign(body.receipt, { [field]: field === 'version' ? 2 : 'different' });
    transport.mockImplementation(async input => String(input).includes('session-identity') ? Response.json({ ...owner, expiresAt: Date.now() + 60_000 })
      : Response.json(field === 'success-only' ? { success: true } : body, { status: field === 'missing' ? 404 : field === 'unavailable' ? 503 : 200 }));
    await manager.reconcileClockReceipts(); expect(rowBytes()).toBe(original); expect(await queue.getActions()).toEqual([clock]);
  });
  it.each(['owner', 'epoch', 'invalidation'] as const)('rejects a late receipt after %s changes', async change => {
    await attempted(); const original = rowBytes(); const reply = pending<Response>();
    const normal = transport.getMockImplementation()!;
    transport.mockImplementation(async (input, init) => String(input).includes('/receipts/') ? reply.promise : normal(input, init));
    const checking = manager.reconcileClockReceipts();
    await vi.waitFor(() => expect(businessCalls()).toHaveLength(1));
    if (change === 'owner') owner = { userId: 'b', tenantId: 'other' };
    else if (change === 'epoch') localStorage.setItem(identity.QUEUE_IDENTITY_EPOCH_KEY, 'changed');
    else identity.invalidateQueueOwner();
    reply.resolve(Response.json(receipt())); await checking; expect(rowBytes()).toBe(original);
  });
  it('serializes duplicate status checks before their first await', async () => {
    await attempted(); const reply = pending<Response>(); const normal = transport.getMockImplementation()!;
    transport.mockImplementation(async (input, init) => String(input).includes('/receipts/') ? reply.promise : normal(input, init));
    const a = manager.reconcileClockReceipts(); const b = manager.reconcileClockReceipts();
    await vi.waitFor(() => expect(businessCalls()).toHaveLength(1));
    reply.resolve(Response.json(receipt())); await Promise.all([a, b]);
    expect(await queue.getActions()).toEqual([]); expect(businessCalls()).toHaveLength(1);
  });
  it('does not query never-attempted clocks or reinterpret historical events', async () => {
    await queue.enqueueAction(clock); await queue.enqueueAction({ ...clock, id: 'old', type: 'CLOCK_IN', payload: { staff_id: 'a', timestamp: '1970-01-01T00:00:00.001Z' } });
    const original = rowBytes(); await manager.reconcileClockReceipts();
    expect(businessCalls()).toEqual([]); expect(rowBytes()).toBe(original);
  });
  it('freezes candidates and compare-and-sets the entire original attempted record', async () => {
    await attempted(); const [candidate] = await queue.getClockReceiptCandidates();
    expect(Object.isFrozen(candidate)).toBe(true); expect(Object.isFrozen(candidate.action.payload)).toBe(true); expect(Object.isFrozen(candidate.route)).toBe(true);
    const stored = rows.get(clock.id)!; const changed = JSON.parse(String(stored.payload)); changed.routes[0].reason = 'new evidence';
    rows.set(clock.id, { ...stored, payload: JSON.stringify(changed) }); const original = rowBytes();
    expect(await queue.acknowledgeClockReceipt(candidate, 200, receipt())).toBe(false); expect(rowBytes()).toBe(original);
  });
  it('checks the identity fence inside the storage transaction after its await', async () => {
    await attempted(); const [candidate] = await queue.getClockReceiptCandidates(); const original = rowBytes();
    delayTransaction = async () => { identity.invalidateQueueOwner(); delayTransaction = undefined; };
    expect(await queue.acknowledgeClockReceipt(candidate, 200, receipt())).toBe(false); expect(rowBytes()).toBe(original);
  });
  it('retains the attempted clock if the local receipt commit fails', async () => {
    await attempted(); const original = rowBytes(); const [candidate] = await queue.getClockReceiptCandidates();
    failCommit = true;
    await expect(queue.acknowledgeClockReceipt(candidate, 200, receipt())).rejects.toThrow('Commit aborted');
    failCommit = false; expect(rowBytes()).toBe(original);
  });

  it('preserves an explicitly supplied zero clock timestamp instead of replacing it with the current time', async () => {
    await identity.readQueueOwner(); vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    await manager.enqueue({ ...clock, timestamp: 0 }, owner);
    expect((await queue.getActions())[0].timestamp).toBe(0);
    expect(JSON.parse(String(rows.get(clock.id)!.payload)).routes[0].plan.body.events[0].offline_timestamp).toBe('1970-01-01T00:00:00.000Z');
  });
  it.each(['write', 'commit'] as const)('finishes only the original-owner receipt after logout during delayed %s', async phase => {
    await attempted(); const [candidate] = await queue.getClockReceiptCandidates();
    const entered = pending<void>(); const release = pending<void>();
    const delay = () => { entered.resolve(); return release.promise; };
    if (phase === 'write') delayWrite = delay; else delayCommit = delay;
    const saving = queue.acknowledgeClockReceipt(candidate, 200, receipt()); await entered.promise;
    authenticated = false; identity.notifyQueueIdentityChange();
    release.resolve(); expect(await saving).toBe(true);
    const original = JSON.parse(String(rows.get(clock.id)!.payload));
    expect(original.owner).toEqual({ userId: 'a', tenantId: 't' }); expect(original.action).toEqual(clock);
    expect(original.routes[0]).toMatchObject({ status: 'acknowledged', attempts: 1 });
    await expect(queue.getClockQueueSummary()).rejects.toThrow('identity unavailable');
  });
  it.each(['write', 'commit'] as const)('cannot confirm or change the new account from an old receipt during delayed %s', async phase => {
    await attempted();
    // A separately owned immutable record already exists before this transaction.
    const other = JSON.parse(String(rows.get(clock.id)!.payload));
    other.owner = { userId: 'b', tenantId: 'other' }; other.action.id = 'clock-b'; other.action.payload.staff_id = 'b';
    other.routes[0].plan.body.events[0].id = 'clock-b'; other.routes[0].plan.body.events[0].staff_id = 'b';
    const otherRow = { id: 'clock-b', type: clock.type, timestamp: clock.timestamp, payload: JSON.stringify(other) };
    rows.set('clock-b', otherRow);
    const [candidate] = await queue.getClockReceiptCandidates();
    const entered = pending<void>(); const release = pending<void>();
    const delay = () => { entered.resolve(); return release.promise; };
    if (phase === 'write') delayWrite = delay; else delayCommit = delay;
    const saving = queue.acknowledgeClockReceipt(candidate, 200, receipt()); await entered.promise;
    owner = { userId: 'b', tenantId: 'other' }; identity.notifyQueueIdentityChange(); await identity.readQueueOwner();
    release.resolve(); expect(await saving).toBe(true);
    expect(rows.get('clock-b')).toEqual(otherRow);
    expect(await queue.getClockQueueSummary()).toEqual({ confirmed: 0, unconfirmed: 1, legacyHeld: 0 });
    expect((await queue.getActions()).map(action => action.id)).toEqual(['clock-b']);
  });
  it.each(['owner', 'action', 'route', 'attempt-token', 'attempt-status'] as const)('rejects a replacement %s with the same action ID before receipt CAS', async field => {
    await attempted(); const [candidate] = await queue.getClockReceiptCandidates();
    const row = rows.get(clock.id)!; const changed = JSON.parse(String(row.payload));
    if (field === 'owner') changed.owner = { userId: 'b', tenantId: 'other' };
    if (field === 'action') { changed.action.payload.event_type = 'CLOCK_OUT'; changed.routes[0].plan.body.events[0].event_type = 'CLOCK_OUT'; }
    if (field === 'route') changed.routes[0].plan.id = '/api/v1/sync/operation-intents';
    if (field === 'attempt-token') changed.routes[0].attemptToken = 'new-attempt';
    if (field === 'attempt-status') changed.routes[0].status = 'blocked';
    rows.set(clock.id, { ...row, payload: JSON.stringify(changed) }); const original = rowBytes();
    expect(await queue.acknowledgeClockReceipt(candidate, 200, receipt())).toBe(false); expect(rowBytes()).toBe(original);
    if (field === 'owner') {
      owner = { userId: 'b', tenantId: 'other' }; identity.notifyQueueIdentityChange();
      expect(await queue.getClockQueueSummary()).toEqual({ confirmed: 0, unconfirmed: 1, legacyHeld: 0 });
    }
    expect(businessCalls()).toEqual([]);
  });

});
