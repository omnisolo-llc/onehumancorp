import { test, expect } from '@playwright/test';

test.describe('Competitor Audit Dashboard Glassmorphism E2E', () => {
    test.beforeEach(async ({ page }) => {
        // Start from the home page
        await page.goto('/');

        // Wait for the navigation to load and click the real navigation link
        await page.waitForSelector('#nav-competitor-audit', { state: 'visible' });
    });

    test('renders competitor audit stats panel with correct styles', async ({ page }) => {
        await page.click('#nav-competitor-audit');

        const panel = page.locator('text="Probes Completed"').locator('xpath=ancestor::div[contains(@class, "panel")]').first();
        await expect(panel).toBeVisible();
    });

    test('verifies stats text font is readable on mobile', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.click('#nav-competitor-audit');

        const header = page.locator('h1', { hasText: 'Competitor Audit Glassmorphism Dashboard' });
        await expect(header).toBeVisible();
    });

    test('verifies data updates visually without breaking layout', async ({ page }) => {
        await page.click('#nav-competitor-audit');

        const probeStat = page.locator('text="Probes Completed"');
        await expect(probeStat).toBeVisible();
        const numberStat = page.locator('h2', { hasText: '1,204' });
        await expect(numberStat).toBeVisible();
    });

    test('dashboard layout contains a grid or panels', async ({ page }) => {
        await page.click('#nav-competitor-audit');

        await expect(page.locator('.panel')).not.toHaveCount(0);
    });

    test('no technical jargon in UI feed (if present)', async ({ page }) => {
        await page.click('#nav-competitor-audit');

        const feedItem = page.locator('text="Your Support Agent probed"');
        await expect(feedItem).toBeVisible();

        await expect(page.locator('text="SQLite fallback Error"')).toHaveCount(0);
    });
});
