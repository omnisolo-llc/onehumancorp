import { test, expect } from '@playwright/test';

test.describe('The Promoter Agent CUJ', () => {
  test('generates social post and SEO tags for a new product', async ({ page }) => {

    // We start at the homepage/triage feed
    await page.goto('/dashboard.html');

    // Verify the Promoter card is visible and click to go to promoter agent
    await page.click('#promoter-btn');

    // Assert the presence of the promoter agent page
    await expect(page.locator('h1', { hasText: 'The Promoter' })).toBeVisible();

    // Fill in the form
    await page.fill('#product-name', 'Vegan Chocolate Cake');
    await page.fill('#product-desc', 'A delicious vegan chocolate cake');

    // Click "Generate Posts"
    await page.click('#generate-btn');

    // Wait for the variants to appear
    await expect(page.locator('.variant-card').first()).toBeVisible({ timeout: 60000 });
  });
});
