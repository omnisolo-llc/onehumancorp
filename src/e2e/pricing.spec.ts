import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('should display Pricing Plans page', async ({ page }) => {
    await page.goto('/pricing.html');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display all four pricing tiers', async ({ page }) => {
    await page.goto('/pricing.html');
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('should verify Back button functions', async ({ page }) => {
    await page.goto('/pricing.html');
    const backButton = page.locator('button', { hasText: 'Back' });
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page.url()).toContain('/dashboard.html');
  });

  test('should verify upgrade button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('/checkout?tier=Starter');
  });

  test('should verify upgrade to Pro button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Pro via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('/checkout?tier=Pro');
  });

  test('should verify upgrade to Business button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Business via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('/checkout?tier=Business');
  });
});
