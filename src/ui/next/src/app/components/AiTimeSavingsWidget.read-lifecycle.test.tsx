import { act, cleanup, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { invalidateQueueOwner, readQueueOwner, notifyQueueIdentityChange, QUEUE_IDENTITY_EPOCH_KEY, queueIdentityGeneration, hasVerifiedOfflineQueueOwner, subscribeQueueIdentityReadiness } from '@/lib/sync/queueIdentity';
import Savings from '@/app/components/AiTimeSavingsWidget';
import { recordSmokeHttpResponse } from '../../../../../e2e/support/hosted_voice_policy';

const owner = { userId: 'actual-owner', tenantId: 'actual-tenant' };
const origin = 'http://127.0.0.1:37415';
const url = origin + '/api/v1/growth/time-savings';
const unavailable = { success: false, code: 'capability_unavailable', capability: 'measured_time_savings', message: 'No verified result is available. This capability is not implemented; no action was completed.' };
function deferred<T>() { let resolve!: (value: T) => void; let reject!: (error: unknown) => void; const promise = new Promise<T>((yes, no) => { resolve = yes; reject = no; }); return { promise, resolve, reject }; }
beforeEach(() => {
  localStorage.clear(); act(() => invalidateQueueOwner());
  Object.defineProperty(navigator, 'onLine', { configurable: true, value: true });
});
afterEach(() => { cleanup(); act(() => invalidateQueueOwner()); vi.unstubAllGlobals(); vi.restoreAllMocks(); vi.useRealTimers(); });

async function scenario(revalidate: boolean) {
  const results = { failures: [] as string[], httpFailures: [] as string[], verifiedPolicyUrls: new Set<string>(), policyChecks: [] as Promise<void>[] };
  const pendingIdentity = deferred<Response>();
  const firstBody = deferred<unknown>();
  let identityReads = 0, savingsReads = 0, firstSignal: AbortSignal | undefined;
  const events: string[] = [];
  vi.stubGlobal('fetch', vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
    if (input === '/api/v1/auth/session-identity') {
      ++identityReads; events.push('identity:' + identityReads);
      return identityReads === 1 ? Response.json({ ...owner, expiresAt: Date.now() + 60000 }) : pendingIdentity.promise;
    }
    if (input === '/api/v1/billing/my-plan') return Response.json({ current_plan: 'Pro' });
    if (input !== '/api/v1/growth/time-savings') throw new Error('Unexpected request: ' + input);
    ++savingsReads;
    const count = savingsReads;
    const headers = Object.fromEntries(new Headers(init?.headers).entries());
    expect(headers).toEqual({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': owner.tenantId });
    expect(init?.credentials).toBe('same-origin');
    const signal = init?.signal as AbortSignal;
    if (count === 1) {
      firstSignal = signal;
      signal.addEventListener('abort', () => { events.push('savings:1:abort'); firstBody.reject(new DOMException('The operation was aborted', 'AbortError')); }, { once: true });
    }
    events.push('savings:' + count + ':headers:501');
    const body = count === 1 ? firstBody.promise : Promise.resolve(unavailable);
    const response = {
      status: () => 501, url: () => url,
      request: () => ({ method: () => init?.method ?? 'GET', headers: () => headers }),
      json: () => body,
    };
    recordSmokeHttpResponse(response, origin, results, owner);
    return { status: 501, json: () => body } as Response;
  }));
  render(<Savings />);
  await waitFor(() => expect(savingsReads).toBe(1));
  expect(firstSignal?.aborted).toBe(false);
  expect(screen.getByText('Loading recorded time-savings data…')).toBeVisible();
  let verification: Promise<unknown> | undefined;
  if (revalidate) {
    act(() => { verification = readQueueOwner(); });
    expect(screen.getByText('Verify your account to read time-savings data.')).toBeVisible();
    expect(screen.queryByText(/Recorded estimate:/)).not.toBeInTheDocument();
  }
  const abortedDuringRecheck = firstSignal?.aborted;
  await act(async () => { firstBody.resolve(unavailable); await Promise.all(results.policyChecks); });
  if (revalidate) await act(async () => {
    pendingIdentity.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60000 }));
    await verification;
  });
  await screen.findByText('Recorded time-savings data is unavailable.');
  await Promise.all(results.policyChecks);
  return { results, events, savingsReads, abortedDuringRecheck };
}
it('control retains a complete exact receipt without revalidation', async () => {
  const observed = await scenario(false);
  expect(observed.results.failures).toEqual([]);
  expect([...observed.results.verifiedPolicyUrls]).toEqual([url]);
  expect(observed.savingsReads).toBe(1);
});
it('same-owner revalidation preserves a started exact receipt while keeping private UI held', async () => {
  const observed = await scenario(true);
  console.log(JSON.stringify({ events: observed.events, abortedDuringRecheck: observed.abortedDuringRecheck, failures: observed.results.failures, verifiedUrls: [...observed.results.verifiedPolicyUrls] }));
  expect(observed.abortedDuringRecheck).toBe(false);
  expect(observed.results.failures).toEqual([]);
  expect([...observed.results.verifiedPolicyUrls]).toEqual([url]);
});


