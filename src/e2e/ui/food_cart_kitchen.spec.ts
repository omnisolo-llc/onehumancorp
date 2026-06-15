import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Food Cart Kitchen View', () => {
    test('Fatima toggles sold out and updates sync queue offline', async ({ page }) => {
        // Mock offline
        await page.context().setOffline(true);

        await page.goto('/kitchen.html');

        // Ensure we are offline
        const networkIndicator = page.locator('#network-status-indicator');
        await expect(networkIndicator).toBeVisible();

        // Toggle falafel
        const toggleBtn = page.locator('#sold-out-toggle-falafel');
        await expect(toggleBtn).toBeVisible();
        await toggleBtn.click();

        // Check if queue shows pending mutation
        const queueIndicator = page.locator('#queue-dashboard');
        await expect(queueIndicator).toBeVisible();
        await expect(queueIndicator).toContainText('1 Mutations Pending Sync');

        // Go back online and verify queue clears
        await page.context().setOffline(false);
        // Dispatch an online event just in case since playwright offline toggles might need a page reload or event
        await page.evaluate(() => window.dispatchEvent(new Event('online')));

        await expect(queueIndicator).toBeHidden();
    });
});
