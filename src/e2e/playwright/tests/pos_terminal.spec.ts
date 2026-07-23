import { test, expect } from '@playwright/test';

test.describe('POS Terminal - Online and Offline Sync', () => {

  test('Online transaction processes successfully', async ({ page }) => {
    // Navigate to the POS page
    await page.goto('/pos');

    // Add items to cart
    await page.click('button:has-text("Custom Cake")');
    await page.click('button:has-text("Repair Kit")');

    // Verify cart total
    await expect(page.locator('text=$70.00')).toBeVisible();

    // Process payment
    await page.click('button:has-text("Charge via Tap-to-Pay")');

    // Verify success message
    await expect(page.locator('text=Payment processed successfully!')).toBeVisible();

    // Verify cart is empty
    await expect(page.locator('text=$0.00')).toBeVisible();
  });

  test('Offline transaction is queued and synced', async ({ page, context }) => {
    await page.goto('/pos');

    // Simulate going offline
    await context.setOffline(true);

    // Add item
    await page.click('button:has-text("Consultation Hour")');

    // Process payment (while offline)
    await page.click('button:has-text("Charge via Tap-to-Pay")');

    // Verify offline message
    await expect(page.locator('text=Saved Offline. Will sync when connection is restored.')).toBeVisible();

    // Verify queue in localStorage
    const queueData = await page.evaluate(() => localStorage.getItem("pos_offline_queue"));
    expect(queueData).not.toBeNull();
    const queue = JSON.parse(queueData!);
    expect(queue.length).toBe(1);
    expect(queue[0].total).toBe(100);

    // Simulate going back online
    await context.setOffline(false);

    // Wait for auto-sync effect to trigger and complete
    await expect(page.locator('text=Offline transactions synced successfully!')).toBeVisible();

    // Verify queue is cleared
    const newQueueData = await page.evaluate(() => localStorage.getItem("pos_offline_queue"));
    expect(newQueueData).toBeNull();
  });
});
