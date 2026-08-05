import { test, expect } from '../../../../e2e/fixtures';







test.describe('Member Analytics', () => {
  test('Owner can navigate to Settings -> Member Analytics and see the table', async ({ page }) => {
    // 1. Navigate to Settings
    await page.goto('/settings');

    // Wait for the page to load
    await page.waitForSelector('text="Member Analytics"');

    // Check if the member analytics section is visible
    const memberAnalyticsTitle = page.locator('text="Member Analytics"');
    await expect(memberAnalyticsTitle).toBeVisible();

    // Check if the table headers are visible
    const usernameHeader = page.locator('th:has-text("Username")');
    await expect(usernameHeader).toBeVisible();

    const featureHeader = page.locator('th:has-text("Feature")');
    await expect(featureHeader).toBeVisible();

    const tokensUsedHeader = page.locator('th:has-text("Tokens Used")');
    await expect(tokensUsedHeader).toBeVisible();

    const computedCostHeader = page.locator('th:has-text("Computed Cost")');
    await expect(computedCostHeader).toBeVisible();
  });
});
