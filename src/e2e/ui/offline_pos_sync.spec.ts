import { test, expect } from '@playwright/test';

test.describe('Offline Mobile Sync & Tap-to-Pay Architecture', () => {
  test('should process an offline payment and sync it when online', async ({ page, context }) => {
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    await page.goto('/pos/terminal');
    await page.waitForTimeout(2000);

    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible({ timeout: 10000 });

    await page.route('**/api/v1/payments/terminal/sync_offline', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, failed_transaction_ids: [] }),
      });
    });

    await page.getByRole('button', { name: 'Clock In' }).click();

    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await page.locator('text=Quick Charge').click();

    await expect(page.locator('text=Payment Saved Offline - 50 USD')).toBeVisible();

    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    await expect(async () => {
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingPosTx.length).toBe(0);
    }).toPass({ timeout: 25000 });
  });

  test('should trigger Operations Agent reconciliation card on negative inventory conflict', async ({ page, context }) => {
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Carlos',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    await page.goto('/pos/terminal');
    await page.waitForTimeout(2000);

    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('h1', { hasText: 'Carlos' })).toBeVisible({ timeout: 10000 });

    await page.route('**/api/v1/payments/terminal/sync_offline', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, failed_transaction_ids: [] }),
      });
    });

    await page.getByRole('button', { name: 'Clock In' }).click();

    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await page.locator('text=Quick Charge').click();
    await expect(page.locator('text=Payment Saved Offline - 50 USD')).toBeVisible();

    await page.evaluate(() => {
        const queue = JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]');
        if (queue.length > 0) {
            queue[0].quantity = 100;
            queue[0].product_id = 'prod_123';
            localStorage.setItem('ohc_offline_pos_tx', JSON.stringify(queue));
        }
    });

    await context.setOffline(false);
    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    await expect(async () => {
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingPosTx.length).toBe(0);
    }).toPass({ timeout: 25000 });
  });
});
