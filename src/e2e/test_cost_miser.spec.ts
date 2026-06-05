import { test, expect } from './fixtures';

test.describe('Cost Miser E2E Flow', () => {
  test('should verify My Plan, Pricing, and Cost Dashboard', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await page.waitForURL('**/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
    await expect(page.locator('#cost-dashboard-network')).toBeVisible();
    await expect(page.locator('#cost-dashboard-bandwidth-savings')).toBeVisible();
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();

    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await page.waitForURL('**/plan');

    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();
    await page.waitForURL('**/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();

    await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();
    await page.waitForURL('**/checkout?tier=Starter');
    await expect(page.getByRole('heading', { name: 'Secure Checkout' })).toBeVisible();

    await page.getByRole('button', { name: 'Cancel' }).click();
    await page.waitForURL('**/pricing');
  });
});
