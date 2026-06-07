import { test, expect } from '@playwright/test';

test.describe('Offline POS Sync & Tap-to-Pay Architecture', () => {
  test('should queue transaction offline and sync when online', async ({ page, context }) => {
    // Navigate to the terminal screen
    await page.goto('http://localhost:8080/pos/terminal');

    // Add a pin for the terminal screen, if needed by the UI
    // Assuming '1', '2', '3', '4' works based on previous UI code review
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify terminal UI is unlocked and ready
    await expect(page.getByText('Discover Readers')).toBeVisible();

    // Simulate clicking Charge to queue an offline transaction
    // Wait, Stripe terminal discovery requires simulated connectivity, but we want to test offline queueing
    // So let's go offline first.
    await context.setOffline(true);

    // We navigate to checkout or click a tap-to-pay button that queues it
    await page.goto('http://localhost:8080/checkout');
    await expect(page.getByText('Tap to Pay (Stripe Terminal)')).toBeVisible();

    await page.getByText('Tap to Pay (Stripe Terminal)').click();

    // Verify offline status badge or message appears
    await expect(page.getByText('Payment Saved Offline')).toBeVisible();

    // Reconnect to network
    await context.setOffline(false);

    // Trigger online event to flush queue
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Go to dashboard and see if queue is 0, meaning it was flushed
    await page.goto('http://localhost:8080/dashboard');
    await expect(page.locator('.offline-queue-count')).toHaveText('0', { timeout: 10000 });
  });
});
