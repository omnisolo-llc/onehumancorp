import { test, expect } from './fixtures';

test.describe('Cost Dashboard & Plan Limits UI', () => {
  test('should display the cost dashboard and check expected sections', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Navigate to the Cost Dashboard directly
    await page.goto('/cost-dashboard.html');

    // Wait for the main heading to be visible
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Verify key sections are present
    await expect(page.getByText('Total Costs').first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'LLM Usage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Storage Savings' }).first()).toBeVisible();

    // Check if the plan navigation link is present
    await expect(page.locator('button', { hasText: 'Back to My Plan' })).toBeVisible();
  });

  test('should display my plan limits and route to pricing', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Go to My Plan page
    await page.goto('/plan.html');

    // Wait for the main heading to be visible
    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 15000 });

    // Verify data placeholders or limits are populated
    await expect(page.getByText('Estimated Next Bill:').first()).toBeVisible();
    await expect(page.getByText('AI actions used this month').first()).toBeVisible();

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
    await page.goto('/pricing.html');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    // Ensure the starter upgrade button is visible
    const starterButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(starterButton).toBeVisible();

    // The redirect logic changes the URL, so we can verify the checkout or error loads
    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('/api/billing/create-checkout-session') && req.method() === 'POST'),
      starterButton.click()
    ]);
    expect(request.postDataJSON()).toMatchObject({ tier: 'Starter' });
  });
});
