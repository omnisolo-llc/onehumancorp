import { test, expect } from './fixtures';
import { Client } from 'pg';

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

    // Verify unlock
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Check inventory before via DB
    const client = new Client({ connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc' });
    await client.connect();
    const resBefore = await client.query('SELECT inventory_count FROM products WHERE id = $1', ['e2e-product-cake']);
    const inventoryBefore = resBefore.rows[0].inventory_count;

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Trigger New Order
    await page.locator('text=New Order').click();
    await expect(page.locator('text=Payment Saved Offline - 50 USD')).toBeVisible();

    // Add a Stripe terminal component mock check if needed or navigate to payment
    // Since the terminal component needs internet, we would simulate offline queuing here

    // Perform an offline clock in
    await page.locator('text=Clock In').click();
    await expect(page.locator('text=Clocked In')).toBeVisible();

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

    // Verify inventory reduced by 1
    // The worker might take a moment to process the queue, so we retry
    await expect(async () => {
        const resAfter = await client.query('SELECT inventory_count FROM products WHERE id = $1', ['e2e-product-cake']);
        const inventoryAfter = resAfter.rows[0].inventory_count;
        expect(inventoryAfter).toBe(inventoryBefore - 1);
    }).toPass({ timeout: 10000 });

    await client.end();
  });
});
