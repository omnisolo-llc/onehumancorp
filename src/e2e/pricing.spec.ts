import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('pricing');

test.describe('Dynamic Pricing Tiers UI', () => {
  test('should display dynamic pricing settings on pricing page', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Check if the page title is visible
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    // Wait explicitly for the network call to avoid flakiness, although not necessary if the default text renders first
    // In our case we expect to see 'Agent Limit' eventually
    await expect(page.locator('li:has-text("Agent Limit")').first()).toBeVisible({ timeout: 20000 });

    // Check for other dynamic formats
    await expect(page.locator('li:has-text("AI actions / month")').first()).toBeVisible({ timeout: 20000 });
    await expect(page.locator('li:has-text("Storage Quota")').first()).toBeVisible({ timeout: 20000 });
    await expect(page.locator('li:has-text("Products Limit")').first()).toBeVisible({ timeout: 20000 });
  });
});
