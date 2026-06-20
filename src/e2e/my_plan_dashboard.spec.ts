import { test, expect } from './fixtures';

test.describe('My Plan and Cost Transparency Dashboard Screens', () => {
  test('My Plan screen routes to Pricing correctly', async ({ page }) => {
    // Navigate to My Plan
    await page.goto('/plan');

    // Check heading
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // Verify upgrade routing
    const upgradeButton = page.locator('button', { hasText: 'Upgrade' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('/pricing');
  });

  test('My Plan screen routes to Cost Transparency Dashboard correctly', async ({ page }) => {
    await page.goto('/plan');

    // Verify detailed costs routing
    const detailedCostsButton = page.locator('button', { hasText: 'View Detailed Costs' });
    await expect(detailedCostsButton).toBeVisible();
    await detailedCostsButton.click();
    await expect(page.url()).toContain('/cost-dashboard');
  });

  test('Cost Transparency Dashboard screen metrics are visible', async ({ page }) => {
    // Go directly to Cost Transparency Dashboard
    await page.goto('/cost-dashboard');

    // Check core metric elements visibility
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 10000 });

    // Verify elements by id or text mapped to their metrics
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-projected')).toBeVisible();
  });
});
