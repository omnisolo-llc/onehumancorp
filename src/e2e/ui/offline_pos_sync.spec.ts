import { test, expect } from '@playwright/test';

// Skip this test in CI until it can be wired fully with the docker-compose backend setup and mock card reader.
// @skip
test.describe.skip('Offline Mobile Sync & Tap-to-Pay Architecture', () => {
  test('should process an offline payment and sync it when online', async ({ page, context }) => {
    // Navigate to terminal
    await page.goto('/pos/terminal');

    // Simulate terminal setup logic, we will just click the new order and pretend it's connected
    // This is simplified as the UI needs a pin to unlock.
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();

    // Clock in
    await page.getByText('Clock In').click();

    // Connect to a mocked reader
    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    const connectButton = page.getByText('Connect').first();
    if (await connectButton.isVisible()) {
        await connectButton.click();
    }

    // Go offline
    await context.setOffline(true);

    // Process payment
    await page.getByText('Charge $50.00').click();

    // Should show offline success
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    // Verify it's in the queue (localStorage)
    const queueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(queueData).toContain('tap_to_pay');

    // Wait for sync to happen. Without network mocking, it goes through the actual api endpoints.
    // Go online
    await context.setOffline(false);

    await page.waitForTimeout(2000);

    // Verify queue is empty
    const updatedQueueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(updatedQueueData).toBe('[]');
  });
});
