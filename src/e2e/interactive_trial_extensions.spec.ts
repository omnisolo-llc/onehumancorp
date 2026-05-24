import { test, expect } from './fixtures';

test.describe('Interactive Trial Extensions', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should display initial trial days and allow user to extend trial by completing tasks', async ({ page }) => {
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    // Verify "Unlock More Time" section is visible
    await expect(dashboard.getByRole('heading', { name: 'Unlock More Time' })).toBeVisible();

    // Verify initial days is 14
    const daysLeftLocator = dashboard.locator('.text-5xl.font-outfit.font-bold');
    await expect(daysLeftLocator).toHaveText('14');

    // Click "Connect Twitter" task
    const connectTwitterBtn = dashboard.getByRole('button', { name: 'Connect' });
    await expect(connectTwitterBtn).toBeEnabled();
    await connectTwitterBtn.click();
    await expect(connectTwitterBtn).toBeDisabled();
    await expect(connectTwitterBtn).toHaveText('Connected');

    // Verify days increased by 7
    await expect(daysLeftLocator).toHaveText('21');

    // Click "Share Storefront" task
    const shareStorefrontBtn = dashboard.getByRole('button', { name: 'Share', exact: true });
    await expect(shareStorefrontBtn).toBeEnabled();
    await shareStorefrontBtn.click();
    await expect(shareStorefrontBtn).toBeDisabled();
    await expect(shareStorefrontBtn).toHaveText('Shared');

    // Verify days increased by 7
    await expect(daysLeftLocator).toHaveText('28');

    // Click "Refer a Friend" task
    const referFriendBtn = dashboard.getByRole('button', { name: 'Refer' });
    await expect(referFriendBtn).toBeEnabled();
    await referFriendBtn.click();
    await expect(referFriendBtn).toBeDisabled();
    await expect(referFriendBtn).toHaveText('Referred');

    // Verify days increased by 14
    await expect(daysLeftLocator).toHaveText('42');
  });
});
