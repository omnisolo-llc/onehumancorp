import { test, expect } from '../fixtures';

test.describe('Kitchen View - Food Cart Daily Operations', () => {
  test('should display translated orders and support offline-first inventory toggling', async ({ page, context }) => {
    // Navigate to Kitchen Command Center
    await page.goto('/kitchen');

    // Verify responsive mobile layout setup
    await page.setViewportSize({ width: 375, height: 667 });

    // Verify Kitchen Command Center Header
    await expect(page.getByRole('heading', { name: 'Kitchen Command Center' })).toBeVisible();

    // Verify translated orders
    await expect(page.getByText('Alice')).toBeVisible();
    await expect(page.getByText('بدون بصل')).toBeVisible(); // "No onions" in Arabic
    await expect(page.getByText('خبز إضافي')).toBeVisible(); // "Extra pita" in Arabic

    // Verify "Mark Sold Out" capability
    const falafelToggle = page.locator('#sold-out-toggle-falafel');
    await expect(falafelToggle).toBeVisible();
    await expect(falafelToggle).toContainText('Mark Sold Out');

    // Simulate going offline to test the optimistic UI updates & queue
    await context.setOffline(true);

    // Click 'Mark Sold Out'
    await falafelToggle.click();

    // Verify UI reflects the change optimistically
    await expect(falafelToggle).toContainText('Sold Out');
    await expect(page.locator('#queue-dashboard')).toBeVisible();
    await expect(page.locator('#queue-dashboard')).toContainText('1 Pending Sync');

    // Simulate going online
    await context.setOffline(false);

    // Click 'Mark Ready & Notify' on the first order to test another action
    const markReadyBtn = page.getByRole('button', { name: 'Mark Ready & Notify' }).first();
    await markReadyBtn.click();

    // Check that there is only 1 new order left
    await expect(page.getByRole('button', { name: 'Mark Ready & Notify' })).toHaveCount(1);

  });
});
