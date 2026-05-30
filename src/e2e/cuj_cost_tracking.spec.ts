import { test, expect } from './fixtures';

test.describe.configure({ mode: 'serial' });

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display plan and cost transparency details', async ({ page }) => {
    // Navigate to the Dashboard, which is the starting point for CUJs.
    // The fixtures automatically perform the UI login as Admin and land on /dashboard.
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // The user wants to check their plan and usage details by clicking a UI link.
    await page.getByRole('link', { name: /Plan/i }).first().click();

    // Verify "My Plan" page
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('Current Plan')).toBeVisible();
    await expect(page.getByText('Free', { exact: true })).toBeVisible();
    await expect(page.getByText('Your Current Usage')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    // From My Plan page, user clicks to view cost details via the UI button
    await page.getByRole('button', { name: 'View Cost Details', exact: false }).click();

    // Verify "Cost Transparency" page
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();

    // Verify sections on the Cost Transparency page
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });
});
