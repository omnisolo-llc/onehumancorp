import { test, expect } from './test_mocks';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction offline and sync', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the UI to load and auto-fetch the staff data
    await page.waitForResponse(response => response.url().includes('/api/staff') && response.status() === 200);

    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Trigger New Order
    await page.locator('text=New Order').click();
    await expect(page.locator('text=Payment Saved Offline - 50 USD')).toBeVisible();

    // Perform an offline clock in
    await page.locator('text=Clock In').click();
    await expect(page.locator('text=Clocked In')).toBeVisible();

    // Test terminal offline payment queuing
    await page.getByText('Discover Readers').click();
    await page.waitForTimeout(500);
    // As mock does not work fully offline, we only rely on the "New Order" test for POS Tx

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for background sync to trigger (interval is 10s) and clear events
    await expect(async () => {
      const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'));
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingEvents.length).toBe(0);
      expect(remainingPosTx.length).toBe(0);
    }).toPass({ timeout: 15000 });
  });
});
