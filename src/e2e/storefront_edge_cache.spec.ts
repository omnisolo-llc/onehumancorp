import { test as baseTest, expect } from '@playwright/test';

// Use baseTest to bypass fixture login, as the storefront endpoints are public API.
const test = baseTest;

test.describe('Storefront Edge Cache Invalidation & SEO', () => {
  test('should serve cached product page, generate JSON-LD, and invalidate on update', async ({ page, request }) => {
    test.setTimeout(120000);

    const tenantId = '11111111-1111-1111-1111-111111111111';
    const productId = '22222222-2222-2222-2222-222222222222';

    // 1. Visit storefront API directly - hits real backend route using playwright's baseURL via relative path
    const res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(res?.status()).toBe(200);

    // 2. Verify headers from the edge middleware
    const headers = res?.headers() || {};
    expect(headers['cache-control']).toBeDefined();
    expect(headers['etag']).toBeDefined();

    // 3. Ensure our changes caused a HIT on next reload
    const res2 = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    const headers2 = res2?.headers() || {};
    expect(headers2['x-cache']).toBe('HIT');

    // 4. Trigger cache invalidation via webhook
    const invalidateRes = await request.post(`/api/v1/storefront/webhook/invalidate`, {
      data: {
        tags: [`entity:product:${productId}`]
      }
    });
    expect(invalidateRes.status()).toBe(200);

    // Wait a brief moment for cache to clear
    await page.waitForTimeout(2000);

    // 5. Reload should be MISS now after invalidation
    const res3 = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    const headers3 = res3?.headers() || {};
    expect(headers3['x-cache']).toBe('MISS');

    // 6. Verify SEO JSON-LD schema is present
    const updatedHtml = await res3.text();
    expect(updatedHtml).toContain('application/ld+json');
    expect(updatedHtml).toContain('schema.org');
  });


  test('should cache the catalog endpoint and invalidate on update', async ({ page, request }) => {
    test.setTimeout(120000);
    const tenantId = '11111111-1111-1111-1111-111111111111';

    // 1. Visit storefront catalog API directly
    const res = await request.get(`/api/v1/storefront/${tenantId}/catalog`);
    expect(res.status()).toBe(200);

    // 2. Verify headers
    const headers = res.headers();
    expect(headers['cache-control']).toBeDefined();
    expect(headers['etag']).toBeDefined();

    // 3. Next reload should HIT
    const res2 = await request.get(`/api/v1/storefront/${tenantId}/catalog`);
    const headers2 = res2.headers();
    expect(headers2['x-cache']).toBe('HIT');

    // 4. Invalidate via webhook
    const invalidateRes = await request.post(`/api/v1/storefront/webhook/invalidate`, {
      data: {
        tags: [`tenant-id:${tenantId}`]
      }
    });
    expect(invalidateRes.status()).toBe(200);

    await page.waitForTimeout(2000);

    // 5. Next reload should MISS
    const res3 = await request.get(`/api/v1/storefront/${tenantId}/catalog`);
    const headers3 = res3.headers();
    expect(headers3['x-cache']).toBe('MISS');
  });
});
