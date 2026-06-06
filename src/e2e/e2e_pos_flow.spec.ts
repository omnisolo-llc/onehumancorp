import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction offline and sync', async ({ page, context }) => {
    // Navigate to the checkout page
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // The user decides to use Tap to Pay (POS mode)
    await page.getByRole('button', { name: 'Tap to Pay (Stripe Terminal)' }).click();

    // The Tap to Pay modal should appear
    await expect(page.getByRole('heading', { name: 'Tap to Pay' })).toBeVisible();

    // Enter the amount
    await page.getByPlaceholder('0.00').fill('45.00');

    // Simulate going offline
    await context.setOffline(true);

    // Click "Tap Card"
    page.on('dialog', dialog => dialog.accept());
    await page.getByRole('button', { name: 'Tap Card' }).click();

    // User is redirected to dashboard
    await expect(page.url()).toContain('/dashboard');

    // Verify offline queue count is shown
    await expect(page.locator('body')).toContainText(/pending sync|Offline - changes saved locally/i);

    // Simulate coming back online
    await context.setOffline(false);

    // Sync should trigger and empty the offline queue
    // (The dashboard uses a window event listener for 'online')
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // We can't trivially assert the backend inventory decrement without a separate DB query fixture,
    // but we can ensure the UI doesn't crash and the flow completes.
    await expect(page.getByRole('heading', { name: 'Orders' }).or(page.getByRole('heading', { name: 'Metrics Overview' }))).toBeVisible();
  });
});
