import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('should display Pricing Plans page', async ({ page }) => {
    await page.goto('/pricing.html');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display FAQ section', async ({ page }) => {
    await page.goto('/pricing.html');
    await expect(page.locator('h2', { hasText: 'Frequently Asked Questions' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'How do I upgrade, downgrade, or cancel?' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'What is the storage limit?' })).toBeVisible();
  });

  test('should display all four pricing tiers', async ({ page }) => {
    await page.goto('/pricing.html');
    await expect(page.locator('.plan-name', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Business' })).toBeVisible();
  });

  test('should verify Back button functions', async ({ page }) => {
    await page.goto('/pricing.html');
    const backButton = page.locator('a', { hasText: 'Back to Dashboard' });
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page.url()).toContain('/dashboard.html');
  });

  test('should verify upgrade button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('checkout.stripe.com');
  });

  test('should verify upgrade to Pro button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Pro via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('checkout.stripe.com');
  });

  test('should verify upgrade to Business button routes to checkout', async ({ page }) => {
    await page.goto('/pricing.html');
    const upgradeButton = page.locator('button', { hasText: 'Upgrade to Business via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await expect(page.url()).toContain('checkout.stripe.com');
  });
});
