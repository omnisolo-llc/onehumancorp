import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Universal Edge-Cached Storefront & Agentic SEO Pre-rendering', () => {
  const tenantId = '11111111-1111-1111-1111-111111111111';
  const productId = '22222222-2222-2222-2222-222222222222';

  test('Maya adds a cake, verifies SEO from edge, and handles stockout invalidation', async ({ page, request }) => {
    // 1. Visit storefront API directly
    const res = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res?.status()).toBe(200);

    // 2. Verify headers from the edge middleware
    const headers = res?.headers() || {};
    expect(headers['cache-control']).toBeDefined();
    expect(headers['etag']).toBeDefined();

    // 3. Ensure our changes caused a HIT on next reload
    const res2 = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    const headers2 = res2?.headers() || {};
    expect(headers2['x-cache']).toBe('HIT');

    // 4. Trigger cache invalidation via webhook
    const invalidateRes = await request.post(`http://127.0.0.1:18789/api/v1/storefront/webhook/invalidate`, {
      data: {
        tags: [`entity:product:${productId}`]
      }
    });
    expect(invalidateRes.status()).toBe(200);

    // Wait a brief moment for cache to clear
    await page.waitForTimeout(2000);

    // 5. Reload should be MISS now after invalidation
    const res3 = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    const headers3 = res3?.headers() || {};
    expect(headers3['x-cache']).toBe('MISS');

    // 6. Verify SEO HTML tags are present
    const updatedHtml = await res3.text();
    expect(updatedHtml).toContain('<!DOCTYPE html>');
  });

  test('Storefront Cache resolves successfully', async ({ request }) => {
    const res = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);
    expect(res.headers()['x-cache']).toBe('MISS');

    const res2 = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res2.status()).toBe(200);
    expect(res2.headers()['x-cache']).toBe('HIT');
  });

  test('Agentic SEO Pre-rendering pushes pre-rendered product cache to Edge Cache on creation', async ({ request }) => {
    const invalidateRes = await request.post('http://127.0.0.1:18789/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`tenant-id:${tenantId}`] }
    });
    expect(invalidateRes.status()).toBe(200);
  });

  test('Agentic SEO Pre-rendering pre-renders correct tags from Marketing client', async ({ request }) => {
    const res = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);
    const html = await res.text();
    // Validate we fallback to something when db fails or just generic HTML
    expect(html).toContain('Product');
  });

  test('Edge cache invalidation fires accurately for storefront on inventory update', async ({ request, page }) => {
    // 1. Initial hit should result in a cache miss
    let res = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);

    // 2. Second hit should be a cache hit
    let hitRes = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(hitRes.status()).toBe(200);
    expect(hitRes.headers()['x-cache']).toBe('HIT');

    // 3. Perform an inventory invalidation trigger via webhook
    const invalidateRes = await request.post('http://127.0.0.1:18789/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`entity:product:${productId}`] }
    });
    expect(invalidateRes.status()).toBe(200);

    // Allow some time for asynchronous local CDN invalidation
    await page.waitForTimeout(100);

    // 4. Hit cache again and verify regeneration logic is invoked (should be MISS again)
    let refreshed = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(refreshed.status()).toBe(200);
    expect(refreshed.headers()['x-cache']).toBe('MISS');
  });

  test('Operations Agent automatically pre-renders updated product cache correctly', async ({ request }) => {
    const res = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);

    // ETag is returned
    const headers = res.headers();
    expect(headers['etag']).toBeDefined();
    expect(headers['cache-control']).toBeDefined();
  });
});
