import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { fetchForOwnedBusinessAction, openOnboardingSession } from './draftSession';

const owner = { userId: 'service-owner', tenantId: 'service-tenant' };
beforeEach(() => {
  localStorage.clear();
  notifyQueueIdentityChange();
  vi.stubGlobal('fetch', vi.fn(async (url: string) => Response.json(
    url.endsWith('/session-identity') ? { ...owner, expiresAt: Date.now() + 60_000 } : { success: true },
  )));
});
afterEach(() => vi.unstubAllGlobals());

it.each(['/api/v1/booking/services', '/api/v1/builder/generate', '/api/v1/builder/geo_score', '/api/v1/builder/auto_seo', '/api/v1/builder/publish_draft', '/api/v1/agents/hire'])(
  'sends %s through the verified owner boundary and acknowledges dispatch', async url => {
    const expected = await openOnboardingSession();
    const dispatched = vi.fn();
    await fetchForOwnedBusinessAction(url, { method: 'POST', body: '{"title":"Service"}', redirect: 'follow', credentials: 'include', cache: 'force-cache' }, expected, dispatched);
    const call = vi.mocked(fetch).mock.calls.find(([target]) => target === url);
    expect(call).toBeDefined();
    expect(call?.[1]).toMatchObject({ redirect: 'error', credentials: 'same-origin', cache: 'no-store' });
    const headers = new Headers(call?.[1]?.headers);
    expect(headers.get('x-ohc-expected-user')).toBe(owner.userId);
    expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
    expect(dispatched).toHaveBeenCalledOnce();
  },
);
it.each(['https://other.example/api/v1/booking/services', '/api/v1/booking/services?tenant=other', '/api/v1/onboarding/state', '/api/v1/agents/hire?tenant=other'])(
  'never sends owner data to unapproved destination %s', async url => {
    const expected = await openOnboardingSession();
    const count = vi.mocked(fetch).mock.calls.length;
    await expect(fetchForOwnedBusinessAction(url, { method: 'POST', body: 'private' }, expected)).rejects.toThrow('Invalid business action destination');
    expect(vi.mocked(fetch).mock.calls).toHaveLength(count);
  },
);
it('requires the explicit supported mutation method', async () => {
  const expected = await openOnboardingSession();
  const count = vi.mocked(fetch).mock.calls.length;
  await expect(fetchForOwnedBusinessAction('/api/v1/booking/services', { method: 'DELETE' }, expected)).rejects.toThrow('require POST');
  expect(vi.mocked(fetch).mock.calls).toHaveLength(count);
});
it('does not mark or send an action when owner revalidation fails', async () => {
  const expected = await openOnboardingSession();
  const dispatched = vi.fn();
  vi.mocked(fetch).mockResolvedValue(Response.json({ ...owner, tenantId: 'another', expiresAt: Date.now() + 60_000 }));
  await expect(fetchForOwnedBusinessAction('/api/v1/booking/services', { method: 'POST' }, expected, dispatched)).rejects.toThrow('session changed');
  expect(dispatched).not.toHaveBeenCalled();
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
});
it('a failed durable dispatch marker prevents the network mutation', async () => {
  const expected = await openOnboardingSession();
  await expect(fetchForOwnedBusinessAction('/api/v1/booking/services', { method: 'POST' }, expected, () => {
    throw new Error('Local pending marker could not be saved');
  })).rejects.toThrow('pending marker');
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
});
it('refuses an asynchronous marker before sending a non-idempotent mutation', async () => {
  const expected = await openOnboardingSession();
  await expect(fetchForOwnedBusinessAction('/api/v1/booking/services', { method: 'POST' }, expected, async () => { throw new Error('Async marker failed'); })).rejects.toThrow('synchronously');
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
});
it('fences owner invalidation raised by the dispatch marker', async () => {
  const expected = await openOnboardingSession();
  await expect(fetchForOwnedBusinessAction('/api/v1/booking/services', { method: 'POST' }, expected, () => notifyQueueIdentityChange())).rejects.toThrow('not sent');
  expect(vi.mocked(fetch).mock.calls.every(([url]) => String(url).endsWith('/session-identity'))).toBe(true);
});
