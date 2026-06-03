import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        // Intercept API call to return mock data
        await page.route('/api/help', async route => {
            const json = [
                { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started" }
            ];
            await route.fulfill({ json });
        });

        await page.route('/api/videos', async route => {
            const json = [];
            await route.fulfill({ json });
        });

        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');
        await searchInput.fill('Getting Started');

        // Click on the article
        const articleLink = page.locator('a[href="/help/getting-started"]');
        await expect(articleLink).toBeVisible();
        await articleLink.click();

        // Wait for navigation and API load
        await page.waitForURL('/help/getting-started');

        // Verify article content
        await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();

        // Click back button
        const backButton = page.locator('button', { hasText: 'Back to Help Center' });
        await backButton.click();

        // Verify back navigation
        await page.waitForURL('/help');
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });
});
