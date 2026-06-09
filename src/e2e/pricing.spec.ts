import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('should display Pricing Plans page', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display all four pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('should verify Back to Dashboard button functions', async ({ page }) => {
    await page.goto('/pricing');
    const backButton = page.locator('button', { hasText: 'Back to Dashboard' });
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page.url()).toContain('/dashboard');
  });

  test('should navigate to checkout when upgrading to Starter via Stripe', async ({ page }) => {
    await page.goto('/pricing');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('/checkout?tier=Starter');
  });
});
