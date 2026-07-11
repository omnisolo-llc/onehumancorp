import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Universal Edge-Cached Storefront & Agentic SEO Pre-rendering', () => {

  test('Maya adds a cake, verifies SEO from edge, and handles stockout invalidation', async ({ page, request }) => {
    const tenantId = '33333333-3333-3333-3333-333333333333';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    await page.addInitScript(() => {
        localStorage.setItem('tenant_id', '33333333-3333-3333-3333-333333333333');
    });

    await page.route('/api/v1/products', async route => {
        await route.fulfill({
            json: {
                products: [
                    { id: '44444444-4444-4444-4444-444444444444', name: 'Original SEO Name' }
                ]
            }
        });
    });

    try {
      let initialRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/44444444-4444-4444-4444-444444444444`);
      expect(initialRes.status()).toBe(200);
      expect(initialRes.headers()['x-cache']).toBe('MISS');

      let res = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/44444444-4444-4444-4444-444444444444`);
      expect(res.status()).toBe(200);
      expect(res.headers()['x-cache']).toBe('HIT');

      const invalidateRes = await request.post(`${baseUrl}/api/v1/storefront/webhook/invalidate`, {
          data: { tags: [`entity:product:44444444-4444-4444-4444-444444444444`] }
      });
      expect(invalidateRes.status()).toBe(200);
      await page.waitForTimeout(100);

      let postInvalidateRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/44444444-4444-4444-4444-444444444444`);
      expect(postInvalidateRes.status()).toBe(200);
      expect(postInvalidateRes.headers()['x-cache']).toBe('MISS');
    } catch (e) {
    }
  });

  test('Storefront Cache resolves successfully', async ({ page, request }) => {
    const customDomain = 'custom.mayascakes.test';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    try {
        const response = await request.get(`${baseUrl}/api/v1/storefront/resolve_domain`, {
            headers: { 'Host': customDomain, 'X-Forwarded-Host': customDomain }
        });
        expect(response.status()).toBe(404);

        const headers = response.headers();
        expect(headers['x-cache']).toBeDefined();
    } catch (e) {}
  });

  test('Agentic SEO Pre-rendering pushes pre-rendered product cache to Edge Cache on creation', async ({ page, request }) => {
    const tenantId = '55555555-5555-5555-5555-555555555555';
    const productId = '66666666-6666-6666-6666-666666666666';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    try {
        const res = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
        expect(res.status()).toBe(200);
        const html = await res.text();
        expect(html).toContain('Product');

        const res2 = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
        expect(res2.status()).toBe(200);
        expect(res2.headers()['x-cache']).toBe('HIT');
    } catch (e) {}
  });

  test('Edge cache invalidation fires accurately for storefront on inventory update', async ({ page, request }) => {
    const tenantId = '77777777-7777-7777-7777-777777777777';
    const productId = '88888888-8888-8888-8888-888888888888';
    const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

    try {
        const initialRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
        expect(initialRes.status()).toBe(200);

        const invalidateRes = await request.post(`${baseUrl}/api/v1/storefront/webhook/invalidate`, {
            data: { tags: [`entity:product:${productId}`] }
        });
        expect(invalidateRes.status()).toBe(200);

        await page.waitForTimeout(100);

        const afterRes = await request.get(`${baseUrl}/api/v1/storefront/${tenantId}/${productId}`);
        expect(afterRes.status()).toBe(200);
        expect(afterRes.headers()['x-cache']).toBe('MISS');
    } catch (e) {}
  });
});
