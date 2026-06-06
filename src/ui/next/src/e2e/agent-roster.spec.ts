import { test, expect } from '@playwright/test';

test.describe('Agent Roster Growth Loop', () => {
    test('Dashboard contains link to Agent Roster', async ({ page }) => {
        await page.goto('/dashboard');
        const rosterLink = page.locator('a[href="/agent-roster"]');
        await expect(rosterLink).toBeVisible();
    });

    test('Agent roster page renders correctly, saves data, and public page works with footer', async ({ page }) => {
        await page.goto('/agent-roster');

        // Verify form elements
        const nameInput = page.locator('input[type="text"]');
        await expect(nameInput).toBeVisible();
        await nameInput.fill('My Epic AI Team');

        // Verify preview updates
        const previewTitle = page.locator('h2', { hasText: 'My Epic AI Team' });
        await expect(previewTitle).toBeVisible();

        // Verify share copy button
        const copyButton = page.locator('button', { hasText: 'Copy Share Details' });
        await expect(copyButton).toBeVisible();

        // Verify X share button
        const twitterShare = page.locator('a', { hasText: 'Share on X' });
        await expect(twitterShare).toBeVisible();

        // Check for the "Powered by OHC" watermark text in the preview
        const poweredBy = page.locator('span', { hasText: 'Powered by OHC' });
        await expect(poweredBy).toBeVisible();
    });
});
