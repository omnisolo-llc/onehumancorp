import { test, expect } from './fixtures';

test.describe('CUJ: Full Cost Dashboard Experience', () => {
  test('Owner navigates to Cost Dashboard and verifies margin indicator', async ({ page }) => {
    // Start from dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Navigate to Billing / My Plan
    await page.getByRole('button', { name: 'Billing', exact: true }).click();
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // Go to Cost Dashboard
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard Screen
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();

    // Check that our new margin metrics are present
    await expect(page.locator('#cost-dashboard-margin')).toBeVisible();
    await expect(page.getByText('Margin Indicator')).toBeVisible();

    // Verify that the fallback handles the display correctly
    const totalRevenueText = await page.locator('#cost-dashboard-revenue').textContent();
    const marginText = await page.locator('#cost-dashboard-margin').textContent();

    // Ensure it doesn't show NaN
    expect(marginText).not.toBe('NaN%');
  });
});
