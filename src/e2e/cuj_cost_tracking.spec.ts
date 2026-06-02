import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByText('Total Costs', { exact: true })).toBeVisible();
    await expect(page.getByText('LLM Usage', { exact: true })).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees', { exact: true })).toBeVisible();

    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Current Plan' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Estimated Next Bill' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Your Current Usage' })).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();
  });

  test('Owner checks current plan and views cost dashboard', async ({ page }) => {
    // Start from dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Check elements dynamically populated on My Plan screen by clicking "Billing"
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // Verify My Plan Screen
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Check elements dynamically populated
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');

    // View Cost Details
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard Screen
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();

    // Verify dynamic metrics are populated
    await expect(page.locator('#cost-dashboard-period')).toContainText('Period:');
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();

    // Back to My Plan
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // View Upgrade Plans
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();

    // Verify Pricing Screen
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();
  });
});
