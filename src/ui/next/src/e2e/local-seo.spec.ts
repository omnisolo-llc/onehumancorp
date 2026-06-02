import { test, expect } from '@playwright/test';

test.describe('Automated Local SEO & Google Business Sync Growth Loop', () => {
    test.beforeEach(async ({ page }) => {
        // Clear local storage to reset state
        await page.goto('/');
        await page.evaluate(() => localStorage.clear());
    });

    test('Dashboard Promoter Card - Connect and Approve Flow', async ({ page }) => {
        // Go to dashboard
        await page.evaluate(() => localStorage.setItem('has_onboarded', 'true'));
        await page.goto('/dashboard');

        // 1. Verify card exists and is in disconnected state
        await expect(page.locator('text=Local Visibility')).toBeVisible();
        await expect(page.locator('text=Action Needed')).toBeVisible();

        // 2. Connect Google Maps
        await page.click('button:has-text("Connect Google Maps")');

        // 3. Verify it connects and shows pending reviews
        await expect(page.locator('text=Synced with Google Maps')).toBeVisible();
        await expect(page.locator('text=New Reviews to Approve')).toBeVisible();

        // Verify mock review is visible
        await expect(page.locator('text=Carlos did a great job fixing my plumbing')).toBeVisible();
        await expect(page.locator('text=AI Draft Reply').first()).toBeVisible();

        // 4. Approve a review
        const approveBtn = page.locator('button:has-text("Approve & Reply")').first();
        await approveBtn.click();

        // Ensure the review is removed
        await expect(page.locator('text=Carlos did a great job fixing my plumbing')).not.toBeVisible();
    });

    test('Local Visibility Settings Page Sync', async ({ page }) => {
        // Mock connected state
        await page.evaluate(() => localStorage.setItem('gbp_connected', 'true'));
        await page.goto('/local-visibility');

        await expect(page.locator('h1', { hasText: 'Local Visibility' })).toBeVisible();
        await expect(page.locator('text=Connected and syncing automatically')).toBeVisible();

        // Test manual sync
        await page.click('button:has-text("Sync Now")');

        // Should eventually say Last synced: ...
        await expect(page.locator('text=Never')).not.toBeVisible();
        await expect(page.locator('text=Last synced:')).toBeVisible();
    });
});
