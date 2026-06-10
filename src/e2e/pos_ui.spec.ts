import { test, expect } from './fixtures';
import { pool } from './global-setup';

test.describe('In-Person POS UI', () => {
  test('CUJ: Navigates from dashboard to POS, enters amount, taps, and sees receipt', async ({ adminPage }) => {
    // 1. Navigation from Dashboard
    await adminPage.goto('/dashboard.html');
    await adminPage.waitForSelector('#dashboard-title');

    // Find and click the Quick Charge POS link
    const posLink = adminPage.locator('a:has-text("Quick Charge POS")');
    await expect(posLink).toBeVisible();
    await posLink.click();

    // Ensure we landed on the POS view
    await adminPage.waitForSelector('#amount-display');
    await expect(adminPage.locator('#amount-display')).toHaveText('$0.00');

    // 2. Numpad Amount Entry Validation
    // Enter $15.00
    await adminPage.locator('button.num-btn:has-text("1")').click();
    await adminPage.locator('button.num-btn:has-text("5")').click();
    await adminPage.locator('button.num-btn:has-text("00")').click();

    await expect(adminPage.locator('#amount-display')).toHaveText('$15.00');

    // 3. Trigger Accept Contactless Payment
    const chargeBtn = adminPage.locator('#charge-btn');
    await expect(chargeBtn).toBeEnabled();
    await chargeBtn.click();

    // Wait for the tap overlay
    const overlay = adminPage.locator('#tap-overlay');
    await expect(overlay).toBeVisible();
    await expect(adminPage.locator('#tap-amount-subtitle')).toHaveText('$15.00');

    // 4. Simulate Tap to Pay processing
    const simulateBtn = adminPage.locator('#simulate-tap-btn');
    await expect(simulateBtn).toBeVisible();

    // Intercept network request to terminal intent if necessary, or let it fire in the background
    // Click simulate
    await simulateBtn.click();
    await expect(simulateBtn).toHaveText('Processing...');

    // 5. Success receipt display
    const receiptScreen = adminPage.locator('#receipt-screen');
    await expect(receiptScreen).toBeVisible({ timeout: 5000 });

    await expect(adminPage.locator('#receipt-amount')).toHaveText('$15.00');
    await expect(adminPage.locator('.receipt-text')).toHaveText('Payment Successful');

    // Return to New Sale
    await adminPage.locator('button:has-text("New Sale")').click();
    await expect(adminPage.locator('#amount-display')).toHaveText('$0.00');
    await expect(overlay).not.toBeVisible();
    await expect(receiptScreen).not.toBeVisible();
  });
});
