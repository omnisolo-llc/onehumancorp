import { test, expect } from '@playwright/test';

test.describe('In-App Video Tutorials', () => {
    test('renders videos tab, fetches videos, and opens/closes the modal player', async ({ page }) => {
        await page.route('**/api/videos', async route => {
            console.log("Mocking /api/videos route");
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify([
                    { id: 1, title: 'How to set up your first store easily', duration: '1:23', url: 'https://example.com/video1.mp4' },
                    { id: 2, title: 'Adding staff to your account', duration: '4:56', url: 'https://example.com/video2.mp4' }
                ])
            });
        });

        // Go to a page where HelpWidget is available (layout.tsx ensures it's on pages like dashboard)
        await page.goto('/dashboard'); // Use the dashboard or any public page where layout applies

        // The help widget should be present.
        const helpButton = page.locator('button[aria-label="Help"]').first();
        await expect(helpButton).toBeVisible();

        // Click the help widget floating button to open the menu
        await helpButton.click();

        // Check that tabs are visible
        const videosTabButton = page.locator('button', { hasText: 'Videos' });
        await expect(videosTabButton).toBeVisible();

        // Click the Videos tab
        await videosTabButton.click();

        // Wait for the videos to be fetched and rendered
        // The API returns 10 videos. We'll wait for at least one to show up.
        // The videos are rendered with titles, like 'How to set up your first store easily'
        const firstVideoTitle = page.locator('p', { hasText: 'How to set up your first store easily' });
        await expect(firstVideoTitle).toBeVisible();

        // Verify some other videos are present
        await expect(page.locator('p', { hasText: 'Adding staff to your account' })).toBeVisible();

        // Click on the first video to open the modal player
        // The video container is a div parent of the title
        const videoContainer = firstVideoTitle.locator('..').locator('..'); // go up to the container
        await videoContainer.click();

        // Verify the modal player opens
        const modalContainer = page.locator('div.fixed.z-\\[100\\]');
        await expect(modalContainer).toBeVisible();

        // Verify the modal has the correct mobile constraints (max-w-[375px])
        await expect(modalContainer.locator('div.max-w-\\[375px\\]')).toBeVisible();

        // Verify the video title is shown in the modal header
        await expect(modalContainer.locator('h3', { hasText: 'How to set up your first store easily' }).nth(1)).toBeVisible();

        // Click the close button
        const closeButton = modalContainer.locator('button[aria-label="Close video"]');
        await closeButton.click();

        // Verify the modal player closes
        await expect(modalContainer).not.toBeVisible();
    });
});
