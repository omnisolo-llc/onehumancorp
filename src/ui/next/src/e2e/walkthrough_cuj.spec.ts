import { test, expect } from '@playwright/test';

test.describe('Walkthrough CUJ', () => {
  test('Maya completes the interactive walkthrough on the dashboard', async ({ page }) => {
    // Mock the backend status API call to avoid hitting the database / pgvector container
    await page.route('**/api/agents/status', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          active_agents: 3,
          queued_tasks: 0,
          system_health: 'healthy'
        }),
      });
    });

    // Mock tooltips API
    await page.route('**/api/tooltips', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
           "walkthrough-btn-tooltip": "Start an interactive guide."
        }),
      });
    });

    // Navigate starting from home page with the test flag
    await page.goto('/dashboard');

    // Wait for dashboard to load
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    await page.getByRole('button', { name: 'Start Tour' }).click();

    // Check that walkthrough is active and on step 1
    await expect(page.locator('h3', { hasText: 'Track Your Revenue' })).toBeVisible();
    await expect(page.locator('text=Step 1 of 2')).toBeVisible();

    // Click next
    await page.getByRole('button', { name: 'Next' }).click();

    // Check step 2
    await expect(page.locator('h3', { hasText: 'Monitor Your Traffic' })).toBeVisible();
    await expect(page.locator('text=Step 2 of 2')).toBeVisible();

    // Click finish
    await page.getByRole('button', { name: 'Finish' }).click();

    // Ensure walkthrough disappears
    await expect(page.locator('h3', { hasText: 'Monitor Your Traffic' })).toBeHidden();
  });
});
