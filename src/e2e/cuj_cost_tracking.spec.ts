import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('Owner navigates to pricing page and validates plan visibility', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Check for starter recommendation
    await expect(page.locator('text=Recommended')).toBeVisible();
  });

  test('Owner checks current plan and usage limits', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Verify plan name and next bill
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');

    // Verify usage sections exist
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();
  });

  test('Owner views cost transparency dashboard metrics', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();

    await expect(page.locator('#cost-dashboard-period')).toContainText('Period:');
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();

    // Check breakdown
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
  });

  test('Owner explores upgrade flow from dashboard to pricing', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // Click change plan
    await page.getByRole('button', { name: 'Change Plan' }).click();

    // Validate we are on the pricing page
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
  });

  test('Owner checks cost details flow from plan page', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-screen')).toBeVisible();

    // Click view cost details
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Validate we are on the cost dashboard
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();
  });
});
