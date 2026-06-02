import { test, expect } from '@playwright/test';

test.describe('Billing and Cost Management CUJ', () => {

  test('Persona: Business Owner views current plan details', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Navigate to My Plan page
    await page.goto('/plan');

    // Verify it landed on the My Plan page
    await expect(page.getByRole('heading', { name: /My Plan/i, exact: true })).toBeVisible();

    // Verify current plan snapshot
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');

    // Verify usage section
    await expect(page.getByText('Your Current Usage')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();
  });

  test('Persona: Business Owner views cost transparency dashboard', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Navigate to Cost Dashboard
    await page.goto('/cost-dashboard');

    // Verify it landed on the Cost Transparency Dashboard
    await expect(page.getByRole('heading', { name: /Cost Transparency Dashboard/i, exact: true })).toBeVisible();

    // Verify overview
    await expect(page.locator('#cost-dashboard-period')).toContainText('Period:');
    await expect(page.getByText('Total Costs')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();

    // Verify cost breakdown
    await expect(page.getByText('Cost Breakdown')).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });

  test('Persona: Business Owner reviews pricing plans', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Navigate to Pricing Plans
    await page.goto('/pricing');

    // Verify it landed on the Pricing Plans page
    await expect(page.getByRole('heading', { name: /Pricing Plans/i, exact: true })).toBeVisible();

    // Verify all 4 tiers are present
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();
  });

  test('Persona: Business Owner clicks to upgrade to Starter', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Navigate to Pricing Plans
    await page.goto('/pricing');

    // Click Upgrade to Starter
    await page.getByRole('button', { name: /Upgrade to Starter via Stripe/i }).click();

    // Verify redirection to checkout page with correct tier query param
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
  });

  test('Persona: Business Owner clicks to upgrade to Pro', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Navigate to Pricing Plans
    await page.goto('/pricing');

    // Click Upgrade to Pro
    await page.getByRole('button', { name: /Upgrade to Pro via Stripe/i }).click();

    // Verify redirection to checkout page with correct tier query param
    await expect(page).toHaveURL(/.*\/checkout\?tier=Pro/);
  });
});
