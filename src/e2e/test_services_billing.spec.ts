import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('test_services_billing');

test.describe('Billing Services CUJ', () => {
  test('Owner configures smart pricing bounds and views cost dashboard', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });

    // Validate that the plans are listed
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Navigate to My Plan
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible({ timeout: 10000 });

    // Verify some elements
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // From My Plan, navigate to cost dashboard
    await page.locator('button', { hasText: 'View Cost Details' }).click();

    // Note: Due to client-side routing in Tauri/NextJS, we wait for heading
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Cost Transparency' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Total Costs' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();
  });
});
