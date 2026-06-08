import { test, expect } from '@playwright/test';

test.describe('Viral Loyalty Program Generation', () => {
  test('should generate a loyalty program and provide share links', async ({ page }) => {
    // Navigate to loyalty dashboard
    await page.goto('/dashboard/loyalty');

    // Click generate program
    await Promise.all([
      page.waitForResponse(resp => resp.url().includes('/api/v1/growth/loyalty/generate') && resp.status() === 200),
      page.click('button:has-text("Generate Program")')
    ]);

    // Verify success and share link
    await expect(page.locator('text=Program Generated Successfully')).toBeVisible();
    await expect(page.locator('.loyalty-share-link')).toBeVisible();
  });
});
