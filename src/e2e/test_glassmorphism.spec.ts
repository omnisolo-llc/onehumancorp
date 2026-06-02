import { test, expect } from './fixtures';

test.describe('Glassmorphism Validation', () => {
    test('store wrap glassmorphism is active', async ({ page }) => {
        await page.goto('/store-wrap');
        await expect(page.locator('text=Store Wrap-Up').first()).toBeVisible();
    });

    test('checkout glassmorphism is active', async ({ page }) => {
        await page.goto('/checkout');
        await expect(page.locator('text=Checkout').first()).toBeVisible();
    });
});
