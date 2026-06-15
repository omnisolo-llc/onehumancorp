import { test, expect } from '@playwright/test';

test.describe('Offline Mobile Sync & Tap-to-Pay Architecture', () => {
  test('should process an offline payment and sync it when online', async ({ page, context }) => {
    await page.goto('/pos.html');

    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();

    await page.getByText('Clock In').click();

    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    const connectButton = page.getByText('Connect').first();
    if (await connectButton.isVisible()) {
        await connectButton.click();
    }

    await context.setOffline(true);

    await page.getByText('Charge $50.00').click();

    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    const queueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(queueData).toContain('tap_to_pay');

    await context.setOffline(false);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    await page.waitForTimeout(3000);

    const updatedQueueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(updatedQueueData).toBe('[]');
  });

  test('should trigger Operations Agent reconciliation card on negative inventory conflict', async ({ page, context }) => {
    await page.goto('/pos.html');

    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();
    await page.getByText('0').click();

    await page.getByText('Clock In').click();

    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    const connectButton = page.getByText('Connect').first();
    if (await connectButton.isVisible()) {
        await connectButton.click();
    }

    await context.setOffline(true);

    await page.getByText('Charge $50.00').click();
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible({ timeout: 10000 });

    await page.evaluate(() => {
        const queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
        if (queue.length > 0) {
            queue[0].quantity = 100; // Force conflict
            queue[0].product_id = 'prod_123';
            localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
        }
    });

    await context.setOffline(false);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    await page.waitForTimeout(8000);

    const updatedQueueData = await page.evaluate(() => localStorage.getItem('ohc_offline_queue'));
    expect(updatedQueueData).toBe('[]');

    await page.goto('/dashboard');

    await expect(page.getByText(/We oversold the item prod_123 by /)).toBeVisible({ timeout: 15000 });
  });
});
