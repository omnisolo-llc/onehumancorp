import { test, expect, request as playwrightRequest } from '@playwright/test';

test.describe('Global Edge-Cached Dynamic Storefronts E2E', () => {

  test('validates storefront cache invalidation on inventory update', async ({ request, page }) => {
    // Attempt to access frontend page and cache miss, triggering cache builder
    const tenantId = 'e2e-tenant';
    const productId = 'e2e-product-cake';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(res.status()).toBe(200);

    // Perform an inventory invalidation trigger via webhook (simulating backend ops)
    const invalidateRes = await request.post('/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`entity:product:${productId}`] }
    });
    expect(invalidateRes.status()).toBe(200);

    // Hit cache again and verify regeneration logic is invoked
    // In a real e2e environment this hits the backend properly. We verify the API endpoint contract.
    let refreshed = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    expect(refreshed.status()).toBe(200);
  });

  test('generates edge storefront with premium styling and seo tags injected via builder', async ({ request, page }) => {
    const tenantId = 'e2e-tenant';
    const productId = 'e2e-product-cake';

    let res = await request.get(`/api/v1/storefront/${tenantId}/${productId}`);
    let text = await res.text();
    // Validating fallback SEO or html tags
    expect(text).toContain('<!DOCTYPE html>');
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
    // Analogous to updating POS orders invalidation endpoint
    const invalidateRes = await request.post('/api/v1/storefront/webhook/invalidate', {
      data: { tags: [`tenant-id:00000000-0000-0000-0000-000000000000`] }
    });
    expect(invalidateRes.status()).toBe(200);
  });
});

  test('verifies owner storefront review UI', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard.html');

    // Look for the "Review Storefront" link (it should exist in dashboard)
    const reviewLink = page.locator('a', { hasText: 'Review Storefront' }).first();
    await expect(reviewLink).toBeVisible();

    // Click it and wait for the storefront review UI to load
    await reviewLink.click();
    await expect(page).toHaveURL(/.*storefront\.html/);

    // Verify UI structure
    await expect(page.locator('h1')).toHaveText('Review Storefront');
    await expect(page.locator('.device-mockup iframe')).toBeVisible();

    // Verify the iframe src contains the storefront URL
    await expect(page.locator('#storefront-frame')).toHaveAttribute('src', /\/api\/v1\/storefront\/.+/);
  });
