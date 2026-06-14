import { test, expect } from './fixtures';
import { pool } from './global-setup';

test.describe('In-Person POS UI', () => {
  test('CUJ: Navigates from dashboard to POS, enters amount, taps, and sees receipt', async ({ page }) => {
    // 1. Navigation from Dashboard
    await page.goto('/dashboard.html');
    await page.waitForSelector('#dashboard-title');

    // Find and click the Quick Charge POS link
    const posLink = page.locator('a:has-text("Quick Charge POS")');
    await expect(posLink).toBeVisible();
    await posLink.click();

    // Ensure we landed on the POS view
    await page.waitForSelector('#amount-display');
    await expect(page.locator('#amount-display')).toHaveText('$0.00');

    // 2. Numpad Amount Entry Validation
    // Enter $15.00
    await page.locator('button.num-btn:has-text("1")').click();
    await page.locator('button.num-btn:has-text("5")').click();
    await page.locator('button.num-btn:has-text("00")').click();

    await expect(page.locator('#amount-display')).toHaveText('$15.00');

    // 3. Trigger Accept Contactless Payment
    const chargeBtn = page.locator('#charge-btn');
    await expect(chargeBtn).toBeEnabled();
    await chargeBtn.click();

    // Wait for the tap overlay
    const overlay = page.locator('#tap-overlay');
    await expect(overlay).toBeVisible();
    await expect(page.locator('#tap-amount-subtitle')).toHaveText('$15.00');

    // 4. Simulate Tap to Pay processing
    const simulateBtn = page.locator('#simulate-tap-btn');
    await expect(simulateBtn).toBeVisible();

    // Intercept network request to terminal intent if necessary, or let it fire in the background
    // Click simulate
    await simulateBtn.click();
    await expect(simulateBtn).toHaveText('Processing...');

    // 5. Success receipt display
    const receiptScreen = page.locator('#receipt-screen');
    await expect(receiptScreen).toBeVisible({ timeout: 5000 });

    await expect(page.locator('#receipt-amount')).toHaveText('$15.00');
    await expect(page.locator('.receipt-text')).toHaveText('Payment Successful');

    // Return to New Sale
    await page.locator('button:has-text("New Sale")').click();
    await expect(page.locator('#amount-display')).toHaveText('$0.00');
    await expect(overlay).not.toBeVisible();
    await expect(receiptScreen).not.toBeVisible();
  });
});
