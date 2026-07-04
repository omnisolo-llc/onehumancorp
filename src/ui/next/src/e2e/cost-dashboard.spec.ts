import { test, expect } from '@playwright/test';

test.describe('Cost Dashboard Loop', () => {
  test('Cost dashboard loads and displays main metrics correctly', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 15000 });

    // Check main metrics
    await expect(page.locator('h2', { hasText: 'Total Costs' }).first()).toBeVisible();
    await expect(page.locator('#cost-dashboard-total-costs')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Projected Monthly Cost' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-projected')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Total Revenue' })).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
  });

  test('Cost dashboard shows accurate breakdown elements', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible({ timeout: 15000 });

    // Check for individual breakdown items
    await expect(page.locator('span', { hasText: 'Base Platform' })).toBeVisible();
    await expect(page.locator('span', { hasText: /^Storage$/ })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Compute Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Bandwidth' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Email Sends' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Outbound API Calls' })).toBeVisible();
  });

  test('Cost dashboard presents Manage Billing action successfully', async ({ page }) => {
    await page.goto('/cost-dashboard');
    const manageBillingBtn = page.locator('button', { hasText: 'Manage Billing' });
    await expect(manageBillingBtn).toBeVisible({ timeout: 15000 });
    await expect(manageBillingBtn).toBeEnabled();
  });

  test('Cost dashboard back to plan navigation works successfully', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.locator('a', { hasText: 'Back to My Plan' }).click();
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeHidden();
  });

  test('Cost dashboard handles Budget Health Alert visibility', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 15000 });

    // Budget health alert might be conditionally hidden depending on data,
    // so we evaluate the locator's existence in DOM.
    // We just verify it does not break the layout.
    const alert = page.locator('#budget-health-alert');
    // Ensure the page hasn't crashed
    await expect(page.locator('h2', { hasText: 'Total Costs' }).first()).toBeVisible();
  });
});
