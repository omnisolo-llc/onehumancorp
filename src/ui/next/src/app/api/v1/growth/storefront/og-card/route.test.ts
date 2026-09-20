import { afterEach, describe, expect, it, vi } from 'vitest';
import { GET, runtime } from './route';

afterEach(() => vi.restoreAllMocks());

describe('native Node storefront share-card route', () => {
  it('retains the SVG response and explicit public cache policy', async () => {
    expect(runtime).toBe('nodejs');
    const response = await GET(new Request('https://app.example.test/api/v1/growth/storefront/og-card'));
    expect(response.status).toBe(200);
    expect(response.headers.get('content-type')).toBe('image/svg+xml');
    expect(response.headers.get('cache-control')).toBe('public, max-age=60, s-maxage=60, stale-while-revalidate=300');
    const svg = await response.text();
    expect(svg).toContain('<svg');
    expect(svg).toContain('Product');
    expect(svg).toContain('my-store');
  });

  it('escapes user-supplied text instead of creating active SVG elements', async () => {
    const url = new URL('https://app.example.test/api/v1/growth/storefront/og-card');
    url.searchParams.set('product_name', '<script>alert("unsafe")</script>');
    url.searchParams.set('tenant', 'Owner & Partners\' Store');
    const response = await GET(new Request(url));
    const svg = await response.text();
    expect(response.status).toBe(200);
    expect(svg).not.toContain('<script>');
    expect(svg).toContain('&lt;script&gt;alert(&quot;unsafe&quot;)&lt;/script&gt;');
    expect(svg).toContain('Owner &amp; Partners&#039; Store');
  });

  it('reports failure without echoing malformed input to the response or log', async () => {
    const log = vi.spyOn(console, 'error').mockImplementation(() => {});
    const request = new Request('https://app.example.test');
    Object.defineProperty(request, 'url', { value: 'not-a-url-with-private-input' });
    const response = await GET(request);
    expect(response.status).toBe(500);
    expect(await response.text()).toBe('Failed to generate the image');
    expect(log).toHaveBeenCalledExactlyOnceWith('storefront.og_card.generation_failed');
  });
});
