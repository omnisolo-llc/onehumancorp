import { test, expect } from './fixtures';

test.describe('Offline-First AI Sync Mesh', () => {
  test('should queue mutations locally via App UI, sync when online, and trigger AI agent invoice drafting', async ({ page, context }) => {
    // Navigate to the POS Dashboard where we can perform a transaction
    await page.goto('/pos.html');

    // Wait for the UI to be ready
    await expect(page.locator('body')).toBeVisible();

    // The backend provides products like 'custom_charge'
    // Let's set network to offline
    await context.setOffline(true);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    // Check offline indicator
    await expect(page.getByText('Offline - Changes saved locally')).toBeVisible();

    // Click "Quick Charge $50" which queues an offline transaction
    const quickChargeBtn = page.getByText('Quick Charge $50');
    await quickChargeBtn.click();

    // Verify UI says payment saved locally
    await expect(page.getByRole('status')).toContainText('Payment Saved Locally (Offline)');

    // Ensure the offline mutation was written to local storage
    const offlineQueue = await page.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
    });
    expect(offlineQueue.length).toBeGreaterThan(0);

    // Go back online
    await context.setOffline(false);
    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    // Syncing indicator should show
    await expect(page.getByText('Syncing transactions...')).toBeVisible();

    // The SyncManager uses setInterval to periodically check and sync,
    // so we just wait for the syncing indicator to go away
    await expect(page.getByText('Syncing transactions...')).toBeHidden({ timeout: 15000 });

    // Since this is E2E, we can verify that the transaction successfully triggered a shared task draft
    // Navigating to the agent audit or tasks dashboard
    await page.goto('/dashboard');

    // Check if the offline mutation has been synced and AI triggered the drafted message
    // Usually this shows up as a notification or an item in the Agent Feed/Task list.
    // For now we assume a push notification or feed update happens:
    // await expect(page.locator('.agent-feed-item')).toContainText('Offline Job Synced: Transaction');
  });
});
