import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('pricing');

test.describe('Dynamic Pricing Tiers UI', () => {
  test('should display dynamic pricing settings on pricing page', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Check if the page title is visible
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    // Ensure we can see some pricing options or limits
    await expect(page.locator('text=Agent')).first().toBeVisible({ timeout: 20000 });

    // Check for other dynamic formats
    await expect(page.locator('text=AI action')).first().toBeVisible({ timeout: 20000 });
    await expect(page.locator('text=Storage Quota')).first().toBeVisible({ timeout: 20000 });
    await expect(page.locator('text=Product')).first().toBeVisible({ timeout: 20000 });
  });
});
