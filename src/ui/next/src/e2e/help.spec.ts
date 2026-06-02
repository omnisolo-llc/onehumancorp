import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
    test('renders help center and navigates to an article', async ({ page }) => {
        await page.goto('/help');

        // Verify Help Center title
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

        // Search for an article
        const searchInput = page.getByPlaceholder('Search for help articles...');
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

        // Navigate back
        await page.goto('/help');

        // Verify back navigation
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });

    test('navigates to a non-existent help article and shows 404', async ({ page }) => {
        // E2E Test 1: 404 flow
        await page.goto('/help/does-not-exist');

        // Verify the 404 page is rendered correctly
        await expect(page.locator('h1', { hasText: 'Article Not Found' })).toBeVisible();
        await expect(page.locator('p', { hasText: "We couldn't find the article you're looking for." })).toBeVisible();

        // Test back button
        const backButton = page.locator('button').filter({ hasText: 'Back to Help Center' });
        await Promise.all([
          page.waitForURL('**/help'),
          backButton.click()
        ]);
        await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();
    });

    test('searching for a term with no results shows appropriate message', async ({ page }) => {
        // E2E Test 2: Empty search flow
        await page.goto('/help');

        const searchInput = page.getByPlaceholder('Search for help articles...');
        await searchInput.fill('xyznonexistentterm');

        // Verify empty state message
        await expect(page.locator('p', { hasText: 'No articles found matching "xyznonexistentterm"' })).toBeVisible();
    });
});
