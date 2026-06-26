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

    // Verify that the table/list shows the jobs or "No active" state.
    // Wait for the "Loading..." to disappear.
    await expect(page.locator('text=Loading background tasks...')).not.toBeVisible({ timeout: 10000 });

    // It should either show 'No active or recent tasks found' or an actual job card
    const noActive = page.locator('text=No active or recent tasks found. Your agents are standing by.');
    const jobCards = page.locator('.glassmorphism.border').filter({ hasText: 'Completed at' }).or(page.locator('.glassmorphism.border').filter({ hasText: 'Started at' }));

    const hasNoActive = await noActive.isVisible();
    const hasJobs = await jobCards.count() > 0;

    expect(hasNoActive || hasJobs).toBeTruthy();
  });
});
