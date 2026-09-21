import { test, expect } from './fixtures';

test.describe('Business Analytics Widget Soft Paywall', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('should display the analytics widget with basic metrics', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.getByText('Total Sales')).toBeVisible();
    await expect(page.getByText('Low Stock')).toBeVisible();
  });

  test('should display the locked advanced AI analytics upgrade CTA', async ({ page }) => {
    const paywall = page.getByTestId('ai-feature-paywall');
    await expect(paywall).toBeVisible();
    await expect(paywall.getByRole('heading', { name: 'Advanced AI Analytics' })).toBeVisible();
    await expect(paywall.getByText(/products are driving revenue/i)).toBeVisible();

    const upgradeLink = paywall.getByRole('link', { name: /Upgrade to Pro/ });
    await expect(upgradeLink).toBeVisible();
    await expect(upgradeLink).toHaveAttribute('href', '/pricing');
  });
});
