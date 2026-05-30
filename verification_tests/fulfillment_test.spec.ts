import { test, expect } from '@playwright/test';

test.describe('Fulfillment Checkout CUJ', () => {
  test('Merchant checkout should display dynamic fulfillment options', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'maya@onehumancorp.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');
    await expect(page.locator('h1:has-text("Good Morning")')).toBeVisible();
    await page.click('button:has-text("Upgrade Plan")');
    await expect(page.locator('h1:has-text("Pricing")')).toBeVisible();
    await page.click('button:has-text("Upgrade to Pro via Stripe")');
    await expect(page.locator('h1:has-text("Checkout")')).toBeVisible();
    await expect(page.locator('text=You are 3 miles away!')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('input[value="pickup"]')).toBeVisible();
    await expect(page.locator('input[value="local_delivery"]')).toBeVisible();
    await expect(page.locator('input[value="shipping"]')).toBeVisible();
    await page.check('input[value="local_delivery"]');
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Pay Now")');
    await expect(page.locator('h1:has-text("Good Morning")')).toBeVisible();
  });
});
