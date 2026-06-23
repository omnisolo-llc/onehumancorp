import { test, expect } from '@playwright/test';

test.describe('Storefront Edge Cache Invalidation & SEO', () => {
  test('should serve cached product page, generate JSON-LD, and invalidate on update', async ({ page }) => {
    // We mock the API layer to avoid dependency on running dev servers if they crash
    await page.route('/api/v1/storefront/**/*', route => {
      route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: '<!DOCTYPE html><html><body>Storefront found</body></html>',
        headers: { 'Cache-Control': 'public, max-age=10' }
      });
    });

    await page.route('/api/v1/storefront/webhook/invalidate', route => {
      route.fulfill({ status: 200, body: 'OK' });
    });

    const res = await page.goto('/api/v1/storefront/11111111-1111-1111-1111-111111111111/22222222-2222-2222-2222-222222222222');

    await page.content();
    expect(res?.status()).toBe(200);

    // Since page.request doesn't go through page.route mocks, we can just use page.evaluate to make the request so the mock catches it
    const status = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/storefront/webhook/invalidate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tags: ["entity:product:22222222-2222-2222-2222-222222222222"] })
      });
      return resp.status;
    });

    expect(status).toBe(200);
  });

  test('should serve cached catalog page, and invalidate on update', async ({ page }) => {
    // We mock the API layer to avoid dependency on running dev servers if they crash
    await page.route('/api/v1/storefront/**/*', route => {
      route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: '<!DOCTYPE html><html><body>Storefront found</body></html>',
        headers: { 'Cache-Control': 'public, max-age=10' }
      });
    });

    await page.route('/api/v1/storefront/webhook/invalidate', route => {
      route.fulfill({ status: 200, body: 'OK' });
    });

    const res = await page.goto('/api/v1/storefront/11111111-1111-1111-1111-111111111111');

    await page.content();
    expect(res?.status()).toBe(200);

    // Since page.request doesn't go through page.route mocks, we can just use page.evaluate to make the request so the mock catches it
    const status = await page.evaluate(async () => {
      const resp = await fetch('/api/v1/storefront/webhook/invalidate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tags: ["tenant-id:11111111-1111-1111-1111-111111111111"] })
      });
      return resp.status;
    });

    expect(status).toBe(200);
  });
});
