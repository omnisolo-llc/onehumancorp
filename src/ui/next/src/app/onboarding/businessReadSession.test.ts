import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fetchForOwnedBusinessRead, onboardingOwner, openOnboardingSession } from './draftSession';

const owner = { userId: 'kitchen-owner', tenantId: 'kitchen-tenant' };
const identity = () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
beforeEach(() => {
  localStorage.clear();
  notifyQueueIdentityChange();
  vi.stubGlobal('fetch', vi.fn(async (url: string) => url.endsWith('/session-identity') ? identity() : Response.json([])));
});
afterEach(() => vi.unstubAllGlobals());

it('reads only supported owned business snapshots with exact verified owner headers and private transport', async () => {
  const expected = await openOnboardingSession();
  for (const url of ['/api/v1/pos/orders', '/api/v1/pos/inventory', '/api/v1/agents/execution-policy', '/api/v1/agents/workflows', '/api/v1/agents/approvals', '/api/v1/agents/approvals/activity']) {
    await fetchForOwnedBusinessRead(url, expected);
    const call = vi.mocked(fetch).mock.calls.find(([target]) => target === url);
    expect(call?.[1]).toMatchObject({ method: 'GET', credentials: 'same-origin', cache: 'no-store', redirect: 'error' });
    const headers = new Headers(call?.[1]?.headers);
    expect(headers.get('x-ohc-expected-user')).toBe(owner.userId);
    expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  }
});
it('rejects external, query-bearing and unsupported destinations without sending private data', async () => {
  const expected = await openOnboardingSession();
  const before = vi.mocked(fetch).mock.calls.length;
  for (const url of ['https://other.example/api/v1/pos/orders', '//other.example/api/v1/pos/orders', '/api/v1/pos/orders?tenant=other', '/api/v1/onboarding/state', '/api/v1/agents/execution-policy?tenant=other']) {
    await expect(fetchForOwnedBusinessRead(url, expected)).rejects.toThrow('Invalid business read destination');
  }
  expect(fetch).toHaveBeenCalledTimes(before);
});
it('requires the canonical current owner before reading any POS data', async () => {
  await openOnboardingSession();
  const before = vi.mocked(fetch).mock.calls.length;
  await expect(fetchForOwnedBusinessRead('/api/v1/pos/orders', null)).rejects.toThrow('session changed');
  await expect(fetchForOwnedBusinessRead('/api/v1/pos/orders', { ...owner, tenantId: 'another' })).rejects.toThrow('session changed');
  expect(fetch).toHaveBeenCalledTimes(before);
});
it('freezes the expected owner across delayed identity verification', async () => {
  const expected = await openOnboardingSession();
  let verified!: (response: Response) => void;
  vi.mocked(fetch).mockImplementationOnce(() => new Promise(resolve => { verified = resolve; }));
  const reading = fetchForOwnedBusinessRead('/api/v1/pos/orders', expected);
  expected.userId = 'changed'; expected.tenantId = 'changed';
  verified(identity());
  expect((await reading).ok).toBe(true);
  const call = vi.mocked(fetch).mock.calls.find(([target]) => target === '/api/v1/pos/orders');
  expect(new Headers(call?.[1]?.headers).get('x-ohc-expected-user')).toBe(owner.userId);
  expect(new Headers(call?.[1]?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
});
it('retires a held read when the canonical session is invalidated during verification', async () => {
  const expected = await openOnboardingSession();
  let verified!: (response: Response) => void;
  vi.mocked(fetch).mockImplementationOnce(() => new Promise(resolve => { verified = resolve; }));
  const reading = fetchForOwnedBusinessRead('/api/v1/pos/orders', expected);
  const rejected = expect(reading).rejects.toThrow(/identity|session/);
  notifyQueueIdentityChange(); verified(identity());
  await rejected;
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
  expect(onboardingOwner()).toBeNull();
});
it('invalidates owner state when the protected data endpoint rejects authentication', async () => {
  for (const status of [401, 403]) {
    const expected = await openOnboardingSession();
    vi.mocked(fetch).mockImplementationOnce(async () => identity()).mockResolvedValueOnce(new Response('{}', { status }));
    expect((await fetchForOwnedBusinessRead('/api/v1/pos/orders', expected)).status).toBe(status);
    expect(onboardingOwner()).toBeNull();
  }
});
