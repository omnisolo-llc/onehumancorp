import { test, expect } from '@playwright/test';

test.describe('Analytics Charts E2E tests', () => {

  test.beforeEach(async ({ page }) => {
    // Start from the home page.
    await page.goto('http://localhost:3000');
    // Ensure dashboard is loaded
    await expect(page.locator('text=/OHC Dashboard/i').first()).toBeVisible();
  });

  test('should display stats button and open analytics charts', async ({ page }) => {
    const statsBtn = page.locator('button:has-text("Stats")').first();
    await expect(statsBtn).toBeVisible();

    await statsBtn.click();

    // Verify Business Analytics panel is opened
    const title = page.locator('text=/Business Analytics/i').first();
    await expect(title).toBeVisible();
  });

  test('should load team overview chart from real backend metrics', async ({ page }) => {
    const statsBtn = page.locator('button:has-text("Stats")').first();
    await statsBtn.click();

    // The backend `get_analytics` implementation returns a "Team Overview" chart
    // with points for "AI Agents" and "Humans". We check if this mock data has been removed
    // and replaced by the backend data structure mapping in the rust UI.
    const teamOverviewChartTitle = page.locator('text=/Team Overview/i').first();
    await expect(teamOverviewChartTitle).toBeVisible();

    const aiAgentsLabel = page.locator('text=/AI Agents/i').first();
    await expect(aiAgentsLabel).toBeVisible();

    const humansLabel = page.locator('text=/Humans/i').first();
    await expect(humansLabel).toBeVisible();
  });

  test('should load operations chart from real backend metrics', async ({ page }) => {
    const statsBtn = page.locator('button:has-text("Stats")').first();
    await statsBtn.click();

    // "Operations" is the second chart mapped in main.rs from get_analytics
    const operationsChartTitle = page.locator('text=/Operations/i').first();
    await expect(operationsChartTitle).toBeVisible();

    const pendingApprovalsLabel = page.locator('text=/Pending Approvals/i').first();
    await expect(pendingApprovalsLabel).toBeVisible();

    const activeHandoffsLabel = page.locator('text=/Active Handoffs/i').first();
    await expect(activeHandoffsLabel).toBeVisible();
  });

  test('should not contain hardcoded Revenue Over Time mock data', async ({ page }) => {
    const statsBtn = page.locator('button:has-text("Stats")').first();
    await statsBtn.click();

    // Assert that the old mock data is completely gone.
    const oldMockTitle = page.locator('text=/Revenue Over Time/i').first();
    await expect(oldMockTitle).not.toBeVisible();
  });

  test('should be able to close the analytics dashboard', async ({ page }) => {
    const statsBtn = page.locator('button:has-text("Stats")').first();
    await statsBtn.click();

    const title = page.locator('text=/Business Analytics/i').first();
    await expect(title).toBeVisible();

    const closeBtn = page.locator('button:has-text("Close")').first();
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();

    await expect(title).not.toBeVisible();
  });

});
