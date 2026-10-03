import { beforeEach, expect, it, vi } from 'vitest';
import { NextRequest } from 'next/server';
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
const descriptor = { id: 'registry-agent', name: 'Reviewed agent', description: 'Reviewed description', author: 'Owner', version: '1.0.0', endpoint: 'https://registry.example.test/definition' };
const NOW = 1_800_000_000;
const backendFetch = vi.fn<typeof fetch>();
beforeEach(async () => {
  backendFetch.mockReset().mockImplementation(async (_target, options) => {
    const rpc = JSON.parse(new TextDecoder().decode(options?.body as Uint8Array));
    return Response.json({ jsonrpc: '2.0', id: rpc.id, result: descriptor });
  });
  dependencies = {
    config: parseAuthRuntimeConfig({ OMNISOLO_WEB_CANONICAL_ORIGIN: 'https://app.example.test', BACKEND_URL: 'https://backend.example.test' }),
    ring: await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'test', OMNISOLO_WEB_SESSION_SECRET: 'Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I' }),
    now: () => NOW, fetchImpl: backendFetch, timeoutMs: 1000, requestLimitBytes: 8192, responseLimitBytes: 8192,
  };
});
async function request(tenant: string, value: unknown = descriptor, method = 'POST', query = '') {
  const session = { version: 1 as const, iat: NOW, exp: NOW + 3600, accessToken: 'test-bearer-' + tenant, user: { id: 'user-' + tenant, username: tenant, roles: ['ADMIN'], organizationId: tenant } };
  const sealed = await sealSession(session, dependencies.ring, sessionCodecContext(dependencies.config), { now: NOW, backendExpiresAt: session.exp });
  const cookie = serializeSessionCookie(cookieForSession(dependencies.config, sealed, session.iat, session.exp)).split(';', 1)[0];
  return new NextRequest('https://app.example.test/api/v1/agents/marketplace' + query, { method, headers: { cookie, 'x-tenant-id': 'forged', 'x-user-id': 'forged', 'content-type': 'application/json', origin: 'https://app.example.test' }, ...(method === 'POST' ? { body: JSON.stringify(value) } : {}) });
}
it.each([
  { ...descriptor, role: 'Writer' },
  { ...descriptor, system_prompt: 'Private instructions' },
  { ...descriptor, name: '' },
  { ...descriptor, endpoint: 'javascript:alert(1)' },
  { name: 'Incomplete agent' },
])('rejects unsupported or incomplete publication before authenticated backend effects %#', async value => {
  const { POST } = await import('./route');
  const response = await POST(await request('tenant-A', value));
  expect(response.status).toBe(400);
  expect(backendFetch).not.toHaveBeenCalled();
});
it.each(['name', 'description', 'author', 'version', 'endpoint', 'id'])('does not accept a receipt that changes the reviewed %s', async field => {
  backendFetch.mockImplementation(async (_target, options) => {
    const rpc = JSON.parse(new TextDecoder().decode(options?.body as Uint8Array));
    return Response.json({ jsonrpc: '2.0', id: rpc.id, result: { ...descriptor, [field]: field === 'endpoint' ? 'https://other.example.test/definition' : 'changed' } });
  });
  const { POST } = await import('./route');
  expect((await POST(await request('tenant-A'))).status).toBe(502);
});
it('uses each sealed session for backend authority and never returns a prior tenant publication after denial', async () => {
  backendFetch.mockImplementation(async (_target, options) => {
    const headers = new Headers(options?.headers);
    const tenant = headers.get('x-tenant-id');
    expect(headers.get('x-user-id')).toBe('user-' + tenant);
    expect(headers.get('authorization')).toBe('Bearer test-bearer-' + tenant);
    if (tenant === 'tenant-B') return Response.json({ error: 'tenant-B denied' }, { status: 403 });
    const rpc = JSON.parse(new TextDecoder().decode(options?.body as Uint8Array));
    return Response.json({ jsonrpc: '2.0', id: rpc.id, result: descriptor });
  });
  const { POST, GET } = await import('./route');
  expect((await POST(await request('tenant-A'))).status).toBe(200);
  const denied = await GET(await request('tenant-B', undefined, 'GET'));
  expect(denied.status).toBe(403);
  expect(await denied.text()).not.toContain(descriptor.name);
  expect(backendFetch).toHaveBeenCalledTimes(2);
});
it('rejects an unsealed request before provider effects', async () => {
  const { POST } = await import('./route');
  const response = await POST(new Request('https://app.example.test/api/v1/agents/marketplace', { method: 'POST', body: JSON.stringify(descriptor) }));
  expect(response.status).toBe(401);
  expect(backendFetch).not.toHaveBeenCalled();
});

it.each([null, '', '   '])('rejects missing or blank fetch identity before backend effects: %j', async id => {
  const query = new URLSearchParams({ method: 'fetch' });
  if (id !== null) query.set('agent_id', id);
  const { GET } = await import('./route');
  expect((await GET(await request('tenant-A', undefined, 'GET', '?' + query))).status).toBe(400);
  expect(backendFetch).not.toHaveBeenCalled();
});
it('rejects a fetched receipt for a different agent despite matching RPC correlation', async () => {
  const { GET } = await import('./route');
  expect((await GET(await request('tenant-A', undefined, 'GET', '?method=fetch&agent_id=another-agent'))).status).toBe(502);
});
it('returns the complete descriptor only for the requested fetched agent', async () => {
  const { GET } = await import('./route');
  const response = await GET(await request('tenant-A', undefined, 'GET', '?method=fetch&agent_id=' + descriptor.id));
  expect(response.status).toBe(200);
  expect(await response.json()).toEqual(descriptor);
});
