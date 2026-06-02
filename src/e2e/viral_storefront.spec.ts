import { test, expect } from './fixtures';

test.describe('Viral Storefront embed logic', () => {
  test('exposes og-card for storefront widget', async ({ page, request }) => {
    // Navigate to referals and check embed
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Embed on Your Website' })).toBeVisible();

    // Verify OG image generation endpoint
    const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
    expect(ogCard.headers()['content-type']).toContain('image/png');
  });
});
