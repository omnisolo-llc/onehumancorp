import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Performance & SEO Card', () => {
  test('displays edge cache and SEO status', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard');

    // Check if the card is visible
    const performanceCard = page.locator('.performance-card');
    await expect(performanceCard).toBeVisible();

    // Check content
    await expect(performanceCard).toContainText('Performance & SEO');
    await expect(performanceCard).toContainText('Edge Cache Active');
    await expect(performanceCard).toContainText('SEO Status: Excellent');
  });
});
