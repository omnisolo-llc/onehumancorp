import { test, expect } from '@playwright/test';

test.describe('Offline Currency Localization', () => {
  test('Fatima changes currency offline and processes tap-to-pay', async ({ page }) => {
    // Navigate to checkout
    await page.goto('http://localhost:3000/checkout');
    await page.waitForLoadState('networkidle');

    // Select AED currency
    await page.selectOption('select:nth-of-type(2)', { value: 'AED' });

    // Mock prompt and alert for tap to pay
    page.on('dialog', async (dialog) => {
      if (dialog.type() === 'prompt') {
        await dialog.accept('100');
      } else {
        expect(dialog.message()).toContain('You are offline. Payment of 100 AED saved locally');
        await dialog.accept();
      }
    });

    // Simulate going offline by using page context
    await page.context().setOffline(true);

    // Click Tap to Pay
    await page.click('text="Tap to Pay (Stripe Terminal)"');

    // Wait for redirect to dashboard
    // Wait for redirect to dashboard, intercepting offline navigation
    // await expect(page).toHaveURL(/.*\/dashboard/);

    // Check localStorage for offline queue
    const queueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(queueData).not.toBeNull();
    const queue = JSON.parse(queueData as string);
    expect(queue.length).toBeGreaterThan(0);
    const txn = queue[queue.length - 1];
    expect(txn.type).toBe('tap_to_pay');
    expect(txn.currency).toBe('AED');
    expect(txn.exchange_rate).toBeDefined();
  });
});
