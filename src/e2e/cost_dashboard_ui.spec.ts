import { test, expect } from './fixtures';

test.describe('Cost Dashboard & Plan Limits UI', () => {
  test('should display the cost dashboard and check expected sections', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Navigate to the Cost Dashboard directly
    await page.goto('/cost-dashboard');

    // Wait for the main heading to be visible
    await expect(page.locator('h2', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Verify key sections are present
    await expect(page.locator('div', { hasText: 'Total Costs' })).toBeVisible();
    await expect(page.locator('div', { hasText: 'LLM Usage' }).first()).toBeVisible();
    await expect(page.locator('div', { hasText: 'Storage' }).first()).toBeVisible();
    await expect(page.locator('div', { hasText: 'Network & Storage Savings' }).first()).toBeVisible();

    // Check if the cost-dashboard navigation link is present
    await expect(page.getByRole('button', { name: 'Back to Dashboard' })).toBeVisible();
  });

  test('should display my cost-dashboard limits and route to pricing', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Go to My Plan page
    await page.goto('/cost-dashboard');

    // Wait for the main heading to be visible
    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 15000 });

    // Verify data placeholders or limits are populated
    await expect(page.locator('div', { hasText: 'Estimated Next Bill' })).toBeVisible();
    await expect(page.locator('div', { hasText: 'AI actions used this month' })).toBeVisible();

    // Verify actions
    const upgradeButton = page.locator('button', { hasText: 'Upgrade' }).first();
    await expect(upgradeButton).toBeVisible();

    // Click on upgrade to ensure it leads to the pricing page
    await upgradeButton.click();

    // Expect to land on pricing
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });
  });

  test('should verify checkout routing works from pricing', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    // Ensure the starter upgrade button is visible
    const starterButton = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    await expect(starterButton).toBeVisible();

    // Attempt clicking the upgrade path
    await starterButton.click();

    // The redirect logic changes the URL, so we can verify the checkout or error loads
    await page.waitForURL(/\/checkout\?tier=Starter/);
    await expect(page.getByText('Plan Upgrade')).toBeVisible({ timeout: 15000 });
  });
});
