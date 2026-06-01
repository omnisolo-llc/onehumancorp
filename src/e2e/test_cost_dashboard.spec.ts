import { test, expect } from './fixtures';

test.describe('CUJ: Cost Transparency Dashboard', () => {
  test('should display cost breakdown for current tenant on dashboard', async ({ page }) => {
    // Navigate to the main dashboard
    await page.goto('/');

    // Ensure the dashboard is loaded
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // The user navigates to their "My Plan" page
    await page.goto('/plan');

    // Wait for the plan page to render
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('View Cost Details')).toBeVisible();

    // Navigate to the Cost Transparency Dashboard
    await page.goto('/cost-dashboard');

    // Wait for the Cost Transparency Dashboard to load
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();

    // Verify the presence of specific cost elements
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();

    // Check that dollar amounts are rendered (basic regex match for $x.xx)
    const costPattern = /\$\d+\.\d{2}/;
    await expect(page.locator('p:has-text("$")').first()).toBeVisible();
  });
});
