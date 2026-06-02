import { test, expect } from '@playwright/test';

test.describe('Offline Multi-Currency Checkout CUJ', () => {
    test('Fatima can accept USD offline, which is saved locally and synced later', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');

        // Emulate going offline
        await page.context().setOffline(true);

        // Go to checkout
        await page.goto('/checkout');

        // Change currency to USD
        const currencySelect = page.locator('select').nth(1); // the second select
        await currencySelect.selectOption('USD');

        // Assert the toast notification appears
        await expect(page.getByText(/Converted using yesterday's rate/)).toBeVisible();

        // Tap to Pay
        // We have to mock the prompt
        page.on('dialog', dialog => dialog.accept('100'));
        await page.getByText('Tap to Pay (Stripe Terminal)').click();

        // Wait for redirect to dashboard
        await expect(page).toHaveURL(/\/dashboard/);

        // Check offline queue via local storage evaluation
        const offlineQueue = await page.evaluate(() => {
            return JSON.parse(window.localStorage.getItem('ohc_offline_queue') || '[]');
        });

        expect(offlineQueue.length).toBeGreaterThan(0);
        expect(offlineQueue[0].amount).toBe(100);
        expect(offlineQueue[0].currency).toBe('USD');
        expect(offlineQueue[0].type).toBe('tap_to_pay');

        // Reconnect
        await page.context().setOffline(false);

        // Trigger a storage event to force a sync if needed, or wait for the online listener
        await page.evaluate(() => {
            window.dispatchEvent(new Event('online'));
        });

        // Wait for sync to clear the queue
        await page.waitForFunction(() => {
            const queue = JSON.parse(window.localStorage.getItem('ohc_offline_queue') || '[]');
            return queue.length === 0;
        }, { timeout: 10000 });

        const finalQueue = await page.evaluate(() => {
            return JSON.parse(window.localStorage.getItem('ohc_offline_queue') || '[]');
        });
        expect(finalQueue.length).toBe(0);
    });
});
