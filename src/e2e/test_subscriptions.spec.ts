import { test, expect } from '@playwright/test';

test.describe('AI-Driven Subscription & Membership Lifecycle Management', () => {
  test('Subscription offer generation UI', async ({ page }) => {
    // We are testing the UI logic directly for the new mobile-first HTML file
    await page.goto('/ui/subscription-offer-generator.html');

    // Ensure we are testing the mobile viewport layout
    await page.setViewportSize({ width: 375, height: 667 });

    // Verify container width doesn't cause horizontal scroll
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(375);

    await expect(page.locator('h1')).toHaveText('Recurring Membership');
  });
});
