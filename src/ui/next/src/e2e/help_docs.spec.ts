import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Verify that categories are rendered (Getting Started, My Store, Payments)
        await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible({ timeout: 15000 });
        await expect(page.locator('h2', { hasText: 'My Store' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Payments' })).toBeVisible();

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

        // Wait for hydration to complete by checking for initial content
        await expect(page.locator('h2', { hasText: 'Getting Started' })).toBeVisible({ timeout: 15000 });

        // Search for an article that matches My Store
        const searchInput = page.getByPlaceholder('Search for help articles and videos...');

        const responsePromise = page.waitForResponse(response =>
            response.url().includes("/api/help/search") &&
            (response.status() === 200 || response.status() === 304)
        );
        await searchInput.fill('My Store');
        await responsePromise;

        // Wait for UI to update (non-matching articles should disappear)
        await expect(page.locator('a[href="/help/getting-started-1"]')).not.toBeVisible({ timeout: 10000 });

        const articleLink = page.locator('a[href="/help/add-products"]');
        await expect(articleLink).toBeVisible({ timeout: 10000 });
    });
});
