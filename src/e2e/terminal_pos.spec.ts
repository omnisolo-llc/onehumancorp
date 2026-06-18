import { test, expect } from './fixtures';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {

  test.beforeEach(async ({ memberPage }) => {
    // Navigate to POS terminal path
    await memberPage.goto(`/ui/pos.html`);

    // Ensure display initializes to $0.00
    await expect(memberPage.locator('#amount-display')).toHaveText('$0.00');
  });

  test('Processes quick charge UI and reserves payment offline', async ({ memberPage, context }) => {
    // Tap 5, 0, 0, 0 to create $50.00 charge
    await memberPage.locator('button.num-btn', { hasText: '5' }).first().click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();
    await memberPage.locator('button.num-btn', { hasText: '0' }).nth(0).click();

    await expect(memberPage.locator('#amount-display')).toHaveText('$50.00');

    // Click "Charge"
    await memberPage.locator('#charge-btn').click();

    // Ensure overlay is visible
    await expect(memberPage.locator('#tap-overlay')).toBeVisible();
    await expect(memberPage.locator('#tap-amount-subtitle')).toHaveText('$50.00');

    // Set network to offline BEFORE simulating tap
    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Ensure network indicator says "Working Offline"
    await expect(memberPage.locator('#network-status-text')).toHaveText('Working Offline', { timeout: 5000 });

    // Click Simulate Tap Button
    await memberPage.locator('#simulate-tap-btn').click();

    // Verify it drops back to receipt screen showing offline queue message
    await expect(memberPage.locator('#receipt-screen')).toBeVisible();
    await expect(memberPage.locator('.receipt-text')).toHaveText('Payment saved offline. Will sync when network is restored.');
  });
});
