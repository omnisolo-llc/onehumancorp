import { test, expect } from './fixtures';

test.describe('Cost Dashboard and Pricing CUJ', () => {
  test('Non-technical owner can navigate cost dashboard, plan, and pricing pages seamlessly', async ({ page }) => {
    // 1. Owner logs into the dashboard
    await page.goto('/dashboard');
    await expect(page).toHaveTitle(/OneHumanCorp/i); // Adjusted title assumption based on standard behavior

    // 2. Owner navigates to 'My Plan'
    await page.goto('/plan');
    await expect(page.locator('h1')).toContainText('My Plan');

    // Check that usage limits exist (soft limits UI should be there)
    await expect(page.locator('text=AI Actions Used')).toBeVisible();
    await expect(page.locator('text=Storage Used')).toBeVisible();

    // 3. Owner clicks 'View Cost Details'
    await page.click('text=View Cost Details');
    await expect(page).toHaveURL(/\/cost-dashboard/);
    await expect(page.locator('h1')).toContainText('Business Advisory Dashboard');

    // Check cost breakdown elements
    await expect(page.locator('text=Total Costs')).toBeVisible();
    await expect(page.locator('text=LLM Usage')).toBeVisible();
    await expect(page.locator('text=Storage')).toBeVisible();
    await expect(page.locator('text=Payment Fees')).toBeVisible();

    // 4. Owner navigates to Pricing to review plans
    await page.click('text=Back to My Plan');
    await expect(page).toHaveURL(/\/plan/);
    await page.click('text=Change Plan');
    await expect(page).toHaveURL(/\/pricing/);

    await expect(page.locator('h1')).toContainText('Pricing Plans');
    // Ensure all 4 plans exist
    await expect(page.locator('h3:has-text("Free")')).toBeVisible();
    await expect(page.locator('h3:has-text("Starter")')).toBeVisible();
    await expect(page.locator('h3:has-text("Pro")')).toBeVisible();
    await expect(page.locator('h3:has-text("Business")')).toBeVisible();

    // 5. Owner attempts to upgrade to Starter
    await page.click('text=Upgrade to Starter via Stripe');
    await expect(page).toHaveURL(/\/checkout\?tier=Starter/);
    await expect(page.locator('h1')).toContainText('Checkout');
  });
});
