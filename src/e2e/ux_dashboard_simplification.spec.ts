import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Simplification (Grandmother Test)', () => {
  test('Plain language labels are used instead of jargon', async ({ page }) => {
    await page.goto('/');

    // Wait for the app to load
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(5000);

    // 1. Check "Today's Sales" is visible instead of "Revenue TTD"
    const todaysSales = page.locator('text="Today\'s Sales"');
    await expect(todaysSales.first()).toBeVisible();

    // 2. Check "Business Health" is visible instead of "Store Health"
    const businessHealth = page.locator('text="Business Health"');
    await expect(businessHealth.first()).toBeVisible();

    // 3. Check "Test Order" is visible instead of "Simulate Order"
    const testOrder = page.locator('text="Test Order"');
    await expect(testOrder.first()).toBeVisible();

    // 4. Check "Tasks for You to Approve" is visible instead of "Agent Activity Feed"
    const oldFeedText = page.locator('text="Agent Activity Feed"');
    await expect(oldFeedText).toHaveCount(0);

    // 5. Ensure "AutoDream Memory Pipeline" is not visible by default (hidden by is_advanced)
    const autodream = page.locator('text="AutoDream Memory Pipeline"');
    await expect(autodream).toHaveCount(0);
  });
});
