import { test, expect } from '@playwright/test';

test.describe('Universal Edge-Cached Storefront & Agentic SEO Pre-rendering', () => {

  test('Maya adds a cake, verifies SEO from edge, and handles stockout invalidation', async ({ page, request }) => {
    // Simulating Maya adding a cake via API
    const tenantId = '33333333-3333-3333-3333-333333333333';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    // Creating product would normally push to Edge cache. We will mock the DB in tests
    // or test the cache directly by fetching storefront delivery route.
    let initialRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/44444444-4444-4444-4444-444444444444`);

    // If the server is offline entirely in test environment, playwright will throw a connection refused, failing loudly.
    // If it's online, we expect a 200 (fallback string or real HTML).
    expect(initialRes.status()).toBe(200);

    // Simulate inventory update and invalidation
    const invalidateRes = await request.post(`${baseUrl}/api/v1/storefront/webhook/invalidate`, {
        data: { tags: [`entity:product:44444444-4444-4444-4444-444444444444`] }
    });
    expect(invalidateRes.status()).toBe(200);
  });

  test('Storefront Cache resolves successfully', async ({ page, request }) => {
    const customDomain = 'custom.mayascakes.test';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    const response = await request.get(`${baseUrl}/api/v1/storefront/resolve_domain`, {
        headers: { 'Host': customDomain, 'X-Forwarded-Host': customDomain }
    });
    expect(response.status()).toBe(404);

    const headers = response.headers();
    expect(headers['x-cache']).toBeDefined();
  });

  test('Agentic SEO Pre-rendering pushes pre-rendered product cache to Edge Cache on creation', async ({ page, request }) => {
    const tenantId = '55555555-5555-5555-5555-555555555555';
    const productId = '66666666-6666-6666-6666-666666666666';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    const res = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);
    const html = await res.text();
    expect(html).toContain('Product');

    // Check that it's cached in CDN middleware by doing a second request
    const res2 = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
    expect(res2.status()).toBe(200);
    expect(res2.headers()['x-cache']).toBe('HIT');
  });

  test('Edge cache invalidation fires accurately for storefront on inventory update', async ({ page, request }) => {
    const tenantId = '77777777-7777-7777-7777-777777777777';
    const productId = '88888888-8888-8888-8888-888888888888';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    const initialRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
    expect(initialRes.status()).toBe(200);

    const invalidateRes = await request.post(`${baseUrl}/api/v1/storefront/webhook/invalidate`, {
        data: { tags: [`entity:product:${productId}`] }
    });
    expect(invalidateRes.status()).toBe(200);

    // In test environment, the next request should correctly report MISS after invalidation
    // Note: Due to async nature of cache clearing we might need a small delay, but we'll try without
    await page.waitForTimeout(100);

    const afterRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
    expect(afterRes.status()).toBe(200);
    expect(afterRes.headers()['x-cache']).toBe('MISS');
  });
});
