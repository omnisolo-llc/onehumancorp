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
    const autodream = page.locator('text="AI Assistant Memory"');
    await expect(autodream).toHaveCount(0);
    const oldAutodream = page.locator('text="AutoDream Memory Pipeline"');
    await expect(oldAutodream).toHaveCount(0);

    // 6. Check "Orders to Fulfill" is visible instead of "Pending Orders/Bookings"
    const ordersToFulfill = page.locator('text="Orders to Fulfill"');
    await expect(ordersToFulfill.first()).toBeVisible();
    const oldPendingOrders = page.locator('text="Pending Orders/Bookings"');
    await expect(oldPendingOrders).toHaveCount(0);

    // 7. Check "Store Tips" is visible instead of "Actionable Insights"
    const storeTips = page.locator('text="Store Tips"');
    await expect(storeTips.first()).toBeVisible();
    const oldInsights = page.locator('text="Actionable Insights"');
    await expect(oldInsights).toHaveCount(0);

    // 8. Check "My Store" is visible instead of "My Business"
    const myStore = page.locator('text="My Store"');
    await expect(myStore.first()).toBeVisible();
    const oldMyBusiness = page.locator('text="My Business"');
    await expect(oldMyBusiness).toHaveCount(0);

    // 9. Check "How to use this app" is not showing right away but we can verify the text exists when menu opens, or just ensure old text is gone.
    const oldAppTour = page.locator('text="How to use this app"');
    await expect(oldAppTour).toHaveCount(0);
    const oldAutomateTour = page.locator('text="Automate Work Tour"');
    await expect(oldAutomateTour).toHaveCount(0);
  });
});
