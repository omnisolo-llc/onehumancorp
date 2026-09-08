import { test, expect } from '../../../../e2e/fixtures';

test.describe('Release Notes & Changelog', () => {
    test('renders Changelog page and can be accessed from AppShell', async ({ page }) => {
        // Go to dashboard to see AppShell
        await page.goto('/dashboard');

        // Find and click the "What's New" link in the sidebar
        const whatsNewLink = page.locator('a.app-nav-link', { hasText: "What's New" });
        await expect(whatsNewLink).toBeVisible();
        await whatsNewLink.click();

        // Verify we are on the changelog page
        await expect(page).toHaveURL(/\/changelog/);
        await expect(page.locator('h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();

        // Verify release notes content is rendered
        await expect(page.locator('h2', { hasText: 'v1.2.0' })).toBeVisible();

        const brokenImages = await page.locator('img').evaluateAll((images) => images
            .filter((image) => !image.complete || image.naturalWidth === 0)
            .map((image) => image.getAttribute('src')));
        expect(brokenImages).toEqual([]);

        // Check the website link
        const externalLink = page.locator('a', { hasText: 'Read the full technical changelog on our website' });
        await expect(externalLink).toBeVisible();
        await expect(externalLink).toHaveAttribute('href', 'https://cloud.omnisolo.co/changelog');
    });
});
