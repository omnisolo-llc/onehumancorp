import { test, expect } from './fixtures';

test.describe('Billing & Rate Limits', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display cost transparency and billing', async ({ page }) => {
    await page.goto('/dashboard');

    // From dashboard, go to My Plan
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Check elements on My Plan page
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();

    // From My Plan, view cost details
    await page.locator('button', { hasText: 'View Cost Details' }).click();
    await expect(page).toHaveURL(/.*cost-dashboard/);
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Verify Cost Breakdown
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();

    // Verify Cost Summary
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();

    // Go to pricing
    await page.locator('button', { hasText: 'Back to My Plan' }).click();
    await expect(page).toHaveURL(/.*plan/);

    await page.locator('button', { hasText: 'Change Plan' }).click();
    await expect(page).toHaveURL(/.*pricing/);
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    // Verify plans listed
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });
});
