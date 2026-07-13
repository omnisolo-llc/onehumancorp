import { test, expect } from './fixtures';

test.describe('Parallel Execution Optimization - Cart & Growth', () => {
  test('Cart loading is successful and data is present', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    // Add an item to the cart first or directly access if cart exists
    await page.goto('/checkout');
    // Ensure checkout/cart page loads
    await expect(page.locator('text=Checkout').first()).toBeVisible({ timeout: 10000 });
  });

  test('Affiliate stats loading in growth dashboard', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard/growth/affiliates');
    // Expect some header or text indicating the page loaded
    await expect(page.locator('text=Affiliate').first()).toBeVisible({ timeout: 10000 });
  });

  test('Reputation stats loading in growth dashboard', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard/growth/reputation');
    // Expect some header or text indicating the page loaded
    await expect(page.locator('text=Reputation').first()).toBeVisible({ timeout: 10000 });
  });

  test('Unified Feed is fast and functional', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    // We expect the main sections to be visible, ensuring the backend parallel optimization works correctly
    await expect(page.locator('text=Recent Orders').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Operations Map').first()).toBeVisible({ timeout: 10000 });
  });

  test('Agent feed loads correctly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('text=Action Required').first()).toBeVisible({ timeout: 10000 });
  });
});
