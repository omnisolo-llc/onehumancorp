import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { GET, HEAD } from './route';
import { PUBLIC_SITE_SECURITY_HEADERS } from '@/lib/auth/publicSiteRequest';
const path = '/api/v1/public/sites/30000000-0000-4000-8000-000000000003';
beforeEach(() => {
  vi.stubEnv('OMNISOLO_WEB_CANONICAL_ORIGIN', 'https://app.example.test');
  vi.stubEnv('BACKEND_URL', 'https://backend.example.test');
  vi.stubGlobal('fetch', vi.fn(async () => new Response('<h1>Reviewed public version</h1>', { headers: PUBLIC_SITE_SECURITY_HEADERS })));
});
afterEach(() => { vi.unstubAllGlobals(); vi.unstubAllEnvs(); });
it('mounts the anonymous root handler without needing session keys, cookies or identity', async () => {
  const response = await GET(new Request('https://app.example.test' + path));
  expect(response.status).toBe(200); expect(await response.text()).toBe('<h1>Reviewed public version</h1>');
  expect(String(vi.mocked(fetch).mock.calls[0][0])).toBe('https://backend.example.test' + path);
});
it.each(['/pages/about', '/pages/Caf%C3%A9', '/products/40000000-0000-4000-8000-000000000004'])('mounts the reviewed subresource %s', async suffix => {
  expect((await GET(new Request('https://app.example.test' + path + suffix))).status).toBe(200);
  expect(String(vi.mocked(fetch).mock.calls[0][0])).toBe('https://backend.example.test' + path + suffix);
});
it('does not allow the framework to infer an unaudited HEAD response', async () => {
  const response = HEAD(); expect(response.status).toBe(405); expect(fetch).not.toHaveBeenCalled(); expect(response.headers.get('cache-control')).toBe('no-store');
});
