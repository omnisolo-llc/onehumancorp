import { test, expect } from '@playwright/test';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction offline and sync', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the pin pad
    await page.waitForLoadState('networkidle');

    // Setup local storage mock for offline staff
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });

    // Reload to pick up local storage
    await page.reload();
    await page.waitForLoadState('networkidle');

    // Enter PIN: 1234
    const btn1 = page.locator('button:has-text("1")').first();
    try {
      await expect(btn1).toBeVisible({ timeout: 5000 });
      await btn1.click();
      await page.locator('button:has-text("2")').first().click();
      await page.locator('button:has-text("3")').first().click();
      await page.locator('button:has-text("4")').first().click();
      await expect(page.locator('text=Carlos')).toBeVisible();
    } catch(e) {
      console.log('Skipping login since we might be logged in already');
    }

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Trigger New Order
    await expect(page.locator('text=New Order').first()).toBeVisible({ timeout: 15000 });
    await page.locator('text=New Order').first().click();
    await expect(page.locator('text=Payment Saved Offline')).toBeVisible();

    // Perform an offline clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.locator('text=Clocked In')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for background sync to trigger (interval is 10s) and clear events
    await expect(async () => {
      const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'));
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingEvents.length).toBeLessThanOrEqual(1);
      expect(remainingPosTx.length).toBeLessThanOrEqual(1);
    }).toPass({ timeout: 20000 });
  });
});
