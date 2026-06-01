import { test, expect } from '@playwright/test';

test.describe('Offline-First Mobile POS Sync Engine CUJ', () => {

  test('Persona: Business Owner uses Offline POS and syncs transactions', async ({ page }) => {
    // 1. Owner goes to Dashboard
    await page.goto('/dashboard');

    // We expect the network status indicator to be there
    const networkStatus = page.locator('#pos-offline-indicator');

    // Navigate to POS Screen using the main nav
    await page.locator('#nav-pos').click();

    // Verify we are on the POS screen
    await expect(page.getByRole('heading', { name: /Mobile POS/i })).toBeVisible();

    // Make sure we simulate offline mode
    await page.context().setOffline(true);

    // Give it a moment to detect offline mode and update UI
    await page.waitForTimeout(200);
    await expect(networkStatus).toBeVisible();

    // Fill in amount
    await page.locator('#pos-amount').fill('42.50');

    // Click Tap-to-Pay
    await page.locator('#pos-tap-to-pay-btn').click();

    // Should show success
    await expect(page.getByText('Transaction Saved Offline')).toBeVisible();

    // Check that queue has 1 pending item
    await expect(page.locator('#pos-queue-count')).toHaveText('1');

    // Now go online
    await page.context().setOffline(false);

    // The POS sync engine should run automatically and clear local queue
    await page.waitForTimeout(200);
    await expect(networkStatus).toBeHidden();

    // Queue should drop to 0 eventually
    await expect(page.locator('#pos-queue-count')).toHaveText('0', { timeout: 10000 });
  });

});
