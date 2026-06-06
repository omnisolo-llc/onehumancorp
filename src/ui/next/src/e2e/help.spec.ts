import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');
        await searchInput.fill('Getting Started');

        // Click on the article
        const articleLink = page.locator('a[href="/help/getting-started-1"]');
        await expect(articleLink).toBeVisible();
        await articleLink.click();

        // Wait for navigation and API load
        await page.waitForURL('/help/getting-started-1');

        // Verify article content
        await expect(page.locator('h1', { hasText: 'Getting Started with Your Store' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();

        await page.goto('/help');
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });

    test('should use backend search for filtering articles', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article that matches My Store
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');

        // Use Promise.all to wait for the request to the search endpoint
        const [response] = await Promise.all([
            page.waitForResponse(response =>
                response.url().includes('/api/help/search') && response.status() === 200
            ),
            searchInput.fill('My Store')
        ]);

        // Wait for UI to update
        const articleLink = page.locator('a[href="/help/my-store"]');
        await expect(articleLink).toBeVisible();
    });
});
