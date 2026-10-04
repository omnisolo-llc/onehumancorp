import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { SyncManager } from './SyncManager';
import { enqueueAction, getActions, getActionRoutes, claimAction, completeAction } from '../../app/utils/offlineQueue';
import { readQueueOwner } from './queueIdentity';
import { planRoutes } from './queueRoutes';
import type { OfflineAction } from '../../app/utils/offlineQueue';
vi.mock('../../app/utils/offlineQueue', () => ({ enqueueAction: vi.fn(), getActions: vi.fn(), getActionRoutes: vi.fn(), claimAction: vi.fn(), completeAction: vi.fn(), getQueueSummary: vi.fn() }));
vi.mock('./queueIdentity', async original => ({ ...await original<object>(), readQueueOwner: vi.fn() }));
const owner = { userId: 'a', tenantId: 't' };
let actions: OfflineAction[];
const states = new Map<string, string>();
const sale: OfflineAction = { id: 'sale', type: 'cash_sale', timestamp: 1, amount: 200 };
const triage: OfflineAction = { id: 'triage', type: 'triage_action', timestamp: 1, payload: { id: 'feed', approved: true } };
beforeEach(() => {
  vi.clearAllMocks(); states.clear(); actions = [sale];
  Reflect.set(SyncManager, 'instance', undefined);
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(true);
  vi.spyOn(console, 'error').mockImplementation(() => {});
  vi.mocked(readQueueOwner).mockResolvedValue(owner);
  vi.mocked(getActions).mockImplementation(async () => actions);
  vi.mocked(getActionRoutes).mockImplementation(async id => planRoutes(actions.find(a => a.id === id)!));
  vi.mocked(claimAction).mockImplementation(async (id, route) => {
    const key = `${id}:${route}`;
    if (states.has(key)) return null;
    states.set(key, 'inflight');
    const action = actions.find(a => a.id === id)!;
    return { action, owner, route: planRoutes(action).find(plan => plan.id === route)!, adapter: 'powersync', attemptToken: key };
  });
  vi.mocked(completeAction).mockImplementation(async (claim, status) => { states.set(claim.attemptToken, status); });
});
afterEach(() => { vi.restoreAllMocks(); vi.unstubAllGlobals(); });
const response = (id: string, route: string, status = 'acknowledged') => Response.json({ outcomes: [{ id, route, status }] });
describe('durable sync outcomes', () => {
  it.each([400, 401, 409, 429])('retains HTTP %s instead of deleting the queue', async status => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response('{}', { status })));
    await SyncManager.getInstance().sync();
    expect([...states.values()]).toEqual(['blocked']);
  });
  it('retains a 200 payment failure and does not automatically retry', async () => {
    const fetchMock = vi.fn().mockResolvedValue(Response.json({ success: false, processed: 1 }));
    vi.stubGlobal('fetch', fetchMock);
    await SyncManager.getInstance().sync(); await SyncManager.getInstance().sync();
    expect([...states.values()]).toEqual(['reconciliation']);
    expect(fetchMock).toHaveBeenCalledOnce();
  });
  it('does not replay an acknowledged POS action when another route returns 503', async () => {
    actions = [sale, triage];
    const fetchMock = vi.fn().mockImplementation(async (route: string) => route.includes('terminal') ? response('sale', route) : new Response('{}', { status: 503 }));
    vi.stubGlobal('fetch', fetchMock);
    await SyncManager.getInstance().sync(); await SyncManager.getInstance().sync();
    expect([...states.values()]).toEqual(['acknowledged', 'reconciliation']);
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });
  it('serializes before a delayed queue read', async () => {
    let resolve!: (value: OfflineAction[]) => void;
    vi.mocked(getActions).mockReturnValue(new Promise(r => { resolve = r; }));
    vi.stubGlobal('fetch', vi.fn(async (route: string) => response('sale', route)));
    const manager = SyncManager.getInstance();
    const first = manager.sync(); const second = manager.sync();
    resolve([sale]); await Promise.all([first, second]);
    expect(getActions).toHaveBeenCalledOnce(); expect(claimAction).toHaveBeenCalledOnce();
  });
  it('holds network ambiguity and adapter acknowledgement failures across repeated calls', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('Connection lost')));
    vi.mocked(completeAction).mockRejectedValue(new Error('Commit failed'));
    await SyncManager.getInstance().sync(); await SyncManager.getInstance().sync();
    expect([...states.values()]).toEqual(['inflight']); expect(fetch).toHaveBeenCalledOnce();
  });
  it('does not send a claimed old owner action after login changes', async () => {
    vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'b', tenantId: 'other' });
    vi.stubGlobal('fetch', vi.fn());
    await SyncManager.getInstance().sync();
    expect(fetch).not.toHaveBeenCalled(); expect([...states.values()]).toEqual(['blocked']);
  });
  it('binds each request to the original user and tenant precondition', async () => {
    vi.stubGlobal('fetch', vi.fn(async (route: string) => response('sale', route)));
    await SyncManager.getInstance().sync();
    expect(fetch).toHaveBeenCalledWith(expect.any(String), expect.objectContaining({ headers: expect.objectContaining({ 'x-ohc-expected-user': 'a', 'x-ohc-expected-tenant': 't' }) }));
  });
  it('normalizes ISO timestamps and rejects invalid input before storage', async () => {
    vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
    await SyncManager.getInstance().enqueue({ type: 'triage_action', timestamp: '2026-09-30T01:00:00Z' });
    expect(enqueueAction).toHaveBeenCalledWith(expect.objectContaining({ timestamp: Date.parse('2026-09-30T01:00:00Z') }));
    await expect(SyncManager.getInstance().enqueue({ type: 'cash_sale', timestamp: 'invalid' })).rejects.toThrow('timestamp');
    expect(enqueueAction).toHaveBeenCalledOnce();
  });
  it('drains a newly enqueued action after the active sync captured an older queue snapshot', async () => {
    let finish!: (response: Response) => void;
    const fetchMock = vi.fn().mockImplementation((route: string) => route.includes('terminal')
      ? new Promise<Response>(resolve => { finish = resolve; })
      : Promise.resolve(response('triage-new', route)));
    vi.stubGlobal('fetch', fetchMock);
    vi.mocked(getActions).mockImplementation(async () => [...actions]);
    vi.mocked(enqueueAction).mockImplementation(async action => { actions = [...actions, action]; });
    const manager = SyncManager.getInstance();
    const active = manager.sync();
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledOnce());
    await manager.enqueue({ ...triage, id: 'triage-new' }, owner);
    expect(fetchMock).toHaveBeenCalledOnce();
    finish(response('sale', '/api/v1/payments/terminal/sync_offline'));
    await active;
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));
    await vi.waitFor(() => expect([...states.values()]).toEqual(['acknowledged', 'acknowledged']));
  });
  it('does not replay an unknown old outcome when a new enqueue requests another drain', async () => {
    let fail!: (error: Error) => void;
    const fetchMock = vi.fn().mockImplementation((route: string) => route.includes('terminal')
      ? new Promise<Response>((_resolve, reject) => { fail = reject; })
      : Promise.resolve(response('triage-new', route)));
    vi.stubGlobal('fetch', fetchMock);
    vi.mocked(getActions).mockImplementation(async () => [...actions]);
    vi.mocked(enqueueAction).mockImplementation(async action => { actions = [...actions, action]; });
    const manager = SyncManager.getInstance();
    const active = manager.sync();
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledOnce());
    await manager.enqueue({ ...triage, id: 'triage-new' }, owner);
    fail(new Error('Lost response'));
    await active;
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledTimes(2));
    await vi.waitFor(() => expect([...states.values()]).toEqual(['reconciliation', 'acknowledged']));
    await manager.sync();
    expect(fetchMock).toHaveBeenCalledTimes(2);
  });
  it('holds follow-on work when the active acknowledgement cannot be persisted', async () => {
    let finish!: (response: Response) => void;
    const fetchMock = vi.fn().mockImplementation(() => new Promise<Response>(resolve => { finish = resolve; }));
    vi.stubGlobal('fetch', fetchMock);
    vi.mocked(getActions).mockImplementation(async () => [...actions]);
    vi.mocked(enqueueAction).mockImplementation(async action => { actions = [...actions, action]; });
    vi.mocked(completeAction).mockRejectedValue(new Error('Storage unavailable'));
    const manager = SyncManager.getInstance();
    const active = manager.sync();
    await vi.waitFor(() => expect(fetchMock).toHaveBeenCalledOnce());
    await manager.enqueue({ ...triage, id: 'triage-new' }, owner);
    finish(response('sale', '/api/v1/payments/terminal/sync_offline'));
    await active;
    await new Promise(resolve => setTimeout(resolve, 0));
    expect(fetchMock).toHaveBeenCalledOnce();
    expect(getActions).toHaveBeenCalledOnce();
    expect([...states.values()]).toEqual(['inflight']);
  });
});