async function pendingFixture({ lifetime = 60000, delayedPlan = false, fakeClock = false } = {}) {
  if (fakeClock) vi.useFakeTimers({ toFake: ['Date', 'setTimeout', 'clearTimeout'] });
  const body = deferred<unknown>(), headers = deferred<Response>(), plan = deferred<Response>();
  let savingsReads = 0;
  let identityReply = async () => Response.json({ ...owner, expiresAt: Date.now() + lifetime });
  let signal!: AbortSignal;
  const fetcher = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
    if (input === '/api/v1/auth/session-identity') return identityReply();
    if (input === '/api/v1/billing/my-plan') return delayedPlan ? plan.promise : Response.json({ current_plan: 'Pro' });
    if (input !== '/api/v1/growth/time-savings') throw new Error('Unexpected request');
    savingsReads += 1;
    if (savingsReads > 1) return Response.json({ hours_saved: 4 });
    signal = init?.signal as AbortSignal;
    return headers.promise;
  });
  vi.stubGlobal('fetch', fetcher);
  let view!: ReturnType<typeof render>;
  await act(async () => { view = render(<Savings />); });
  expect(signal).toBeDefined();
  const releaseHeaders = async (status = 200) => {
    await act(async () => { headers.resolve({ status, json: () => body.promise } as Response); });
  };
  const beginRecheck = () => {
    const response = deferred<Response>(); identityReply = () => response.promise;
    let verification!: Promise<unknown>;
    act(() => { verification = readQueueOwner().catch(error => error); });
    return { response, verification, finish: async (status = 200, identity = owner, expiresAt = Date.now() + 60000) => {
      await act(async () => { response.resolve(Response.json({ ...identity, expiresAt }, { status })); await verification; });
    } };
  };
  return { view, body, signal, fetcher, plan, releaseHeaders, beginRecheck, setIdentityReply: (reply: () => Promise<Response>) => { identityReply = reply; } };
}

