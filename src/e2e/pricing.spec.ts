import { test, expect } from './fixtures';

test.describe('Pricing & Cost Dashboard CUJ', () => {

  test('Persona: Business Owner can view their current plan limits', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // 1. Verify Plan details
    await expect(page.getByRole('heading', { name: /My Plan/i })).toBeVisible();
    await expect(page.getByText(/Current Plan/i)).toBeVisible();

    // Assuming default is "Free"
    await expect(page.getByText('Free', { exact: true })).toBeVisible();

    await expect(page.getByText(/AI Actions Used/i)).toBeVisible();
    await expect(page.getByText(/Storage Used/i)).toBeVisible();
    await expect(page.getByText(/Estimated Next Bill/i)).toBeVisible();
  });

  test('Persona: Business Owner can view the cost dashboard breakdown', async ({ page }) => {
    // 1. Navigate to My Plan, then Cost Dashboard
    await page.goto('/');
    await page.getByRole('button', { name: 'Billing', exact: true }).click();
    await page.getByRole('button', { name: /View Cost Details/i }).click();
    await page.waitForURL('**/cost-dashboard');

    // 2. Verify Cost Dashboard details
    await expect(page.getByRole('heading', { name: /Business Advisory Dashboard/i })).toBeVisible();
    await expect(page.getByText(/Cost Breakdown/i)).toBeVisible();

    await expect(page.getByText(/LLM Usage/i)).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText(/Payment Fees/i)).toBeVisible();
  });

  test('Persona: Business Owner can view upgrade pricing tiers', async ({ page }) => {
    // 1. Navigate to My Plan, then Pricing
    await page.goto('/');
    await page.getByRole('button', { name: 'Billing', exact: true }).click();
    await page.getByRole('button', { name: /View Upgrade Plans/i }).click();
    await page.waitForURL('**/pricing');

    // 2. Verify Pricing Tiers
    await expect(page.getByRole('heading', { name: /Pricing Plans/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();
  });

});
