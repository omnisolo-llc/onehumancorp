import { test, expect } from '@playwright/test';

test.describe('POS Terminal - Offline Tap to Pay Flow', () => {
  test('should queue offline payments and handle payment capture failures gracefully via Agent Tasks', async ({ page }) => {
    // We are simulating Fatima the food cart owner experiencing network drops.

    // 1. Navigate to POS terminal
    await page.goto('/pos/terminal');

    // Wait for the pin screen to be visible
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Wait for the dashboard to load
    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Wait for the product catalog to be populated
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();

    // Simulate going offline
    await page.context().setOffline(true);

    // Verify amber offline indicator
    await expect(page.getByText('Offline - Saving locally').first()).toBeVisible();

    // Add product to cart (vegan celebration cake is $50.00 / 5000 cents which triggers simulate-fail)
    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
    await productButton.click();

    // Verify Bottom Bar and Charge button appears
    const bottomBarChargeBtn = page.locator('button', { hasText: 'Charge' }).last();
    await expect(bottomBarChargeBtn).toBeVisible();

    // Click the Charge button to open Cart Drawer
    await bottomBarChargeBtn.click();

    // Verify Cart Drawer and "Current Order" is visible
    await expect(page.locator('h2:has-text("Current Order")')).toBeVisible();

    // Verify StripeTerminalClient initializes
    await expect(page.locator('h2:has-text("Tap to Pay via Terminal")')).toBeVisible();

    // Process offline payment (this should trigger the 50.00 charge)
    const chargeButton = page.locator('button', { hasText: 'Charge $50.00' }).first();
    await expect(chargeButton).toBeVisible();
    await chargeButton.click();

    // Wait for payment to be queued offline
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible();

    // Go back online
    await page.context().setOffline(false);

    // Wait for backend async processing (sync manager -> pos_sync_worker -> agent_action_requests)
    await page.waitForTimeout(6000);

    // Navigate to Action Center
    await page.goto('/action-center');

    // We expect the CS department task to show up indicating a failed payment
    await expect(page.getByText(/couldn't be processed later/i).first()).toBeVisible({ timeout: 10000 });
  });
});
