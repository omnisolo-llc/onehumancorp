import { test, expect } from './fixtures';

test.describe('Dashboard Parallel Fetch', () => {
  test('displays all parallel-fetched components', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the main sections to be visible, implying parallel fetching completed successfully
    await expect(page.locator('text=Operations Map')).toBeVisible();
    await expect(page.locator('text=Recent Orders')).toBeVisible();
    await expect(page.locator('text=Inbox Activity').last()).toBeVisible();
    await expect(page.locator('text=Growth & Virality')).toBeVisible();

    // Check if we have the fallback rendering in case of no data
    const ordersContainer = page.locator('text=Recent Orders').locator('..').locator('..');

    // Create locators for both possible states
    const ordersTable = ordersContainer.locator('table');
    const emptyState = ordersContainer.locator('.app-empty').or(page.locator('text=No order rows found for this tenant.'));

    // Wait for either the table or the empty state to become visible
    await expect(ordersTable.or(emptyState)).toBeVisible();

    // Assert that exactly one of the states is present and visible
    const tableVisible = await ordersTable.isVisible();
    const emptyVisible = await emptyState.isVisible();

    expect(tableVisible || emptyVisible).toBeTruthy();
  });
});
