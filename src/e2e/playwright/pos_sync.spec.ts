import { test, expect } from '@playwright/test';

test.describe('POS Offline Transaction Sync', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/pos/terminal');
    });

    test('should allow processing offline transaction and see syncing state', async ({ page }) => {
        // Assume offline mode
        await page.context().setOffline(true);

        // Verify page loads correctly and is offline
        await expect(page.locator('h2')).toHaveText('Tap to Pay via Terminal');
        await expect(page.locator('text=Status: Terminal is Offline')).toBeVisible();

        // Let's pretend we have a reader connected via mock state for the test, or wait for the interface.
        // Wait for the Discover Readers button and click
        const discoverBtn = page.locator('button', { hasText: 'Discover Readers' });
        await expect(discoverBtn).toBeVisible();
        await discoverBtn.click();

        // Connect a reader
        const connectBtn = page.locator('button', { hasText: 'Connect' }).first();
        await expect(connectBtn).toBeVisible();
        await connectBtn.click();

        // Process payment
        const chargeBtn = page.locator('button', { hasText: 'Charge' });
        await expect(chargeBtn).toBeVisible();
        await chargeBtn.click();

        // It should show the processing offline status
        await expect(page.locator('text=Status: Processing offline payment...')).toBeVisible();

        // Wait for the simulated transaction to complete and the "syncing" UI to show
        const syncingOverlay = page.locator('#syncing-overlay');
        await expect(syncingOverlay).toBeVisible();

        // Eventually it transitions to Synced state
        await expect(page.locator('text=Status: Payment synced successfully.')).toBeVisible();
        await expect(syncingOverlay).not.toBeVisible();
    });
});
