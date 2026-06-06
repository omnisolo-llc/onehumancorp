import { test, expect } from '@playwright/test';

test.describe('In-App Video Tutorials', () => {
    test('renders videos, and opens/closes the modal player', async ({ page }) => {
        await page.goto('/help/videos');

        // Wait for videos to load
        // Use Playwright's auto-waiting instead of explicit timeout

        // Verify some videos are present
        const firstVideoTitle = page.locator('p', { hasText: 'How to set up your first store easily' });
        await expect(firstVideoTitle).toBeVisible();
        await expect(page.locator('p', { hasText: 'Adding staff to your account' })).toBeVisible();

        // Click on the first video to open the modal player
        const videoContainer = firstVideoTitle.locator('..').locator('..'); // go up to the container
        await videoContainer.click();

        // Verify the modal player opens
        const modalContainer = page.locator('div.fixed.z-\\[100\\]');
        await expect(modalContainer).toBeVisible();

        // Verify the video title is shown in the modal header
        await expect(modalContainer.locator('h3', { hasText: 'How to set up your first store easily' })).toBeVisible();

        // Click the close button
        const closeButton = modalContainer.locator('button[aria-label="Close video"]');
        await closeButton.click();

        // Verify the modal player closes
        await expect(modalContainer).not.toBeVisible();
    });
});
