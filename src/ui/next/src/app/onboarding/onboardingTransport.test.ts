import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fetchForOnboardingOwner, openOnboardingSession } from './draftSession';
import { hasOnboardingWriteFence } from './draftWriteGate';
import { installOnboardingLocks } from './testLocks';

const owner = { userId: 'transport-owner', tenantId: 'transport-tenant' };
beforeEach(() => {
  installOnboardingLocks(); localStorage.clear(); notifyQueueIdentityChange();
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(
    url.endsWith('/session-identity') ? { ...owner, expiresAt: Date.now() + 60_000 } : { success: true },
  )));
});
afterEach(() => vi.unstubAllGlobals());

it('never permits an onboarding request to follow a redirect or override session/cache policy', async () => {
  const expected = await openOnboardingSession();
  await fetchForOnboardingOwner('/api/v1/onboarding/start', {
    method: 'POST', redirect: 'follow', credentials: 'include', cache: 'force-cache', body: '{"private":"draft"}',
  }, expected);
  const call = vi.mocked(fetch).mock.calls.find(([url]) => url === '/api/v1/onboarding/start');
  expect(call?.[1]).toMatchObject({ redirect: 'error', credentials: 'same-origin', cache: 'no-store' });
  expect(new Headers(call?.[1]?.headers).get('x-ohc-expected-tenant')).toBe(owner.tenantId);
});
it('retains the unknown-outcome fence when transport rejects a redirect after dispatch', async () => {
  const expected = await openOnboardingSession();
  vi.mocked(fetch).mockImplementation(async url => {
    if (String(url).endsWith('/session-identity')) return Response.json({ ...owner, expiresAt: Date.now() + 60_000 });
    throw new TypeError('Redirect blocked');
  });
  await expect(fetchForOnboardingOwner('/api/v1/onboarding/draft', { method: 'POST', body: '{}' }, expected)).rejects.toThrow('Redirect blocked');
  expect(hasOnboardingWriteFence(expected)).toBe(true);
  const sent = vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/onboarding/draft').length;
  await expect(fetchForOnboardingOwner('/api/v1/onboarding/draft', { method: 'POST', body: '{}' }, expected)).rejects.toThrow('could not be confirmed');
  expect(vi.mocked(fetch).mock.calls.filter(([url]) => url === '/api/v1/onboarding/draft')).toHaveLength(sent);
});
