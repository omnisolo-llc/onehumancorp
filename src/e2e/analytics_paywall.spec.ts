import { test, expect } from './fixtures';

test.describe('Business Analytics Widget Soft Paywall', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
<<<<<<< HEAD
    await page.goto('/dashboard');
=======
    await page.goto('/');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.waitForLoadState('networkidle');
  });

  test('should display the analytics widget with basic metrics', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    await expect(dashboard.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(dashboard.getByText('Total Sales')).toBeVisible();
<<<<<<< HEAD
    await expect(dashboard.getByText('Low Stock')).toBeVisible();
  });

  test('should display locked advanced AI insights with upgrade CTA', async ({ page }) => {
=======
    await expect(dashboard.getByText('Visitors')).toBeVisible();
  });

  test('should display locked advanced AI insights with upgrade CTA', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    await expect(dashboard.getByText('Advanced AI Insights')).toBeVisible();
    await expect(dashboard.getByText('Unlock predictive analytics')).toBeVisible();

    const upgradeBtn = dashboard.getByRole('button', { name: 'Upgrade to Pro' });
    await expect(upgradeBtn).toBeVisible();

    // Set up dialog handler
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Upgrade to Pro to access Advanced AI Insights?');
      await dialog.accept();
    });

    await upgradeBtn.click();

    // Verify it navigates to pricing-screen
    const pricingScreen = page.locator('#pricing-screen');
    await expect(pricingScreen).toBeVisible();
  });
});
