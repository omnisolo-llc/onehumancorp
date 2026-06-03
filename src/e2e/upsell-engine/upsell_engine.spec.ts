import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Product Bundling and Upsell Engine', () => {
  test('CUJ: Verify upsell engine UI flow', async ({ page }) => {
    // Go to the upsell engine mobile UI page directly for testing
    await page.goto('http://localhost:3000/upsell-engine');

    await expect(page.locator('h1').first()).toContainText('Upsell Engine');

    // Simulate adding item and showing upsells
    await page.click('button:has-text("Simulate Add to Cart")');

    // Wait for mock API response and UI update. Should be 2 upsell cards.
    // Ensure we increase timeout as next.js dev might be slow
    await expect(page.locator('.upsell-card')).toHaveCount(2, { timeout: 30000 });

    // Accept one of the upsells
    await page.locator('.upsell-card button:has-text("Add")').first().click();

    // Verify it was added
    await expect(page.locator('.cart-item:has-text("Matching Scented Candles")')).toBeVisible();

    // Wait for dashboard to test the card visibility (simulated navigation for E2E purposes)
    await page.goto('http://localhost:3000/dashboard');
    await expect(page.locator('text=AI Upsell Revenue').first()).toBeVisible({ timeout: 30000 });
  });
});
