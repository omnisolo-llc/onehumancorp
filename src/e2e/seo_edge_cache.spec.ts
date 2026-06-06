import { test, expect } from '@playwright/test';

test.describe('Universal Edge-Cached Dynamic Storefront & Agentic SEO', () => {
  test('Agent correctly renders SEO meta tags when navigating to the marketing department inbox', async ({ page }) => {
    // Navigate to the Team inbox and open the Marketing department
    await page.goto('/team');

    // Ensure the Team view loaded
    await expect(page.locator('h1', { hasText: 'Your Team' })).toBeVisible();

    // Click on The Promoter (Marketing) department card
    await page.locator('text=The Promoter').click();

    // Verify the newly implemented Storefront SEO & Speed card is visible
    await expect(page.locator('text=Storefront SEO & Speed')).toBeVisible();
    await expect(page.locator('text=0.8s')).toBeVisible();
    await expect(page.locator('text=Promoter Agent updated meta tags for 3 new custom cakes.')).toBeVisible();
  });
});
