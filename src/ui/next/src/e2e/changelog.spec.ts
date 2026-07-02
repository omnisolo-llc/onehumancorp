import { test, expect } from '@playwright/test';

test.describe('Release Notes & Changelog', () => {
    test('renders Changelog page with screenshots and can be accessed from AppShell', async ({ page }) => {
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

        // Wait for images to load, which proves the API correctly fetched the screenshot URL
        // In our manual test setup, we added a placeholder image to the top version
        const screenshot = page.locator('img[alt*="Screenshot"]').first();
        await expect(screenshot).toBeVisible();

        // Check the website link
        const externalLink = page.locator('a', { hasText: 'Read the full technical changelog on our website' });
        await expect(externalLink).toBeVisible();
        await expect(externalLink).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
    });
});
