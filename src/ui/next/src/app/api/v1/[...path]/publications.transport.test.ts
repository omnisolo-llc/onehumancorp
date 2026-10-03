import { beforeEach, expect, it, vi } from 'vitest';
import { parseAuthRuntimeConfig } from '@/lib/auth/runtimeConfig';
import { parseSessionKeyRing } from '@/lib/auth/sessionKeys';
import { sealSession } from '@/lib/auth/sessionCodec';
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from '@/lib/auth/sessionCookie';
import type { BackendTransportDependencies, BackendRequestOptions } from '@/lib/auth/backendTransport';
let dependencies: BackendTransportDependencies;
vi.mock('@/lib/auth/backendTransport', async original => {
  const actual = await original<typeof import('@/lib/auth/backendTransport')>();
  return { ...actual, proxyBackendRequest: (request: Request, path: string, options: BackendRequestOptions) => actual.proxyAuthenticatedRequest(request, path, dependencies, options) };
});
const now = 1800000000; const backend = vi.fn<typeof fetch>();
const id = '10000000-0000-4000-8000-000000000001';
const payload = () => ({ operation_id: id, site_id: null, snapshot_encoding: 'jcs-rfc8785-v1', snapshot: { domain: null,
  pages: [{ path: '/', title: 'Reviewed public page', seo_metadata: { author: { user_id: 'Printed author label', tenant: 'Printed workspace label' } },
    blocks: [{ block_type: 'HeroBlock', content: { headline: 'Review\nCafé' }, sort_order: 0 }] }] } });
