import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
    test('renders tooltip on hover', async ({ page }) => {
        await page.goto('/dashboard');

        // Find the HelpWidget button which has a tooltip
        const helpButton = page.locator('button[aria-label="Help"]');
        await expect(helpButton).toBeVisible();

        // Hover over it
        await helpButton.hover();

        // The tooltip should appear with text
        const tooltip = page.locator('div.fixed.z-\\[100\\]', { hasText: 'Need help? Click here to access our Help Center' }).first();
        await expect(tooltip).toBeVisible();

        // Open the help menu
        await helpButton.click();

        // Go to Whats New tab
        const whatsNewTabButton = page.locator('button', { hasText: 'New' });
        await expect(whatsNewTabButton).toBeVisible();
        await whatsNewTabButton.click();

        // Hover the 'Read full release notes' link
        const releaseNotesLink = page.locator('a[href="/changelog"]');
        await expect(releaseNotesLink).toBeVisible();
        await releaseNotesLink.hover();

        // Verify the tooltip for changelog nav
        const changelogTooltip = page.locator('div.fixed.z-\\[100\\]', { hasText: 'See what\'s new in the latest OneHumanCorp updates' }).first();
        await expect(changelogTooltip).toBeVisible();
    });
});
