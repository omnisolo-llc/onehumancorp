import { beforeEach, expect, it, vi } from 'vitest';
import { classifyRequest } from './publicRoutes';
import * as implementation from './publicSiteRequest';
const SITE = '30000000-0000-4000-8000-000000000003';
const PATH = '/api/v1/public/sites/' + SITE;
const CSP = "default-src 'none'; script-src 'none'; style-src 'unsafe-inline'; img-src https: http:; base-uri 'none'; form-action 'none'; object-src 'none'; frame-src 'none'; connect-src 'none'";
const safe = { 'content-type': 'text/html; charset=utf-8', 'cache-control': 'no-store', 'x-content-type-options': 'nosniff', 'referrer-policy': 'no-referrer', 'content-security-policy': CSP };
const backend = vi.fn<typeof fetch>();
beforeEach(() => backend.mockReset().mockResolvedValue(new Response('<h1>Actual reviewed HTML</h1>', { headers: safe })));
const proxy = (request = new Request('https://app.example.test' + PATH), timeoutMs = 1000) => implementation.proxyPublicSiteRequest(request, { backendOrigin: 'https://backend.example.test', fetchImpl: backend, timeoutMs });
it('grants only canonical anonymous document GETs, leaving private previews and mutation routes protected', () => {
  for (const pathname of [PATH, PATH + '/pages/about', PATH + '/pages/Caf%C3%A9']) expect(classifyRequest({ method: 'GET', pathname, invocation: 'route-handler' }).access).toBe('public');
  for (const pathname of ['/bio/tenant-a', '/api/v1/builder/publications', PATH + '/', PATH + '/products/item', PATH + '/pages/', PATH + '/pages/a//b']) expect(classifyRequest({ method: 'GET', pathname, invocation: 'route-handler' }).access).not.toBe('public');
  for (const method of ['POST', 'DELETE', 'HEAD']) expect(classifyRequest({ method, pathname: PATH, invocation: 'route-handler' }).access).not.toBe('public');
});
it('forwards no browser identity or cache validator and returns only bounded verified HTML with exact security headers', async () => {
  const response = await proxy(new Request('https://app.example.test' + PATH, { headers: { cookie: 'private', authorization: 'Bearer private', 'x-user-id': 'forged', 'x-tenant-id': 'forged', 'if-none-match': 'old' } }));
  expect(response.status).toBe(200); expect(await response.text()).toBe('<h1>Actual reviewed HTML</h1>');
  expect(String(backend.mock.calls[0][0])).toBe('https://backend.example.test' + PATH);
  expect(backend.mock.calls[0][1]).toMatchObject({ method: 'GET', redirect: 'manual', cache: 'no-store', credentials: 'omit' });
  const headers = new Headers(backend.mock.calls[0][1]?.headers); expect([...headers.entries()]).toEqual([['accept', 'text/html']]);
  for (const [key, value] of Object.entries(safe)) expect(response.headers.get(key)).toBe(value);
});
it.each(['?tenant_id=forged', '/pages/%2fprivate', '/pages/%252e', '/pages/about/', '/pages/a//b', '/products/item'])('rejects unreviewed request suffix %s before any backend call', async suffix => {
  expect((await proxy(new Request('https://app.example.test' + PATH + suffix))).status).toBe(400); expect(backend).not.toHaveBeenCalled();
});
it.each([301, 302, 304, 401, 403, 500])('never relays an upstream %s or its private bytes', async status => {
  backend.mockResolvedValue(new Response(status === 304 ? null : 'private backend details', { status, headers: { ...safe, location: 'https://outside.test', 'set-cookie': 'secret', 'www-authenticate': 'internal' } }));
  const response = await proxy(); expect(response.status).toBe(502); expect(await response.text()).not.toContain('private backend details'); expect(response.headers.has('location')).toBe(false); expect(response.headers.has('set-cookie')).toBe(false);
});
it('returns an opaque uncached404 when the backend withdraws publication eligibility', async () => {
  backend.mockResolvedValue(new Response('private failure context', { status: 404 }));
  const response = await proxy(); expect(response.status).toBe(404); expect(await response.text()).not.toContain('private failure'); expect(response.headers.get('cache-control')).toBe('no-store');
});
it.each(Object.keys(safe))('rejects a successful response without its required %s header', async missing => {
  const headers = { ...safe }; delete headers[missing as keyof typeof headers];
  backend.mockResolvedValue(new Response('<h1>Unchecked</h1>', { headers }));
  expect((await proxy()).status).toBe(502);
});
it('rejects invalid UTF8 and an oversized declared document before exposing bytes', async () => {
  backend.mockResolvedValueOnce(new Response(new Uint8Array([255]), { headers: safe })); expect((await proxy()).status).toBe(502);
  backend.mockResolvedValueOnce(new Response('small', { headers: { ...safe, 'content-length': String(8 * 1024 * 1024 + 1) } })); expect((await proxy()).status).toBe(502);
});
it('cancels an oversized streamed document and does not buffer unbounded bytes', async () => {
  let count = 0; let cancelled = false;
  backend.mockResolvedValue(new Response(new ReadableStream({ pull(controller) { count++; controller.enqueue(new Uint8Array(1024 * 1024)); }, cancel() { cancelled = true; } }), { headers: safe }));
  expect((await proxy()).status).toBe(502); expect(cancelled).toBe(true); expect(count).toBeLessThanOrEqual(10);
});
it('aborts an incomplete document at the deadline instead of hanging or returning a partial success', async () => {
  let cancelled = false;
  backend.mockResolvedValue(new Response(new ReadableStream({ start(controller) { controller.enqueue(new TextEncoder().encode('<h1>partial')); }, cancel() { cancelled = true; } }), { headers: safe }));
  expect((await proxy(undefined, 20)).status).toBe(503); expect(cancelled).toBe(true);
});
