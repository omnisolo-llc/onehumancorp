import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  // Use MOCK_BACKEND=true injected via env
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page }) => {

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');
    // 3. Check for My Plan components
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // We check for elements that match the static structure in cost-dashboard/page.tsx
    await expect(page.locator('text="Current Plan"').first()).toBeVisible();
    await expect(page.locator('text="AI Actions Used"').first()).toBeVisible();
    await expect(page.locator('text="Storage Used"').first()).toBeVisible();
    await expect(page.locator('text="Estimated Next Bill"').first()).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade' }).first()).toBeVisible();

    // 4. Click Upgrade
    await page.locator('button', { hasText: 'Upgrade' }).first().click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});
