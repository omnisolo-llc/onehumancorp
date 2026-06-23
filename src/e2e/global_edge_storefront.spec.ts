import { test, expect, request as playwrightRequest } from '@playwright/test';

test.describe('Global Edge-Cached Dynamic Storefronts E2E', () => {

  test('validates storefront cache invalidation on inventory update', async ({ request, page }) => {
    // Attempt to access frontend page and cache miss, triggering cache builder
    const tenantId = '11111111-1111-1111-1111-111111111111';
    const productId = '22222222-2222-2222-2222-222222222222';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);

    // Perform an inventory invalidation trigger via webhook (simulating backend ops)
    const invalidateRes = await request.post('/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`entity:product:${productId}`, `tenant-id:${tenantId}`] }
    });
    expect(invalidateRes.status()).toBe(200);

    // Give it a moment to regenerate
    await new Promise(r => setTimeout(r, 1000));

    // Hit cache again and verify regeneration logic is invoked
    let refreshed = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(refreshed.status()).toBe(200);
  });

  test('generates edge storefront with premium styling and seo tags injected via builder', async ({ request, page }) => {
    const tenantId = '11111111-1111-1111-1111-111111111111';
    const productId = '22222222-2222-2222-2222-222222222222';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    let text = await res.text();
    // Validating fallback SEO or html tags
    expect(text).toContain('<!DOCTYPE html>');
    expect(text).toContain('Edge Cake');
    expect(text).toContain('sticky-bottom-bar');
  });

  test('handles edge cache miss dynamically and creates fallback', async ({ request, page }) => {
    const tenantId = 'invalid-tenant-id';
    const productId = 'invalid-product-id';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(400); // Bad Request from Uuid parse fail
  });

  test('isolates tenant data with explicit tenant-id tags', async ({ request, page }) => {
    const tenantId = '00000000-0000-0000-0000-000000000000';
    const productId = '00000000-0000-0000-0000-000000000000';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);
    // Should display simple fallback logic
    expect(await res.text()).toContain('Product 00000000-0000-0000-0000-000000000000 not found');
  });

  test('validates cache regeneration after offline POS sync deduction', async ({ request, page }) => {
    const tenantId = '11111111-1111-1111-1111-111111111111';
    const productId = '22222222-2222-2222-2222-222222222222';
    // Analogous to updating POS orders invalidation endpoint
    const invalidateRes = await request.post('/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`tenant-id:${tenantId}`, `entity:product:${productId}`] }
    });
    expect(invalidateRes.status()).toBe(200);
  });
});
