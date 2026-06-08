import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed on Mobile', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('should display database-backed operations console and verify no horizontal scroll on mobile', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();

    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);

    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);
  });
});
