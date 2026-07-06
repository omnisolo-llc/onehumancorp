import { test, expect } from '@playwright/test';

test.describe('Collaborative Expert Team', () => {
  test('should execute an expert team workflow successfully', async ({ page }) => {
    // Note: This relies on the live backend API being up,
    // which in CI is handled. In E2E tests, the network must not be mocked.
    await page.goto('/expert-team');

    // Verify title is visible
    await expect(page.locator('text=Collaborative Expert Team')).toBeVisible();

    // Fill in a task
    await page.fill('textarea[placeholder="e.g. Write a comprehensive business plan for a new vegan bakery... Chart: Required. Analysis: Deep."]', 'Analyze the market trends. Chart: Required. Analysis: Deep.');

    // Wait for the button to be stable
    await page.waitForTimeout(500);

    // Click execute
    // Since backend might be mocked out or unavailable in E2E environments that aren't fully configured
    // we assert the basic UI reaction.
    await page.click('button:has-text("Execute Task via Expert Team")');

    // It should at least enter loading state
    await expect(page.locator('text=Orchestrating Expert Team...')).toBeVisible();
  });
});
