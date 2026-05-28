import { test, expect } from '@playwright/test';

test.describe('Offline-First Secure Biometric Identity Mesh', () => {
    test.beforeEach(async ({ page }) => {
        // Mock offline sync functionality / clear storage to ensure isolated tests
        await page.goto('/terminal');
        await page.evaluate(() => localStorage.clear());
        await page.reload();
    });

    test('should render staff lock screen and switch offline gracefully', async ({ page, context }) => {
        // Assert lock screen elements
        await expect(page.locator('text=Select Staff')).toBeVisible();
        await expect(page.locator('text=Fatima')).toBeVisible();
        await expect(page.locator('text=Carlos')).toBeVisible();

        // Ensure offline badge is hidden initially
        await expect(page.locator('#lock-offline-badge')).toBeHidden();

        // Go offline
        await context.setOffline(true);
        await page.evaluate(() => window.dispatchEvent(new Event('offline')));

        // Offline badge should appear
        await expect(page.locator('#lock-offline-badge')).toBeVisible();

        await context.setOffline(false);
    });

    test('should authenticate staff with PIN and instantly load dashboard', async ({ page, context }) => {
        // Tap Fatima
        await page.click('#staff-btn-staff_1');

        // Should show biometric prompt first
        await expect(page.locator('text=Face ID')).toBeVisible();

        // Wait for fallback to PIN
        await page.waitForTimeout(1100);
        await expect(page.locator('text=Enter PIN')).toBeVisible();

        // Enter correct PIN (1234)
        await page.click('#pin-1');
        await page.click('#pin-2');
        await page.click('#pin-3');
        await page.click('#pin-4');

        // Dashboard should appear instantly (Sub-100ms switch)
        await expect(page.locator('text=Point of Sale')).toBeVisible();
        await expect(page.locator('text=Fatima')).toBeVisible();
    });

    test('should reject invalid PIN and allow retry', async ({ page }) => {
        // Tap Carlos
        await page.click('#staff-btn-staff_2');

        // Wait for PIN fallback
        await page.waitForTimeout(1100);

        // Enter incorrect PIN (0000)
        await page.click('#pin-0');
        await page.click('#pin-0');
        await page.click('#pin-0');
        await page.click('#pin-0');

        // Error message
        await expect(page.locator('text=Incorrect PIN')).toBeVisible();

        // Enter correct PIN (5678)
        await page.click('#pin-5');
        await page.click('#pin-6');
        await page.click('#pin-7');
        await page.click('#pin-8');

        // Success
        await expect(page.locator('text=Point of Sale')).toBeVisible();
        await expect(page.locator('text=Carlos')).toBeVisible();
    });

    test('should record actions in offline CRDT log when disconnected', async ({ page, context }) => {
        // Login as Alex
        await page.click('#staff-btn-staff_3');
        await page.waitForTimeout(1100);
        await page.click('#pin-9');
        await page.click('#pin-0');
        await page.click('#pin-1');
        await page.click('#pin-2');

        // Ensure logged in
        await expect(page.locator('text=Point of Sale')).toBeVisible();

        // Go offline
        await context.setOffline(true);
        await page.evaluate(() => window.dispatchEvent(new Event('offline')));

        // Offline badge should be visible on dashboard
        await expect(page.locator('#offline-badge')).toBeVisible();

        // Perform actions
        await page.click('#action-coffee');
        await page.click('#action-pastry');

        // Check local audit log
        const auditLog = page.locator('#audit-log');
        await expect(auditLog).toContainText('sale_coffee');
        await expect(auditLog).toContainText('sale_pastry');
        await expect(auditLog).toContainText('Pending');

        await context.setOffline(false);
    });

    test('should auto-sync offline audit logs when connection restores', async ({ page, context }) => {
        // Start offline
        await context.setOffline(true);
        await page.evaluate(() => window.dispatchEvent(new Event('offline')));

        // Login as Fatima (Offline-First Auth)
        await page.click('#staff-btn-staff_1');
        await page.waitForTimeout(1100);
        await page.click('#pin-1');
        await page.click('#pin-2');
        await page.click('#pin-3');
        await page.click('#pin-4');

        // Take action
        await page.click('#action-coffee');

        // Ensure it's pending sync
        await expect(page.locator('#audit-log')).toContainText('Pending');

        // Restore connection
        await context.setOffline(false);
        await page.evaluate(() => window.dispatchEvent(new Event('online')));

        // Should transition to synced, then disappear (our UI clears after 1s on sync)
        await expect(page.locator('#audit-log')).toContainText('Synced');
        await page.waitForTimeout(1200);
        await expect(page.locator('#audit-log')).not.toContainText('Synced');
        await expect(page.locator('#audit-log')).toContainText('No recent actions');
    });
});
