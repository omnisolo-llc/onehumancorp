import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Friction Fix Verification', () => {
  test('Grandmother Test: User navigates smoothly without jargon', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Test 1: Verify the label is Business Health, not Store Rating or Store Health
    const businessHealth = page.locator('text="Business Health"');
    await expect(businessHealth.first()).toBeVisible();

    // Test 2: The tooltip for Business Health should be clear and descriptive
    const helpBtn = page.locator('button:has-text("? Learn about Business Health")').first();
    const tooltipText = page.locator('text="Your Business Health is an AI-calculated score of your business\'s overall health and performance."');
    if (await helpBtn.isVisible()) {
      await helpBtn.click();
      await expect(tooltipText).toBeVisible();
    }
  });

  test('Plain language labels consistency check', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Test 3: Today's Sales is clear
    const todaysSales = page.locator('text="Today\'s Sales"');
    await expect(todaysSales.first()).toBeVisible();

    // Test 4: My Store label is present
    const myStore = page.locator('text="My Store"');
    await expect(myStore.first()).toBeVisible();

    // Test 5: Verify no "Revenue TTD" jargon
    const oldRevenue = page.locator('text="Revenue TTD"');
    await expect(oldRevenue).toHaveCount(0);
  });
});
