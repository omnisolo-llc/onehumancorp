import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { parseAuthRuntimeConfig } from '@/lib/auth/runtimeConfig';
import { parseSessionKeyRing } from '@/lib/auth/sessionKeys';
import { sealSession } from '@/lib/auth/sessionCodec';
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from '@/lib/auth/sessionCookie';
import type { BackendTransportDependencies } from '@/lib/auth/backendTransport';

const holder = vi.hoisted(() => ({ dependencies: undefined as unknown }));
vi.mock('@/lib/auth/backendTransport', async original => {
  const real = await original<typeof import('@/lib/auth/backendTransport')>();
  return { ...real, proxyBackendRequest: (request: Request, path: string, options: object) => real.proxyAuthenticatedRequest(request, path, holder.dependencies as BackendTransportDependencies, options) };
});
let backend: ReturnType<typeof vi.fn<typeof fetch>>;
let cookie: string;
let GET: typeof import('./route').GET;
const path = '/api/v1/staff/timecard/receipts/clock-_1';
beforeEach(async () => {
  const config = parseAuthRuntimeConfig({ OMNISOLO_WEB_CANONICAL_ORIGIN: 'https://app.example.com', BACKEND_URL: 'https://backend.example.com' });
  const ring = await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'test', OMNISOLO_WEB_SESSION_SECRET: 'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I' });
  const now = 1_800_000_000;
  backend = vi.fn<typeof fetch>(async () => Response.json({ success: true, outcomes: [{ id: 'clock-_1', route: '/api/v1/staff/timecard', status: 'acknowledged' }], receipt: { version: 1, actor_id: 'a', id: 'clock-_1', staff_id: 'a', event_type: 'CLOCK_IN', offline_timestamp: '1970-01-01T00:00:00.001Z' } }));
  holder.dependencies = { config, ring, now: () => now, fetchImpl: backend, timeoutMs: 1000, requestLimitBytes: 1024, responseLimitBytes: 4096 };
  const session = { version: 1 as const, iat: now, exp: now + 60, accessToken: 'sealed.server.token', user: { id: 'a', username: 'A', roles: ['OWNER'], organizationId: 't' } };
  const compact = await sealSession(session, ring, sessionCodecContext(config), { now, backendExpiresAt: session.exp });
  cookie = serializeSessionCookie(cookieForSession(config, compact, now, session.exp)).split(';')[0];
  ({ GET } = await import('./route'));
});
afterEach(() => vi.restoreAllMocks());
const request = (target = path, headers: Record<string, string> = {}) => new Request(`https://app.example.com${target}`, { headers: { cookie, 'x-ohc-expected-user': 'a', 'x-ohc-expected-tenant': 't', ...headers } });
it('performs a no-store GET through sealed-session transport for the exact receipt path', async () => {
  const response = await GET(request(`${path}?tenant_id=foreign&user_id=foreign`, { authorization: 'Bearer browser.forged', 'x-tenant-id': 'foreign' }));
  expect(response.status).toBe(200); expect(response.headers.get('cache-control')).toBe('private, no-store');
  expect(await response.json()).toMatchObject({ receipt: { actor_id: 'a', id: 'clock-_1' } });
  expect(backend).toHaveBeenCalledOnce();
  const [url, init] = backend.mock.calls[0];
  expect(String(url)).toBe(`https://backend.example.com${path}`);
  expect(init).toMatchObject({ method: 'GET', cache: 'no-store', redirect: 'manual' });
  expect(init?.body).toBeUndefined();
  const forwarded = new Headers(init?.headers);
  expect(forwarded.get('authorization')).toBe('Bearer sealed.server.token'); expect(forwarded.get('x-tenant-id')).toBe('t');
});
it.each([{ 'x-ohc-expected-user': 'b' }, { 'x-ohc-expected-tenant': 'other' }])('holds owner/session mismatch without backend I/O', async headers => {
  expect((await GET(request(path, headers))).status).toBe(409); expect(backend).not.toHaveBeenCalled();
});
it('rejects missing sealed authentication despite forged browser identity', async () => {
  expect((await GET(request(path, { cookie: '', authorization: 'Bearer forged' }))).status).toBe(401); expect(backend).not.toHaveBeenCalled();
});
it.each(['%72', '%2D', '%5F', 'a%2Fb', 'a%252Fb', '%2e%2e', 'a.b', 'a%20b', '%C3%A9', 'a'.repeat(129), 'a/extra'])('rejects ambiguous or unsafe receipt path %s before transport', async id => {
  // A raw URL property avoids Request normalizing dot segments before the handler.
  const raw = request(); Object.defineProperty(raw, 'url', { value: `https://app.example.com/api/v1/staff/timecard/receipts/${id}` });
  expect((await GET(raw)).status).toBe(400); expect(backend).not.toHaveBeenCalled();
});
it('does not let a POST call use the GET receipt handler', async () => {
  const input = new Request(`https://app.example.com${path}`, { method: 'POST', headers: { cookie } });
  expect((await GET(input)).status).toBe(405); expect(backend).not.toHaveBeenCalled();
});
