import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display my plan page and usage', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('Current Plan')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();
  });

  test('should display pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByText('Free')).toBeVisible();
    await expect(page.getByText('Starter')).toBeVisible();
    await expect(page.getByText('Pro')).toBeVisible();
    await expect(page.getByText('Business')).toBeVisible();
  });

  test('should display cost dashboard metrics', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });

  test('should navigate between plan, pricing, and cost dashboard', async ({ page }) => {
    await page.goto('/plan');
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    await page.goto('/plan');
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
  });

  test('should display checkout page for upgrade', async ({ page }) => {
    await page.goto('/pricing');
    await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Pay Now' })).toBeVisible();
  });
});
