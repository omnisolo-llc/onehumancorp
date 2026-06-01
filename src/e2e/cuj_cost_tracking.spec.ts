import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // 1. Start from home page
    await page.goto('/');

    // 2. Wait for main UI to load
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 3. Navigate to 'My Plan' / Billing view
    await page.getByRole('button', { name: 'Billing' }).first().click();
    await expect(page.getByRole('heading', { name: 'My Plan & Usage' }).first()).toBeVisible();

    // 4. Navigate to Cost Dashboard from My Plan
    await page.getByRole('button', { name: 'View Cost Details' }).first().click();

    // 5. Assert the visibility of Cost Transparency Dashboard and specific cost elements
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('LLM Inference Cost')).toBeVisible();
    await expect(page.getByText('Storage & CDN')).toBeVisible();
    await expect(page.getByText('Payment Processor Fees')).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();

    // 6. Assert specific values correctly rendered (either 0 or populated)
    // The fetch might return 0s, but the elements should have $ symbol
    await expect(page.locator('#cost-dashboard-total')).toContainText('$');
    await expect(page.locator('#cost-dashboard-revenue')).toContainText('$');
    await expect(page.locator('#cost-dashboard-llm')).toContainText('$');
    await expect(page.locator('#cost-dashboard-storage')).toContainText('$');
    await expect(page.locator('#cost-dashboard-payment-fees')).toContainText('$');
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
