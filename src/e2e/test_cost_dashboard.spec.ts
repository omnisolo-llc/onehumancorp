import { test, expect } from './fixtures';

test.describe('CUJ: Cost Dashboard Navigation and Content', () => {
  test('Owner can navigate to cost dashboard and view metrics', async ({ page }) => {
    // Navigate from root
    await page.goto('/');

    const dashLink = page.getByRole('link', { name: 'Go to Dashboard' });
    if (await dashLink.isVisible()) {
      await dashLink.click();
    } else {
      const loginBtn = page.getByRole('button', { name: 'Log In' });
      if (await loginBtn.isVisible()) {
        await loginBtn.click();
      }
    }

    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // From dashboard, go to billing first
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // Now from billing, navigate to cost dashboard
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify header
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();

    // Verify advisory summary
    await expect(page.getByRole('heading', { name: 'Advisory Summary' })).toBeVisible();

    // Verify Cost Transparency section
    await expect(page.getByRole('heading', { name: 'Cost Transparency', exact: true })).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();

    // Verify Cost Breakdown section
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();

    // Test back navigation
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.locator('#my-plan-screen')).toBeVisible();
  });
});
