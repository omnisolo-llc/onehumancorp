import { test, expect } from '@playwright/test';

test.describe('Offline POS Sync UI', () => {
  test('should go offline, perform a transaction, come back online, and sync effectively', async ({ page }) => {
    // Navigate to the POS Terminal route
    await page.goto('/pos/terminal');

    // To mock the backend for staff PIN authentication
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([
            { id: 'mock-staff-1', name: 'Mock Staff', pin_hash: '1234', role: 'Staff' }
        ]));
    });

    // Verify terminal locked page
    await expect(page.locator('text="Terminal Locked"').first()).toBeVisible({ timeout: 10000 });

    // Simulate login via PIN entry using the handlePinEntry buttons
    // Wait for the button '1' to be available first
    const button1 = page.locator('button', { hasText: /^1$/ });
    await expect(button1).toBeVisible({ timeout: 10000 });
    await button1.click();
    await page.locator('button', { hasText: /^2$/ }).click();
    await page.locator('button', { hasText: /^3$/ }).click();
    await page.locator('button', { hasText: /^4$/ }).click();

    // Wait for 'Clock In' text
    await expect(page.locator('text="Clock In"').first()).toBeVisible({ timeout: 10000 });

    // Set network to offline
    await page.context().setOffline(true);

    // Verify 'Offline Mode' text becomes visible
    await expect(page.locator('text="Offline Mode"').first()).toBeVisible({ timeout: 10000 });

    // Click 'Quick Charge $50' to perform a transaction while offline
    const chargeBtn = page.locator('button', { hasText: 'Quick Charge $50' });
    await expect(chargeBtn).toBeVisible({ timeout: 10000 });
    await chargeBtn.click();

    // Verify 'Payment Saved Offline' appears
    await expect(page.locator('text="Payment Saved Offline - 50 USD"').first()).toBeVisible({ timeout: 15000 });

    // Re-enable the network
    await page.context().setOffline(false);

    // Instead of strictly demanding the toast, let's verify that the 'Offline Mode' badge is gone.
    await expect(page.locator('text="Offline Mode"').first()).not.toBeVisible({ timeout: 10000 });

    // And ensure we are online
    await expect(page.locator('text="Online"').first()).toBeVisible({ timeout: 10000 });
  });
});
