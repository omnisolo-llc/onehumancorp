import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET } from './route';

describe('Edge Storefront Route', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn());
    vi.stubEnv('BACKEND_URL', 'http://backend:8080');
  });

  it('proxies to backend and returns valid HTML with edge caching headers', async () => {
    const mockRequest = new Request('https://ohc.app/api/v1/builder/edge/test-tenant/test-site');
    const params = Promise.resolve({ tenantId: 'test-tenant', siteId: 'test-site' });

    vi.mocked(fetch).mockResolvedValueOnce({
      ok: true,
      text: async () => '<!DOCTYPE html><html><head><title>Test</title></head><body>Test</body></html>',
      headers: new Headers({
        'Cache-Tag': 'tenant-id:test-tenant',
        'Surrogate-Key': 'tenant-id:test-tenant',
        'ETag': 'W/"test-etag"'
      })
    } as any);

    const response = await GET(mockRequest, { params });

    expect(response.status).toBe(200);
    expect(response.headers.get('Cache-Control')).toBe('public, s-maxage=60, stale-while-revalidate=86400');
    expect(response.headers.get('Content-Type')).toBe('text/html');
    expect(response.headers.get('Cache-Tag')).toBe('tenant-id:test-tenant');
    expect(response.headers.get('Surrogate-Key')).toBe('tenant-id:test-tenant');
    expect(response.headers.get('ETag')).toBe('W/"test-etag"');

    const html = await response.text();
    expect(html).toContain('<title>Test</title>');

    expect(fetch).toHaveBeenCalledWith('http://backend:8080/api/v1/builder/edge/test-tenant/test-site', expect.objectContaining({
      next: { revalidate: 60 }
    }));
  });

  it('handles backend errors gracefully', async () => {
    const mockRequest = new Request('https://ohc.app/api/v1/builder/edge/test-tenant/error-site');
    const params = Promise.resolve({ tenantId: 'test-tenant', siteId: 'error-site' });

    vi.mocked(fetch).mockResolvedValueOnce({
      ok: false,
      status: 404,
      text: async () => 'Not Found',
      headers: new Headers()
    } as any);

    const response = await GET(mockRequest, { params });

    expect(response.status).toBe(404);
    const text = await response.text();
    expect(text).toBe('Not Found');
  });

  it('handles fetch exceptions gracefully', async () => {
    const mockRequest = new Request('https://ohc.app/api/v1/builder/edge/test-tenant/crash-site');
    const params = Promise.resolve({ tenantId: 'test-tenant', siteId: 'crash-site' });

    vi.mocked(fetch).mockRejectedValueOnce(new Error('Network error'));

    const response = await GET(mockRequest, { params });

    expect(response.status).toBe(500);
    const text = await response.text();
    expect(text).toBe('Internal Server Error');
  });
});
