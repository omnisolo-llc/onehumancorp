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

    // Enqueue an agent intent mutation into localStorage
    await page.evaluate(() => {
        let queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
        queue.push({
            id: 'intent-test-id-123',
            type: 'agent_intent',
            payload: { action: 'draft_email', recipient: 'customer@example.com', subject: 'Follow up' },
            timestamp: new Date().toISOString()
        });
        localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
        // Trigger queue update
        window.dispatchEvent(new Event('ohc_queue_updated'));
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
    await page.waitForFunction(() => {
        const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
        return queue.length === 0;
    }, { timeout: 15000 });

    const queueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(queueData).toBe('[]');

    // The network status indicator should disappear since we are online and queue is empty
    await expect(page.locator('#network-status-indicator')).toHaveClass(/hidden/, { timeout: 5000 });
  });
});
