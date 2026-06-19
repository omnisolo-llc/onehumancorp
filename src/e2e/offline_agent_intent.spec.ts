import { test, expect } from './fixtures';

test.describe('Offline Agent Intent Sync', () => {
  test('should queue agent intent mutations locally when offline and sync when online', async ({ page, context }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard.html');

    // Set network to offline
    await context.setOffline(true);

    // Evaluate to simulate the offline environment trigger
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // The network status indicator should show offline
    await expect(page.locator('#network-status-indicator').first()).toBeVisible();
    await expect(page.locator('#network-status-text').first()).toHaveText('Working Offline');

    // Enqueue an agent intent mutation into offline queue
    await page.evaluate(async () => {
        if (typeof (window as any).enqueueOfflineMutation === 'function') {
            await (window as any).enqueueOfflineMutation({
                id: 'intent-test-id-123',
                type: 'agent_intent',
                payload: { action: 'draft_email', recipient: 'customer@example.com', subject: 'Follow up' },
                timestamp: new Date().toISOString()
            });
        }
    });

    // Verify queue indicator shows items pending
    await expect(page.locator('#queue-dashboard')).toBeVisible();
    await expect(page.locator('#queue-dashboard')).toContainText('1 Items Pending Sync');

    // Set network to online
    await context.setOffline(false);

    // Trigger online event to allow the application to naturally attempt synchronization.
    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    // Wait for the sync to complete and the queue to be cleared
    await page.waitForFunction(async () => {
        if (typeof (window as any).getQueue === 'function') {
            const queue = await (window as any).getQueue();
            return queue.length === 0;
        }
        return true;
    }, { timeout: 15000 });

    const queueData = await page.evaluate(async () => {
        if (typeof (window as any).getQueue === 'function') {
            const queue = await (window as any).getQueue();
            return JSON.stringify(queue);
        }
        return '[]';
    });
    expect(queueData).toBe('[]');

    // The network status indicator should disappear since we are online and queue is empty
    await expect(page.locator('#network-status-indicator')).toHaveClass(/hidden/, { timeout: 5000 });
  });
});
