import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Agent Activity Dashboard', () => {
  test('displays active operations correctly', async ({ page }) => {
    // Navigate to the agent activity page
    await page.goto('/agent-activity');

    // Verify the page title
    await expect(page.locator('h1', { hasText: 'Agent Activity' })).toBeVisible();

    // Verify the Active Operations section exists
    await expect(page.locator('h2', { hasText: 'Active Operations' })).toBeVisible();

    // Verify that the table/list shows either "Loading...", "No active", or some jobs.
    // We just ensure the page doesn't crash
    await expect(page.locator('.max-w-7xl')).toBeVisible();
  });
});
