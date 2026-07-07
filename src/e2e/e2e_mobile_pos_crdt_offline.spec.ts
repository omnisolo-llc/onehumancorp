import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Mobile POS Offline-First & CRDT State Engine', () => {
    test('Fatima uses POS offline, takes orders, and syncs upon reconnection', async ({ page }) => {
        // Assume context of Fatima the Food Cart Operator.
        await adminPage(page, async (page) => {
            // Navigate to POS
            await page.goto('/pos.html');
            await expect(page.locator('h1', { hasText: 'Quick Charge' })).toBeVisible();

            // Simulate Offline Mode
            await page.evaluate(() => {
                // Manually trigger offline indicator test event
                window.__TAURI__.event.emit('sync_status', { online: false, pending: 0, syncing: false });
            });

            // Expect indicator pill to show "Working Offline"
            const offlineIndicator = page.locator('#offline-indicator');
            await expect(offlineIndicator).toBeVisible();
            await expect(page.locator('#offline-text')).toHaveText('Working Offline');

            // Quick Charge $50 Cash
            await page.locator('text="Quick Charge"').first().click();
            await page.locator('text="Quick Charge $50"').click();

            // Should activate charge button
            await expect(page.locator('#charge-btn')).toBeEnabled();
            await page.locator('#charge-btn').click();

            // Wait for tap overlay and simulate record cash sale
            await expect(page.locator('#tap-overlay')).toBeVisible();
            await page.locator('#record-cash-sale-btn').click();

            // Expect truthful UI feedback "Offline Charge Saved"
            await expect(page.locator('.receipt-text')).toHaveText('Offline Charge Saved.');

            // Simulate pending count update from Tauri
            await page.evaluate(() => {
                window.__TAURI__.event.emit('sync_status', { online: false, pending: 1, syncing: false });
            });
            await expect(page.locator('#offline-text')).toHaveText('Pending Sync');
            await expect(page.locator('#pending-count')).toHaveText('1');

            // Reconnect
            await page.evaluate(() => {
                window.__TAURI__.event.emit('sync_status', { online: true, pending: 1, syncing: true });
            });
            await expect(page.locator('#offline-text')).toHaveText('Syncing...');

            // Sync finish
            await page.evaluate(() => {
                window.__TAURI__.event.emit('sync_status', { online: true, pending: 0, syncing: false });
            });
            await expect(offlineIndicator).toBeHidden();
        });
    });
});
