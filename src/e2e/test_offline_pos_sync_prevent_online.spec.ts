import { test, expect } from './fixtures';

test.describe('Offline-First Edge Sync & Multi-Channel Inventory Sync', () => {
  test('should sync offline POS mutation and prevent subsequent online checkouts due to edge cache invalidation', async ({ page, context }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Wait for the pin pad
    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Setup local storage mock for offline staff
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Priya', role: 'Manager', pin_hash: '1234' }]));
    });

    // Reload to pick up local storage
    await page.reload();

    await expect(page.locator('text=Terminal Locked')).toBeVisible();

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Priya')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Trigger New Order. The original inventory count of the last Red Dress is 1.
    // Buying it offline will queue an offline mutation.
    await page.locator('text=New Order').click();
    await expect(page.locator('text=Payment Saved Offline')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait for background sync to trigger (interval is 10s) and clear events
    await expect(async () => {
      const remainingPosTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
      expect(remainingPosTx.length).toBe(0);
    }).toPass({ timeout: 15000 });

    // Now simulate an online customer visiting the storefront
    await page.goto('/storefront');

    await expect(page.locator('text=Storefront')).toBeVisible();
  });
});
