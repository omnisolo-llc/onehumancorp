import { test, expect } from '@playwright/test';

test.describe('POS Terminal Offline Sync', () => {
  test('should handle offline transaction and sync when back online', async ({ page, context }) => {
    // Navigate to home to log in properly via local storage bypass
    await page.goto('/');

    // Ensure we have some staff in local storage for testing offline PIN entry
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'tenant_1');
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        { id: 'staff_1', name: 'Test Staff', pin_hash: '1234', tenant_id: 'tenant_1', role: 'Cashier' }
      ]));
    });

    await page.goto('/pos/terminal');

    // Wait for the PIN entry to appear
    await expect(page.locator('button:has-text("1")').first()).toBeVisible({ timeout: 15000 });

    // Enter PIN '1234'
    await page.locator('button:has-text("1")').first().click();
    await page.locator('button:has-text("2")').first().click();
    await page.locator('button:has-text("3")').first().click();
    await page.locator('button:has-text("4")').first().click();

    // Wait for the quick actions section to appear
    await expect(page.locator('text="Quick Actions"').first()).toBeVisible({ timeout: 15000 });

    // Simulate going offline
    await context.setOffline(true);

    // Verify offline indicator
    await expect(page.locator('text="Offline Mode"')).toBeVisible();

    // Trigger an offline transaction (e.g. Quick Charge)
    await page.click('text="Quick Charge $50"');

    // Verify UI says payment saved offline
    await expect(page.locator('text="Payment Saved Offline"').first()).toBeVisible();

    // Verify local storage has the transaction
    const offlineTx = await page.evaluate(() => localStorage.getItem('ohc_offline_pos_tx'));
    expect(offlineTx).toBeTruthy();
    const parsed = JSON.parse(offlineTx || '[]');
    expect(parsed.length).toBeGreaterThan(0);

    // Go back online
    await context.setOffline(false);

    // Verify online indicator
    await expect(page.locator('text="Online"').first()).toBeVisible();

    // Wait for sync resolution (localStorage cleared)
    // we use a while loop inside evaluate rather than toPass since Playwright in our environment
    // might evaluate expect blocks too fast or the component sync logic requires network idle
    await page.waitForFunction(() => {
      const tx = localStorage.getItem('ohc_offline_pos_tx');
      return tx === '[]' || !tx;
    }, { timeout: 25000 });
  });
});
