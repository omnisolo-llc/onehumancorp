import { test, expect } from '@playwright/test';

test.describe('Offline-Tolerant Mobile Sync Protocol', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

  test('should queue actions offline and sync when online', async ({ page, context }) => {
    // Navigate to a page that supports offline actions (e.g., an agent feed mock)
    await page.goto('/test-offline-sync');

    // Ensure we start online
    await context.setOffline(false);

    // Verify online indicator or initial state
    await expect(page.locator('#network-status')).toHaveText('Online');

    // Go offline
    await context.setOffline(true);
    await expect(page.locator('#network-status')).toHaveText('Offline');

    // Perform an action that should be queued
    await page.click('#trigger-action-btn');

    // Optimistic UI check (translucent glass "Pending Sync" state)
    const actionCard = page.locator('.action-card');
    await expect(actionCard).toHaveClass(/pending-sync/);
    await expect(actionCard.locator('.sync-icon')).toBeVisible();

    // Verify localStorage has the queued item
    const queueData = await page.evaluate(() => localStorage.getItem('ohc_offline_actions_queue'));
    expect(queueData).not.toBeNull();
    const queue = JSON.parse(queueData!);
    expect(queue.length).toBeGreaterThan(0);
    expect(queue[0].status).toBe('pending');

    // Reconnect to network
    await context.setOffline(false);
    await expect(page.locator('#network-status')).toHaveText('Online');

    // The app should automatically sync.
    // Wait for the UI to update from "Pending Sync" to "Done"
    await expect(actionCard).not.toHaveClass(/pending-sync/);
    await expect(actionCard).toHaveClass(/completed/);
    await expect(actionCard.locator('.sync-icon')).not.toBeVisible();

    // Verify localStorage queue is cleared or marked completed
    const finalQueueData = await page.evaluate(() => localStorage.getItem('ohc_offline_actions_queue'));
    if (finalQueueData) {
        const finalQueue = JSON.parse(finalQueueData);
        expect(finalQueue.length === 0 || finalQueue.every((a: any) => a.status === 'completed')).toBeTruthy();
    }
  });
});
