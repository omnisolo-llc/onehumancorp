import { test, expect } from '@playwright/test';

test.describe('AI Agent Department Architecture', () => {
  test('CUJ: Order processing and Action Review Center draft', async ({ page }) => {
    test.setTimeout(90000);

    // Navigate to home and start the real E2E flow
    await page.goto('/');

    // Standard login flow
    await page.waitForLoadState('networkidle', { timeout: 15000 });

    const emailLocator = page.locator('input[type="email"], input[name="username"]').first();
    await emailLocator.fill('maya@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login"), button:has-text("Sign In")').first().click();

    const newOrderLocator = page.locator('button:has-text("New Order")').first();
    await newOrderLocator.click();
    await page.locator('input[placeholder="Customer Name"]').first().fill('E2E Test User');
    await page.locator('input[placeholder="Order Details"]').first().fill('Custom Cake');
    await page.locator('button:has-text("Create Order")').first().click();

    const arcLocator = page.locator('text=Action Review Center').first();
    await arcLocator.click();
    const draftCard = page.locator('text=Draft confirmation for order event');

    await expect(draftCard).toBeVisible({ timeout: 15000 });
    await page.locator('button', { hasText: 'Approve' }).first().click();
    await expect(draftCard).not.toBeVisible();
  });
});