it('forwards the expected view owner through the enqueueMutation alias', async () => {
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await SyncManager.getInstance().enqueueMutation({ id: 'owned', type: 'triage_action', timestamp: 1 }, owner);
  expect(enqueueAction).toHaveBeenCalledWith({ id: 'owned', type: 'triage_action', timestamp: 1 }, owner);
});

it('drains children atomically released by a completion receipt without replaying the parent', async () => {
  const parent = { id: 'field-parent', type: 'sync_event', timestamp: 1, payload: { entity_type: 'appointment', entity_id: 'job-1', action_type: 'UpdateStatus', payload: { status: 'Completed' } }, field_completion: { job_id: 'job-1', customer_id: 'customer-1', notes: '' } };
  const child = { id: 'field-child', type: 'generate_invoice', timestamp: 1, payload: { job_id: 'job-1', customer_id: 'customer-1' }, field_completion_parent_id: parent.id };
  actions = [parent];
  vi.mocked(getActions).mockImplementation(async () => [...actions]);
  vi.mocked(completeAction).mockImplementation(async (claim, status) => {
    states.set(claim.attemptToken, status);
    if (claim.action.id === parent.id && status === 'acknowledged') actions = [parent, child];
  });
  const transport = vi.fn(async (route: string) => response(route.includes('sync/events') ? parent.id : child.id, route));
  vi.stubGlobal('fetch', transport);
  await SyncManager.getInstance().sync();
  await vi.waitFor(() => expect(transport).toHaveBeenCalledTimes(2));
  await vi.waitFor(() => expect([...states.values()]).toEqual(['acknowledged', 'acknowledged']));
  await SyncManager.getInstance().sync(); expect(transport).toHaveBeenCalledTimes(2);
});
it('resumes only durable field completion work after a fresh application mount', async () => {
  vi.stubGlobal('fetch', vi.fn(async (route: string) => response('field-resume', route)));
  actions = [{ id: 'field-resume', type: 'generate_invoice', timestamp: 1, payload: { job_id: 'job-1', customer_id: 'customer-1' }, field_completion_parent_id: 'acknowledged-parent' }];
  await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).toHaveBeenCalledOnce();
  actions = [sale]; await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(fetch).toHaveBeenCalledOnce();
});

