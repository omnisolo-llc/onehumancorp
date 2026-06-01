import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Click "Billing" button to navigate to My Plan
    await page.getByRole('button', { name: 'Billing' }).click();

    // Verify My Plan screen is visible
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText(/^Plan: /)).toBeVisible();
    await expect(page.getByText(/^Estimated Next Bill: /)).toBeVisible();
    await expect(page.getByText(/^Storage Used: /)).toBeVisible();

    // Click "View Cost Details" button to navigate to Cost Dashboard
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard screen is visible
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();

    // Click "Back to My Plan"
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
  });
});
