import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page }) => {

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // 3. Check for My Plan components
    await expect(page.locator('text=My Plan').first()).toBeVisible();
    await expect(page.locator('text=Current Plan').first()).toBeVisible();
    await expect(page.locator('text=AI Actions Used').first()).toBeVisible();
    await expect(page.locator('text=Storage Used').first()).toBeVisible();
    await expect(page.locator('text=Estimated Next Bill').first()).toBeVisible();
    await expect(page.locator('button:has-text("Upgrade")').first()).toBeVisible();

    // 4. Click Upgrade
    await page.locator('button:has-text("Upgrade")').click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});