it.each(['pending', 'reconciliation'])('field-only mount recovery never drains unrelated work from a mixed queue (%s field child)', async state => {
  const child = { id: 'mixed-field', type: 'generate_invoice', timestamp: 1, payload: { job_id: 'job-1', customer_id: 'customer-1' }, field_completion_parent_id: 'acknowledged-parent' };
  actions = [sale, child];
  if (state === 'reconciliation') states.set(`${child.id}:/api/v1/invoices/generate`, state);
  vi.stubGlobal('fetch', vi.fn(async (route: string) => response(route.includes('invoices') ? child.id : sale.id, route)));
  await SyncManager.getInstance().resumeFieldCompletionWork();
  expect(vi.mocked(fetch).mock.calls.map(([url]) => url)).toEqual(state === 'pending' ? ['/api/v1/invoices/generate'] : []);
  expect(states.has(`${sale.id}:/api/v1/payments/terminal/sync_offline`)).toBe(false);
});
it('preserves an explicit new enqueue while a narrow mount recovery is in flight', async () => {
  const child = { id: 'active-field', type: 'generate_invoice', timestamp: 1, payload: { job_id: 'job-1', customer_id: 'customer-1' }, field_completion_parent_id: 'acknowledged-parent' };
  actions = [child];
  let finish!: (value: Response) => void;
  vi.mocked(getActions).mockImplementation(async () => [...actions]);
  vi.mocked(enqueueAction).mockImplementation(async action => { actions = [...actions, action]; });
  vi.stubGlobal('fetch', vi.fn((route: string) => route.includes('invoices') ? new Promise<Response>(resolve => { finish = resolve; }) : Promise.resolve(response('new-triage', route))));
  const manager = SyncManager.getInstance(); const recovering = manager.resumeFieldCompletionWork();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledOnce());
  await manager.enqueue({ ...triage, id: 'new-triage' }, owner);
  finish(response(child.id, '/api/v1/invoices/generate')); await recovering;
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
  expect(vi.mocked(fetch).mock.calls.map(([url]) => url)).toEqual(['/api/v1/invoices/generate', '/api/v1/ui/triage/action']);
});
