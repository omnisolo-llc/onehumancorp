import { test, expect } from '@playwright/test';

test.describe('Offline-First Mobile POS Sync Engine CUJ', () => {

  test('Persona: Business Owner uses Offline POS and syncs transactions', async ({ page }) => {
    // Mock the sync endpoint
    await page.route('/api/v1/sync/offline', async route => {
      await route.fulfill({ status: 200, json: { success: true, synced_count: 1 } });
    });

    // 1. Owner goes to Dashboard
    await page.goto('/dashboard');

    // Evaluate to navigate to POS Screen (avoids flaky clicks if nav is hidden in mobile view)
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('pos-screen');
    });

    // Verify we are on the POS screen
    await expect(page.getByRole('heading', { name: /Mobile POS/i })).toBeVisible();

    // Make sure we simulate offline mode
    await page.context().setOffline(true);

    // Give it a moment to detect offline mode and update UI
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    const networkStatus = page.locator('#pos-offline-indicator');
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

    // Trigger online event to immediately kick off sync
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });

    await expect(networkStatus).toBeHidden();

    // Queue should drop to 0 eventually
    await expect(page.locator('#pos-queue-count')).toHaveText('0', { timeout: 10000 });
  });

});
