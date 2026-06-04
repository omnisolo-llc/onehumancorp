import { test, expect } from './fixtures';

test.describe('Cost Engineering & Pricing Logic Verification', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard and wait for network
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('My Plan screen fetches and displays real data instead of mock defaults', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Navigate to My Plan
    await page.locator('nav a:has-text("My Plan")').click();

    // Wait for the screen to become visible
    const myPlanScreen = page.locator('#my-plan-screen');
    await expect(myPlanScreen).toBeVisible();

    // Verify it doesn't just say Loading... indefinitely
    await expect(page.locator('#my-plan-name')).not.toHaveText('Loading...');

    // It should fetch Free tier defaults (since we're signed in as E2E test user)
    await expect(page.locator('#my-plan-name')).toContainText('Plan: Free');

    // Verify AI Usage reflects real data format (e.g., "AI Actions Used: X / 100")
    // Wait for the loading placeholder to be replaced
    await expect(page.locator('#my-plan-ai-usage')).not.toHaveText('Loading...', { timeout: 10000 });
    await expect(page.locator('#my-plan-ai-usage')).toContainText('AI Actions Used: ');

    // Verify Storage Usage
    await expect(page.locator('#my-plan-storage-usage')).not.toHaveText('Loading...');
    await expect(page.locator('#my-plan-storage-usage')).toContainText('Storage Used: ');

    // Verify Next Bill
    await expect(page.locator('#my-plan-next-bill')).not.toHaveText('Loading...');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill: $');
  });

  test('Cost Transparency Dashboard fetches and displays real data', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Navigate to Cost Dashboard
    await page.goto('/cost-dashboard');

    const costScreen = page.locator('#cost-dashboard-screen');
    await expect(costScreen).toBeVisible();

    // Verify dynamic metrics replace the Loading... text
    await expect(page.locator('#cost-dashboard-period')).not.toHaveText('Loading...', { timeout: 10000 });
    await expect(page.locator('#cost-dashboard-period')).toContainText('Period: ');

    // Verify cost breakdown rows populate with formatted currency
    await expect(page.locator('#cost-dashboard-llm')).not.toHaveText('Loading...');
    await expect(page.locator('#cost-dashboard-llm')).toContainText('$');

    await expect(page.locator('#cost-dashboard-storage')).not.toHaveText('Loading...');
    await expect(page.locator('#cost-dashboard-storage')).toContainText('$');

    await expect(page.locator('#cost-dashboard-payment-fees')).not.toHaveText('Loading...');
    await expect(page.locator('#cost-dashboard-payment-fees')).toContainText('$');

    await expect(page.locator('#cost-dashboard-total')).not.toHaveText('Loading...');
    await expect(page.locator('#cost-dashboard-total')).toContainText('$');

    await expect(page.locator('#cost-dashboard-revenue')).not.toHaveText('Loading...');
    await expect(page.locator('#cost-dashboard-revenue')).toContainText('$');
  });
});
