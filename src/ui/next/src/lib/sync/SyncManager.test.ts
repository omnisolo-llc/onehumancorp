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
});

it('forwards the expected view owner through the enqueueMutation alias', async () => {
  vi.spyOn(navigator, 'onLine', 'get').mockReturnValue(false);
  await SyncManager.getInstance().enqueueMutation({ id: 'owned', type: 'triage_action', timestamp: 1 }, owner);
  expect(enqueueAction).toHaveBeenCalledWith({ id: 'owned', type: 'triage_action', timestamp: 1 }, owner);
});
