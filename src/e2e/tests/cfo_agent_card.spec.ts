import { test, expect } from '@playwright/test';

test.describe('CFOAgentCard Integration', () => {
  test('should load safe-to-spend data correctly on the dashboard', async ({ page }) => {
    // Navigate to the dashboard. The stack (frontend and backend) should be running,
    // so we should not mock anything here and rely on the actual response.
    // Explicitly add baseURL from environment or hardcode to 8080 if not set,
    // as it seems Playwright baseURL is somehow not picked up in this standalone test.
    const baseUrl = process.env.BASE_URL || 'http://127.0.0.1:8080';
    await page.goto(`${baseUrl}/dashboard`);

    // Wait for the page to load the component title
    await page.waitForSelector('text=Profit & Tax Card', { timeout: 10000 });

    // Verify the structure of the CFOAgentCard
    await expect(page.locator('text=Money In')).toBeVisible();
    await expect(page.locator('text=Money Out')).toBeVisible();
    await expect(page.locator('text=Estimated Tax Safe')).toBeVisible();

    // Verify there are $ figures present
    const moneyMatch = page.locator('text=/\\$\\d+\\.\\d{2}/');
    await expect(moneyMatch.first()).toBeVisible();
  });
});
