import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('should display Pricing Plans page', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display all four pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h3', { hasText: 'Free' }).first()).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' }).first()).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' }).first()).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' }).first()).toBeVisible();
  });

  test('should verify Back button functions', async ({ page }) => {
    await page.goto('/pricing');
    const backButton = page.locator('button', { hasText: 'Back' }).first();
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page).toHaveURL(/\/dashboard$/);
  });

  test('should verify upgrade button routes to checkout', async ({ page }) => {
    await page.goto('/pricing');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page).toHaveURL(/\/checkout\?tier=Starter$/);
  });
});