it('holds a completed200 body during recheck, then displays it once under the same original owner', async () => {
  const f = await pendingFixture(); await f.releaseHeaders();
  const check = f.beginRecheck();
  await act(async () => f.body.resolve({ hours_saved: 7 }));
  expect(f.signal.aborted).toBe(false);
  expect(screen.queryByText(/7 hours saved/)).not.toBeInTheDocument();
  await check.finish();
  expect(await screen.findByText('Recorded estimate: 7 hours saved.')).toBeVisible();
  expect(f.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(1);
});

for (const retire of ['generation', 'auth', 'storage', 'different-owner', 'failed-check', 'unmount', 'local-plan-denial'] as const) {
  it(`aborts immediately on ${retire} and ignores a late private body`, async () => {
    const f = await pendingFixture({ delayedPlan: retire === 'local-plan-denial' }); await f.releaseHeaders();
    const check = f.beginRecheck();
    if (retire === 'generation') act(() => invalidateQueueOwner());
    if (retire === 'auth') act(() => notifyQueueIdentityChange());
    if (retire === 'storage') act(() => { localStorage.setItem(QUEUE_IDENTITY_EPOCH_KEY, 'changed'); window.dispatchEvent(new StorageEvent('storage', { key: QUEUE_IDENTITY_EPOCH_KEY })); });
    if (retire === 'different-owner') await check.finish(200, { userId: 'other', tenantId: 'other' });
    if (retire === 'failed-check') await check.finish(401);
    if (retire === 'unmount') f.view.unmount();
    if (retire === 'local-plan-denial') await act(async () => f.plan.resolve(Response.json({ error: 'denied' }, { status: 401 })));
    expect(f.signal.aborted).toBe(true);
    await act(async () => f.body.resolve({ hours_saved: 999 }));
    expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
    if (!['different-owner', 'failed-check'].includes(retire)) await check.finish();
    expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  });
}

it('aborts at its original expiry while identity verification is stalled', async () => {
  const f = await pendingFixture({ fakeClock: true }); await f.releaseHeaders();
  const check = f.beginRecheck();
  await act(async () => { await vi.advanceTimersByTimeAsync(60001); });
  expect(f.signal.aborted).toBe(true);
  await act(async () => f.body.resolve({ hours_saved: 999 }));
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  await check.finish(200, owner, Date.now() + 60000);
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  expect(f.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(2);
  expect(screen.getByText('Recorded estimate: 4 hours saved.')).toBeVisible();
});

for (const status of [401, 403, 409]) {
  it(`invalidates current identity immediately for savings${status}, even while a recheck is pending`, async () => {
    const f = await pendingFixture();
    const check = f.beginRecheck();
    const before = queueIdentityGeneration();
    await f.releaseHeaders(status);
    if (status === 409) await act(async () => f.body.resolve({ error: 'session_identity_changed' }));
    expect(queueIdentityGeneration()).toBeGreaterThan(before);
    expect(f.signal.aborted).toBe(true);
    await check.finish();
    expect(screen.queryByText(/Recorded estimate:/)).not.toBeInTheDocument();
  });
}

it('does not extend the original savings lifetime when a same-owner check returns a longer lease', async () => {
  const f = await pendingFixture({ fakeClock: true }); await f.releaseHeaders();
  const check = f.beginRecheck(); await check.finish(200, owner, Date.now() + 120000);
  expect(f.signal.aborted).toBe(false);
  await act(async () => { await vi.advanceTimersByTimeAsync(60001); });
  expect(f.signal.aborted).toBe(true);
  await act(async () => f.body.resolve({ hours_saved: 999 }));
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
});

it('withholds metrics immediately when original expiry passes before its timer can run', async () => {
  const f = await pendingFixture(); await f.releaseHeaders();
  await act(async () => f.body.resolve({ hours_saved: 7 }));
  expect(await screen.findByText('Recorded estimate: 7 hours saved.')).toBeVisible();
  const now = Date.now(); const clock = vi.spyOn(Date, 'now').mockReturnValue(now + 120000);
  f.view.rerender(<Savings />);
  expect(screen.queryByText(/7 hours saved/)).not.toBeInTheDocument();
  clock.mockRestore();
});


it('holds a completed body when a readiness listener starts a second same-owner recheck', async () => {
  const f = await pendingFixture(); await f.releaseHeaders();
  const first = f.beginRecheck(); const second = deferred<Response>();
  f.setIdentityReply(() => second.promise);
  await act(async () => f.body.resolve({ hours_saved: 7 }));
  let armed = true, nested: Promise<unknown> | undefined;
  const unsubscribe = subscribeQueueIdentityReadiness(() => {
    if (armed && hasVerifiedOfflineQueueOwner(owner)) { armed = false; nested = readQueueOwner().catch(error => error); }
  });
  await first.finish();
  expect(nested).toBeDefined();
  expect(f.signal.aborted).toBe(false);
  expect(screen.queryByText(/7 hours saved/)).not.toBeInTheDocument();
  await act(async () => { second.resolve(Response.json({ ...owner, expiresAt: Date.now() + 60000 })); await nested; });
  expect(await screen.findByText('Recorded estimate: 7 hours saved.')).toBeVisible();
  expect(f.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(1);
  unsubscribe();
});

it('fails closed without throwing when original storage authority cannot be read', async () => {
  const f = await pendingFixture(); await f.releaseHeaders();
  const getter = vi.spyOn(localStorage, 'getItem').mockImplementation(() => { throw new Error('Storage unavailable'); });
  await act(async () => f.body.resolve({ hours_saved: 999 }));
  expect(f.signal.aborted).toBe(true);
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  getter.mockRestore();
});

for (const status of [401, 403, 409]) {
  it(`ignores a late retired savings${status} without invalidating a subsequently verified owner`, async () => {
    const f = await pendingFixture();
    act(() => notifyQueueIdentityChange());
    expect(f.signal.aborted).toBe(true);
    const check = f.beginRecheck(); await check.finish(200, { userId: 'new-user', tenantId: 'new-tenant' });
    const before = queueIdentityGeneration();
    await f.releaseHeaders(status);
    if (status === 409) await act(async () => f.body.resolve({ error: 'session_identity_changed' }));
    expect(queueIdentityGeneration()).toBe(before);
    expect(screen.queryByText(/Recorded estimate:/)).not.toBeInTheDocument();
  });
}


it('starts a fresh read after navigation instead of reviving an expired body', async () => {
  const first = await pendingFixture({ fakeClock: true }); await first.releaseHeaders();
  const check = first.beginRecheck();
  await act(async () => { await vi.advanceTimersByTimeAsync(60001); });
  expect(first.signal.aborted).toBe(true);
  await check.finish(200, owner, Date.now() + 60000);
  expect(first.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(2);
  first.view.unmount();
  const next = await pendingFixture(); await next.releaseHeaders();
  await act(async () => first.body.resolve({ hours_saved: 999 }));
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  await act(async () => next.body.resolve({ hours_saved: 4 }));
  expect(screen.getByText('Recorded estimate: 4 hours saved.')).toBeVisible();
  expect(next.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(1);
});


it('recovers after failed verification through a fresh canonical transition without reviving old data', async () => {
  const f = await pendingFixture(); await f.releaseHeaders();
  const failed = f.beginRecheck(); await failed.finish(401);
  expect(f.signal.aborted).toBe(true);
  await act(async () => f.body.resolve({ hours_saved: 999 }));
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  const recovered = f.beginRecheck(); await recovered.finish();
  expect(await screen.findByText('Recorded estimate: 4 hours saved.')).toBeVisible();
  expect(screen.queryByText(/999 hours saved/)).not.toBeInTheDocument();
  expect(f.fetcher.mock.calls.filter(([url]) => url === '/api/v1/growth/time-savings')).toHaveLength(2);
});
