import { test, expect } from '@playwright/test';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('Cost Dashboard Loop', () => {
  test('Cost dashboard loads and displays data', async ({ page }) => {
    // Navigate to the dashboard page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });

    // Check that the Advisory Summary is present
    await expect(page.locator('h2', { hasText: 'Advisory Summary' })).toBeVisible();

    // Check that the Cost Transparency section is present
    await expect(page.locator('h2', { hasText: 'Cost Transparency' })).toBeVisible();

    // Check that Total Costs is displayed
    await expect(page.locator('h2', { hasText: 'Total Costs' })).toBeVisible();

    // Check that Cost Breakdown section is present
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();

    // Check for individual breakdown items
    await expect(page.locator('span', { hasText: 'LLM Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' })).toBeVisible();

    // Check Budget Alert section is present
    await expect(page.locator('h2', { hasText: 'Monthly Budget & Alerts' })).toBeVisible();

    // Check Budget Threshold Input
    const budgetInput = page.getByTestId('budget-threshold-input');
    await expect(budgetInput).toBeVisible();
    await budgetInput.fill('150');

    // Check Budget Notify Slider
    const notifySlider = page.getByTestId('budget-notify-slider');
    await expect(notifySlider).toBeVisible();
    await notifySlider.fill('90');

    // Save Budget Alert
    await page.locator('button', { hasText: 'Save Budget' }).click();
    await expect(page.locator('button', { hasText: 'Saved!' })).toBeVisible({ timeout: 5000 });

    // Reload the page and verify persistence
    await page.reload();
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByTestId('budget-threshold-input')).toHaveValue('150', { timeout: 10000 });
    await expect(page.getByTestId('budget-notify-slider')).toHaveValue('90', { timeout: 10000 });

    // Check navigation works
    await page.locator('button', { hasText: 'Back to My Plan' }).click();
    await expect(page).toHaveURL('/plan');
  });
});
