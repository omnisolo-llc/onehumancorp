import { test, expect } from './fixtures';

test.describe('Viral OG Share Card Endpoint', () => {
  test('should return an image response for the new viral CTA', async ({ request }) => {
    const response = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Test Product');
    expect(response.ok()).toBeTruthy();
  });
});
Triggering a new submit to force checks
// Force push again
