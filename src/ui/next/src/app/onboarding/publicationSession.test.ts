import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange, readQueueOwner } from '@/lib/sync/queueIdentity';
import * as owned from './draftSession';

vi.mock('@/lib/sync/queueIdentity', async original => ({ ...await original<typeof import('@/lib/sync/queueIdentity')>(), readQueueOwner: vi.fn() }));
const owner = { userId: 'publisher-a', tenantId: 'workspace-a' };
const id = '10000000-0000-4000-8000-000000000001';
const root = '/api/v1/builder/publications';
const fetchPublication = owned.fetchForOwnedPublication;
beforeEach(async () => {
  localStorage.clear(); notifyQueueIdentityChange(); vi.mocked(readQueueOwner).mockResolvedValue({ ...owner });
  await owned.openOnboardingSession(); vi.stubGlobal('fetch', vi.fn(async () => Response.json({})));
});
afterEach(() => { vi.unstubAllGlobals(); vi.restoreAllMocks(); });

it.each([[root, 'POST'], [root + '/operations/' + id, 'GET'], [root + '/' + id, 'DELETE']])('binds the real publication operation %s to the verified owner', async (url, method) => {
  await fetchPublication(url, { method, redirect: 'follow', credentials: 'include', cache: 'force-cache' }, owner);
  expect(fetch).toHaveBeenCalledWith(url, expect.objectContaining({ method, redirect: 'error', credentials: 'same-origin', cache: 'no-store' }));
  const headers = new Headers(vi.mocked(fetch).mock.calls[0][1]?.headers);
  expect(headers.get('x-ohc-expected-user')).toBe(owner.userId);
  expect(headers.get('x-ohc-expected-tenant')).toBe(owner.tenantId);
  expect(headers.has('content-type')).toBe(false);
});
it.each([
  ['https://outside.example' + root, 'POST'], ['//outside.example' + root, 'POST'],
  ['/\n/outside.example' + root, 'POST'], [root + '?tenant=other', 'POST'],
  [root + '/operations/' + id, 'POST'], [root + '/' + id, 'GET'], [root, 'GET'],
  [root + '/operations/not-a-uuid', 'GET'], [root + '/' + id + '#fragment', 'DELETE'],
  [root + '/../../payments', 'POST'], [root + '/operations/' + id + '\n', 'GET'],
])('rejects an unapproved publication destination %# before I/O', async (url, method) => {
  await expect(async () => fetchPublication(url, { method }, owner)).rejects.toThrow();
  expect(fetch).not.toHaveBeenCalled();
});
it('persists the dispatch marker after a fresh owner read and before the mutation', async () => {
  const steps: string[] = [];
  vi.mocked(readQueueOwner).mockImplementation(async () => { steps.push('verify'); return { ...owner }; });
  vi.mocked(fetch).mockImplementation(async () => { steps.push('post'); return Response.json({}); });
  await fetchPublication(root, { method: 'POST' }, owner, () => { steps.push('persist'); });
  expect(steps).toEqual(['verify', 'persist', 'post']);
});
it('does not dispatch when marker storage fails or the owner changes during verification', async () => {
  await expect(async () => fetchPublication(root, { method: 'POST' }, owner, () => { throw new Error('Storage unavailable'); })).rejects.toThrow('Storage unavailable');
  expect(fetch).not.toHaveBeenCalled();
  vi.mocked(readQueueOwner).mockResolvedValue({ userId: 'other', tenantId: 'other' });
  await expect(async () => fetchPublication(root, { method: 'POST' }, owner)).rejects.toThrow();
  expect(fetch).not.toHaveBeenCalled();
});
it('retires access on an exact owner contradiction while preserving ordinary conflict semantics', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: 'operation_conflict' }, { status: 409 }));
  expect((await fetchPublication(root, { method: 'POST' }, owner)).status).toBe(409);
  expect(owned.onboardingOwner()).toEqual(owner);
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: 'session_identity_changed' }, { status: 409 }));
  await fetchPublication(root, { method: 'POST' }, owner);
  expect(owned.onboardingOwner()).toBeNull();
});
it('does not trust a403 whose raw effect keys contradict one another', async () => {
  vi.mocked(fetch).mockResolvedValue(new Response('{"schema_version":1,"error":"publication_forbidden","effect":"unknown","effect":"none","message":"Rejected"}', { status: 403, headers: { 'content-type': 'application/json' } }));
  await fetchPublication(root, { method: 'POST' }, owner);
  expect(owned.onboardingOwner()).toBeNull();
});
it('keeps a verified session for the exact pre-effect role denial so its fresh rejection can be recorded', async () => {
  vi.mocked(fetch).mockResolvedValue(Response.json({ schema_version: 1, error: 'publication_forbidden', effect: 'none', message: 'Current authority required' }, { status: 403 }));
  expect((await fetchPublication(root, { method: 'POST' }, owner)).status).toBe(403);
  expect(owned.onboardingOwner()).toEqual(owner);
});
it.each([
  { schema_version: 1, error: 'publication_forbidden', effect: 'unknown', message: 'Unknown' },
  { schema_version: 2, error: 'publication_forbidden', effect: 'none', message: 'Unrecognized' },
  { error: 'Forbidden' },
])('retires the session for an unverified403 envelope %#', async body => {
  vi.mocked(fetch).mockResolvedValue(Response.json(body, { status: 403 }));
  await fetchPublication(root, { method: 'POST' }, owner);
  expect(owned.onboardingOwner()).toBeNull();
});
