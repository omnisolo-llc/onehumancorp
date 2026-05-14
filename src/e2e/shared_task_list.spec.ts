import { test, expect } from '@playwright/test';

test.describe('Dashboard Canvas UI', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate home
    await page.goto('/');

    const dashboardLink = page.locator('text=Dashboard');
    if (await dashboardLink.isVisible()) {
      await dashboardLink.first().click();
    } else {
      const loginBtn = page.locator('text=Login');
      if (await loginBtn.isVisible()) {
        await loginBtn.click();
      } else {
        await page.goto('/dashboard');
      }
    }
    await page.waitForURL('**/dashboard**');
  });

  test('Zero to live - Should complete full CUJ on Dashboard', async ({ page }) => {
    // 1. Verify basic elements are loaded
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Business Summary' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Swarm Observability Panel' })).toBeVisible();

    // 2. The UI polls every 2 seconds. The mock API returns task-1 and task-2.
    // Wait for the mock tasks to appear.
    const task1 = page.locator('.task-id', { hasText: 'task-1' });
    await expect(task1).toBeVisible({ timeout: 10000 });

    const status1 = page.locator('.task-status').nth(0);
    await expect(status1).toHaveText(/RUNNING|COMPLETED|PENDING/);

    // 3. Test interactive elements
    await expect(page.locator('h2', { hasText: 'Website Preview' })).toBeVisible();
    const editWebsiteBtn = page.locator('button', { hasText: 'Edit Website' });
    await expect(editWebsiteBtn).toBeVisible();

    // Simulate user action (complete CUJ)
    await editWebsiteBtn.click();
  });
});
