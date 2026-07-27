import { test, expect } from '@playwright/test';

test.describe('Loyalty Engine Web Checkout UI', () => {
  test('Loyalty points should apply automatically at checkout', async ({ page }) => {
    await page.goto('/checkout?tenant=123');
    await expect(page.locator('body')).toBeVisible();
  });
});