beforeEach(async () => {
  backend.mockReset().mockResolvedValue(Response.json({ status: 'pending' }, { status: 202 }));
  dependencies = { config: parseAuthRuntimeConfig({ OMNISOLO_WEB_CANONICAL_ORIGIN: 'https://app.example.test', BACKEND_URL: 'https://backend.example.test' }),
    ring: await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'test', OMNISOLO_WEB_SESSION_SECRET: 'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I' }), now: () => now,
    fetchImpl: backend, timeoutMs: 1000, requestLimitBytes: 1048576, responseLimitBytes: 2097152 };
});
async function request(path: string, method: string, body?: string, extra: Record<string, string> = {}) {
  const session = { version: 1 as const, iat: now, exp: now + 3600, accessToken: 'synthetic-publication-test', user: { id: 'owner-a', username: 'owner', roles: ['ADMIN'], organizationId: 'tenant-a' } };
  const sealed = await sealSession(session, dependencies.ring, sessionCodecContext(dependencies.config), { now, backendExpiresAt: session.exp });
  const cookie = serializeSessionCookie(cookieForSession(dependencies.config, sealed, session.iat, session.exp)).split(';', 1)[0];
  return new Request('https://app.example.test/api/v1/builder/publications' + path, { method, headers: { cookie, origin: 'https://app.example.test',
    ...(body === undefined ? {} : { 'content-type': 'application/json' }), 'x-ohc-expected-user': 'owner-a', 'x-ohc-expected-tenant': 'tenant-a', ...extra }, ...(body === undefined ? {} : { body }) });
}
async function proxy(path: string, method: string, body?: string, extra?: Record<string, string>) {
  const route = await import('./route');
  return route[method as 'POST' | 'GET' | 'DELETE'](await request(path, method, body, extra), { params: Promise.resolve({ path: ['builder', 'publications', ...path.split('?')[0].split('/').filter(Boolean)] }) });
}
it('preserves every approved content key and exact raw bytes while deriving authority only from the sealed session', async () => {
  const raw = JSON.stringify(payload(), null, 2);
  expect((await proxy('', 'POST', raw)).status).toBe(202);
  const [target, init] = backend.mock.calls[0];
  expect(String(target)).toBe('https://backend.example.test/api/v1/builder/publications');
  expect(new TextDecoder().decode(init?.body as Uint8Array)).toBe(raw);
  const headers = new Headers(init?.headers);
  expect(headers.get('authorization')).toBe('Bearer synthetic-publication-test');
  expect(headers.get('x-tenant-id')).toBe('tenant-a'); expect(headers.get('x-user-id')).toBe('owner-a'); expect(headers.has('cookie')).toBe(false);
});
it.each([
  () => JSON.stringify(payload()).replace('"operation_id":', '"operation_id":"20000000-0000-4000-8000-000000000002","operation_id":'),
  () => JSON.stringify(payload()).replace('"operation_id":', '"operation\\u005fid":"20000000-0000-4000-8000-000000000002","operation_id":'),
  () => JSON.stringify(payload()).replace('"headline":', '"headline":"Changed unseen","headline":'),
  () => JSON.stringify({ ...payload(), tenant_id: 'forged' }),
  () => JSON.stringify({ ...payload(), snapshot_encoding: 'legacy-sorted-json' }),
  () => JSON.stringify(payload()).replace('"domain":null', '"domain":null,/*comment*/'),
  () => JSON.stringify(payload()).slice(0, -1) + ',}',
])('rejects ambiguous raw JSON or unreviewed fields before backend I/O %#', async body => {
  expect((await proxy('', 'POST', body())).status).toBe(400); expect(backend).not.toHaveBeenCalled();
});
it.each([['', 'GET'], ['/not-a-uuid', 'DELETE'], ['/operations/' + id, 'POST'], ['/operations/' + id + '?tenant=forged', 'GET']])('rejects an unapproved publication operation %#', async (path, method) => {
  expect((await proxy(path, method, method === 'POST' ? JSON.stringify(payload()) : undefined)).status).toBeGreaterThanOrEqual(400); expect(backend).not.toHaveBeenCalled();
});
it.each([['/operations/' + id, 'GET'], ['/' + id, 'DELETE']])('forwards only the exact owner-bound read/revoke route %s', async (path, method) => {
  await proxy(path, method); expect(backend).toHaveBeenCalledOnce();
  expect(String(backend.mock.calls[0][0])).toBe('https://backend.example.test/api/v1/builder/publications' + path);
  expect(backend.mock.calls[0][1]?.body).toBeUndefined();
});
it('rejects a stale owner before reading or dispatching its publication', async () => {
  expect((await proxy('', 'POST', JSON.stringify(payload()), { 'x-ohc-expected-user': 'other' })).status).toBe(409); expect(backend).not.toHaveBeenCalled();
});
it('does not route an unsigned publication mutation to the backend', async () => {
  const { POST } = await import('./route');
  const response = await POST(new Request('https://app.example.test/api/v1/builder/publications', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(payload()) }), { params: Promise.resolve({ path: ['builder', 'publications'] }) });
  expect(response.status).toBe(401); expect(backend).not.toHaveBeenCalled();
});
it('allows the publication envelope above the generic1MiB cap while preserving its exact bytes', async () => {
  const raw = ' '.repeat(1048576) + JSON.stringify(payload());
  expect((await proxy('', 'POST', raw)).status).toBe(202);
  expect(new TextDecoder().decode(backend.mock.calls[0][1]?.body as Uint8Array)).toBe(raw);
});
it('rejects a raw envelope above2MiB before backend I/O', async () => {
  expect((await proxy('', 'POST', ' '.repeat(2097152) + JSON.stringify(payload()))).status).toBe(413);
  expect(backend).not.toHaveBeenCalled();
});
it('retains the canonical1MiB snapshot bound inside the larger wire envelope', async () => {
  const draft = payload(); draft.snapshot.pages[0].blocks[0].content.headline = 'x'.repeat(1048576);
  expect((await proxy('', 'POST', JSON.stringify(draft))).status).toBe(400); expect(backend).not.toHaveBeenCalled();
});
it('requires JSON for publication and forbids a body on version revocation', async () => {
  expect((await proxy('', 'POST', JSON.stringify(payload()), { 'content-type': 'text/plain' })).status).toBe(415);
  expect((await proxy('/' + id, 'DELETE', JSON.stringify({ tenant_id: 'forged' }))).status).toBe(400);
  expect(backend).not.toHaveBeenCalled();
});
it('does not apply a JSON body sanitizer to an empty owner receipt read', async () => {
  await proxy('/operations/' + id, 'GET', undefined, { 'content-type': 'application/json' });
  expect(backend).toHaveBeenCalledOnce(); expect(backend.mock.calls[0][1]?.body).toBeUndefined();
});
