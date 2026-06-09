import { test, expect } from '@playwright/test';

test.describe('In-App Help Center and Tooltips', () => {

    test('Help Center FAB is visible and opens sidebar', async ({ page }) => {
        // Mock the fetch API for articles
        await page.route('**/api/help-articles', async route => {
            const json = [{ id: '1', title: 'Accepting payments locally', category: 'Payments', content: 'Use Tap to Pay' }];
            await route.fulfill({ json });
        });

        await page.goto('http://localhost:8080/dashboard.html');

        // Check if FAB exists
        const fab = page.locator('#ohc-help-fab');
        await expect(fab).toBeVisible();

        // Click FAB
        await fab.click();

        // Sidebar should be open
        const sidebar = page.locator('#ohc-help-sidebar');
        await expect(sidebar).toHaveClass(/open/);

        // Search for article
        const searchInput = page.locator('#ohc-help-search');
        await searchInput.fill('payments');

        // Check if filtered
        const articleList = page.locator('.ohc-help-article-link');
        await expect(articleList).toHaveCount(1);
        await expect(articleList.first()).toContainText('Accepting payments locally');
    });

    test('Contextual Tooltip appears on hover', async ({ page }) => {
        await page.route('**/api/help-articles', async route => {
            await route.fulfill({ json: [] });
        });

        await page.goto('http://localhost:8080/dashboard.html');

        // Hover over dashboard button
        const btn = page.locator('#generate-link-btn');
        await btn.hover();

        // Check tooltip
        const tooltip = page.locator('#ohc-global-tooltip');
        await expect(tooltip).toBeVisible();
        await expect(tooltip).toHaveClass(/visible/);
        await expect(tooltip).toContainText('Generates a secure invite link');
    });
});
