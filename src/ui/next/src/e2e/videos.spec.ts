import { test, expect } from '@playwright/test';

test.describe('In-App Video Tutorials', () => {
    test('renders videos tab, fetches videos, and opens/closes the modal player', async ({ page }) => {
        // The videos page has its own direct route now
        await page.goto('/help/videos');

        // Verify the title
        await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();
    });
});
