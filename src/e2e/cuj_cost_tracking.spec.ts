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
});
