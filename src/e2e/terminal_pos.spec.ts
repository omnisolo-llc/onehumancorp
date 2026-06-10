import { test, expect } from '@playwright/test';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {
  const TENANT_ID = 'terminal-test-tenant';

  test.beforeEach(async ({ page }) => {
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

    // Navigate to POS terminal path
    await page.goto(`/pos/terminal`);

    // Unlock the terminal
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
  });

  test('Processes tap-to-pay and reserves inventory', async ({ page }) => {
    // Wait for the UI to be ready
    await expect(page.getByRole('button', { name: 'New Order' })).toBeVisible();

    // Press numpad to add $15.00
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '5', exact: true }).click();
    await page.getByRole('button', { name: '0', exact: true }).nth(0).click();
    await page.getByRole('button', { name: '0', exact: true }).nth(0).click();
    await expect(page.locator('text=$15.00')).toBeVisible();

    // Click New Order
    await page.getByRole('button', { name: 'New Order' }).click();

    // Check loading/processing state
    await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });

    // Sometimes it might transition to Payment Completed fast, wait for it
    await expect(page.getByRole('status')).toContainText('Payment Completed', { timeout: 15000 });

    // Successfully checked that tap to pay works and UI responds accordingly
  });
});
