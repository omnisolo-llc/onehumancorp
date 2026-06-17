import { test, expect } from './fixtures';

test.describe('Dashboard Parallel Fetch', () => {
  test('displays all parallel-fetched components', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the main sections to be visible, implying parallel fetching completed successfully
    await expect(page.locator('text=Operations Map')).toBeVisible();
    await expect(page.locator('text=Recent Orders')).toBeVisible();
    await expect(page.locator('text=Inbox Activity')).toBeVisible();
    await expect(page.locator('text=Growth & Virality')).toBeVisible();

    // Optionally check if we have the fallback rendering in case of no data
    const ordersContainer = page.locator('text=Recent Orders').locator('..').locator('..');
    const ordersHasTable = await ordersContainer.locator('table').count() > 0;
    const ordersHasEmpty = await ordersContainer.locator('.app-empty').count() > 0;
    expect(ordersHasTable || ordersHasEmpty).toBeTruthy();
  });
});
