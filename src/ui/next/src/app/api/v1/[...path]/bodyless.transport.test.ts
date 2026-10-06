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
const now = 1800000000, backend = vi.fn<typeof fetch>();
const path = ['agent-feed', 'owned-decision', 'decision'];
beforeEach(async () => {
  backend.mockReset().mockResolvedValue(Response.json({ decision_recorded: true }));
  dependencies = { config: parseAuthRuntimeConfig({ OMNISOLO_WEB_CANONICAL_ORIGIN: 'https://app.example.test', BACKEND_URL: 'https://backend.example.test' }),
    ring: await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'test', OMNISOLO_WEB_SESSION_SECRET: 'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I' }),
    now: () => now, fetchImpl: backend, timeoutMs: 1000, requestLimitBytes: 1024, responseLimitBytes: 1024 };
});
async function request(method: string, body?: string, expectedTenant = 'tenant-a') {
  const session = { version: 1 as const, iat: now, exp: now + 3600, accessToken: 'synthetic-test-bearer',
    user: { id: 'owner-a', username: 'owner', roles: ['ADMIN'], organizationId: 'tenant-a' } };
  const sealed = await sealSession(session, dependencies.ring, sessionCodecContext(dependencies.config), { now, backendExpiresAt: session.exp });
  const cookie = serializeSessionCookie(cookieForSession(dependencies.config, sealed, session.iat, session.exp)).split(';', 1)[0];
  return new Request('https://app.example.test/api/v1/' + path.join('/'), { method, headers: {
    cookie, origin: 'https://app.example.test', 'content-type': 'application/json',
    'x-ohc-expected-user': 'owner-a', 'x-ohc-expected-tenant': expectedTenant,
  }, ...(body === undefined ? {} : { body }) });
}
it.each(['GET', 'HEAD'] as const)('bodyless %s with a JSON media type authenticates without parsing an absent body', async method => {
  const routes = await import('./route');
  const response = await routes[method](await request(method), { params: Promise.resolve({ path }) });
  expect(response.status).toBe(200);
  expect(backend).toHaveBeenCalledTimes(1);
  const [url, init] = backend.mock.calls[0];
  expect(String(url)).toBe('https://backend.example.test/api/v1/agent-feed/owned-decision/decision');
  expect(init?.method).toBe(method); expect(init?.body).toBeUndefined();
  expect(new Headers(init?.headers).get('authorization')).toBe('Bearer synthetic-test-bearer');
  expect(new Headers(init?.headers).get('x-tenant-id')).toBe('tenant-a');
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});
it.each(['POST', 'PUT', 'PATCH', 'DELETE'] as const)('%s retains strict identity stripping and rejects malformed JSON', async method => {
  const routes = await import('./route');
  const response = await routes[method](await request(method, JSON.stringify({ state: 'APPROVED', tenant_id: 'forged', nested: { user_id: 'forged', text: 'kept' } })), { params: Promise.resolve({ path }) });
  expect(response.status).toBe(200);
  expect(JSON.parse(new TextDecoder().decode(backend.mock.calls[0][1]?.body as Uint8Array))).toEqual({ state: 'APPROVED', nested: { text: 'kept' } });
  backend.mockClear();
  expect((await routes[method](await request(method, '{invalid'), { params: Promise.resolve({ path }) })).status).toBe(400);
  expect(backend).not.toHaveBeenCalled();
});
it('the bodyless read still rejects a changed owner before contacting the backend', async () => {
  const { GET } = await import('./route');
  expect((await GET(await request('GET', undefined, 'foreign'), { params: Promise.resolve({ path }) })).status).toBe(409);
  expect(backend).not.toHaveBeenCalled();
});
