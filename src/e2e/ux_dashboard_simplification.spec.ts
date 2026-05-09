import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Simplification', () => {
  test('Verify simplified language and 5 key actions on Dashboard', async ({ page }) => {
    // Navigate from the home page
    await page.goto('/');

    // Wait for the app to load
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(5000);

    // Assert simplified "Team Activity" button is present instead of "Agent Activity"
    const teamActivityBtn = page.getByRole('button', { name: 'Team Activity' });
    await expect(teamActivityBtn).toBeVisible();

    // Assert 5 key actions are available from the Dashboard view

    // 1. Add Product
    const addProductBtn = page.getByText('+');
    await expect(addProductBtn).toBeVisible();

    // 2. View Orders
    const viewPastOrdersBtn = page.getByRole('button', { name: 'View Past Orders' });
    const viewOrdersBtn = page.getByRole('button', { name: 'View Orders' });

    const isViewPastVisible = await viewPastOrdersBtn.isVisible();
    const isViewOrdersVisible = await viewOrdersBtn.isVisible();
    expect(isViewPastVisible || isViewOrdersVisible).toBeTruthy();

    // 3. Check Messages
    const checkMessagesBtn = page.getByRole('button', { name: 'Messages' });
    await expect(checkMessagesBtn).toBeVisible();

    // 4. See Analytics
    const seeAnalyticsBtn = page.getByRole('button', { name: 'Analytics' });
    await expect(seeAnalyticsBtn).toBeVisible();

    // 5. Share Store
    const shareStoreBtn = page.getByText('🔗');
    await expect(shareStoreBtn).toBeVisible();

    // Test clicking one of them to ensure it works
    await checkMessagesBtn.click();

  });
});
