import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Milestone Alert E2E', () => {
  // Instead of logging in with standard metrics, we will mock the dashboard metrics API
  // or test against a real DB fixture. In Playwright here, we use route mocking to simulate the 10th order milestone
  // since this is a UI layer E2E check.

  test('Milestone Alert appears when pending orders >= 10', async ({ browser }) => {
    const context = await browser.newContext();
    const page = await context.newPage();

    // The fixture typically logs in automatically, but here we can mock before navigating to dashboard
    await page.route('**/api/v1/dashboard/metrics', async route => {
      const json = {
        active_customers: 20,
        pending_orders: 12,
        total_sales: 1500,
        total_campaigns_sent: 5
      };
      await route.fulfill({ json });
    });

    await page.goto('/dashboard');

    // Wait for network idle or for our expected component to appear
    const alertLocator = page.locator('.milestone-alert');
    await expect(alertLocator).toBeVisible();

    // Verify copy
    await expect(alertLocator.locator('h3')).toHaveText('10th Order Milestone Reached!');

    await context.close();
  });

  test('Milestone Alert is hidden when pending orders < 10', async ({ browser }) => {
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.route('**/api/v1/dashboard/metrics', async route => {
      const json = {
        active_customers: 5,
        pending_orders: 2,
        total_sales: 150,
      };
      await route.fulfill({ json });
    });

    await page.goto('/dashboard');

    const alertLocator = page.locator('.milestone-alert');
    await expect(alertLocator).toBeHidden();

    await context.close();
  });
});
