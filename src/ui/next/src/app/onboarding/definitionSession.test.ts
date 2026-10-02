import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';
import { fetchForOwnedDefinition, onboardingOwner, openOnboardingSession } from './draftSession';
vi.mock('@/lib/sync/queueIdentity', async original => ({ ...await original<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn() }));
const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
const id = '10000000-0000-4000-8000-000000000001';
const root = '/api/v1/agents/definitions';
beforeEach(async () => {
  localStorage.clear(); notifyQueueIdentityChange(); vi.mocked(readQueueOwner).mockResolvedValue(owner);
  await openOnboardingSession(); vi.stubGlobal('fetch', vi.fn(async () => Response.json({})));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });
it.each([[root + '?q=writer&limit=50', 'GET'], [root, 'POST'], [root + '/' + id + '/install', 'POST'], [root + '/operations/' + id, 'GET']])('uses the shared sealed owner boundary for %s', async (url, method) => {
  await fetchForOwnedDefinition(url, { method, redirect: 'follow', credentials: 'include', cache: 'force-cache' }, owner);
  expect(fetch).toHaveBeenCalledWith(url, expect.objectContaining({ method, redirect: 'error', credentials: 'same-origin', cache: 'no-store' }));
  const headers = new Headers(vi.mocked(fetch).mock.calls[0][1]?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(owner.userId); expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
});
it.each([
  ['https://outside.example/api/v1/agents/definitions', 'POST'], [root + '/' + id + '/install', 'GET'],
  [root + '/operations/' + id, 'POST'], [root + '/../../payments', 'POST'], [root + '?tenant=other', 'GET'],
  [root + '?limit=101', 'GET'], [root + '?cursor=a&cursor=b', 'GET'], [root, 'DELETE'],
])('rejects an unowned destination or method %#', async (url, method) => {
  await expect(fetchForOwnedDefinition(url, { method }, owner)).rejects.toThrow(); expect(fetch).not.toHaveBeenCalled();
});
it('persists its synchronous dispatch marker only after fresh identity verification', async () => {
  const steps: string[] = []; vi.mocked(readQueueOwner).mockImplementation(async () => { steps.push('identity'); return owner; });
  vi.mocked(fetch).mockImplementation(async () => { steps.push('fetch'); return Response.json({}); });
  await fetchForOwnedDefinition(root, { method: 'POST' }, owner, () => { steps.push('persist'); });
  expect(steps).toEqual(['identity', 'persist', 'fetch']);
});
it('does not dispatch when identity switches or marker storage fails', async () => {
  const marker = vi.fn(() => { throw new Error('Storage unavailable'); });
  await expect(fetchForOwnedDefinition(root, { method: 'POST' }, owner, marker)).rejects.toThrow('Storage unavailable');
  expect(fetch).not.toHaveBeenCalled();
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'other', tenantId: 'other' });
  await expect(fetchForOwnedDefinition(root, { method: 'POST' }, owner, marker)).rejects.toThrow();
  expect(marker).toHaveBeenCalledOnce(); expect(fetch).not.toHaveBeenCalled();
});
it('preserves a verified account on the exact role rejection and invalidates a mismatched owner', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ success: false, reason: 'owner_or_admin_required' }, { status: 403 }));
  expect((await fetchForOwnedDefinition(root, { method: 'POST' }, owner)).status).toBe(403);
  expect(onboardingOwner()).toEqual(owner);
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ success: false, reason: 'session_identity_changed' }, { status: 409 }));
  await fetchForOwnedDefinition(root, { method: 'POST' }, owner);
  expect(onboardingOwner()).toBeNull();
});
