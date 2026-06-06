import { test, expect } from './fixtures';

test.describe('Cost Miser E2E Flow', () => {
  test('should verify My Plan, Pricing, and Cost Dashboard', async ({ page }) => {
    // 1. Navigate to My Plan
    await page.goto('/plan');

    // Verify "My Plan" heading
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Verify current plan and next bill estimated
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();

    // Verify AI Actions Used section
    await expect(page.getByText('AI Actions Used')).toBeVisible();

    // Verify Storage Used section
    await expect(page.getByText('Storage Used')).toBeVisible();

    // 2. Navigate to Cost Dashboard from My Plan
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await page.waitForURL('**/cost-dashboard');

    // Verify Cost Dashboard heading
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Verify Cost Breakdown components
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-network')).toBeVisible();
    await expect(page.locator('#cost-dashboard-bandwidth-savings')).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();

    // 3. Navigate back to My Plan
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await page.waitForURL('**/plan');

    // 4. Navigate to Pricing Plans from My Plan
    // There are multiple ways to go to pricing, let's use the one in the header
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();
    await page.waitForURL('**/pricing');

    // Verify Pricing Plans heading
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    // Verify Tiers
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();

    // 5. Upgrade to Starter and hit checkout
    await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();
    await page.waitForURL('**/checkout?tier=Starter');

    // Verify Checkout heading
    await expect(page.getByRole('heading', { name: 'Secure Checkout' })).toBeVisible();

    // Click Cancel to go back to pricing
    await page.getByRole('button', { name: 'Cancel' }).click();
    await page.waitForURL('**/pricing');
  });
});
