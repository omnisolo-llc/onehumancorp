import { test, expect } from './fixtures';

test.describe('Trending Stores Widget Soft Paywall', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should display locked trending stores insights with upgrade CTA', async ({ page }) => {
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    await expect(dashboard.getByText('Trending Stores Insights')).toBeVisible();
    await expect(dashboard.getByText('Unlock trending market data to see what other successful businesses are selling.')).toBeVisible();

    const upgradeBtns = dashboard.getByRole('button', { name: 'Upgrade to Pro' });
    // Since there are two "Upgrade to Pro" buttons (one in Business Analytics and one in Trending Stores),
    // we need to locate the specific one in the Trending Stores section.
    const trendingStoresSection = dashboard.locator('section').filter({ hasText: 'Trending Stores Insights' });
    const upgradeBtn = trendingStoresSection.getByRole('button', { name: 'Upgrade to Pro' });

    await expect(upgradeBtn).toBeVisible();

    // Set up dialog handler
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Upgrade to Pro to access Trending Stores Insights?');
      await dialog.accept();
    });

    await upgradeBtn.click();

    // Verify it navigates to pricing-screen
    const pricingScreen = page.locator('#pricing-screen');
    await expect(pricingScreen).toBeVisible();
  });
});
