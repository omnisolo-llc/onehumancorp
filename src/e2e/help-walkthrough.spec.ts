import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Help Walkthrough', () => {
  test('should open walkthrough overlay when triggered', async ({ page }) => {
    await adminPage(page);
    await page.goto('/dashboard.html');

    // Wait for the start tour button to be available
    const tourBtn = page.locator('#dashboard-walkthrough-btn');
    await tourBtn.waitFor({ state: 'visible' });

    // Click it to start the walkthrough
    await tourBtn.click();

    // Assert the overlay is visible
    const overlay = page.locator('#walkthrough-overlay');
    await expect(overlay).toBeVisible();

    // Assert the bubble is visible
    const bubble = page.locator('#walkthrough-bubble');
    await expect(bubble).toBeVisible();

    // Close the walkthrough
    await page.locator('#wt-close').click();
    await expect(overlay).not.toBeVisible();
  });

  test('should open and play video tutorial', async ({ page }) => {
    await adminPage(page);
    await page.goto('/help');

    // Wait for the video tutorial cards to be visible
    const videoCard = page.getByText('How to set up your first store easily');
    await videoCard.waitFor({ state: 'visible' });

    // Click the video card to open the player modal
    await videoCard.click();

    // Assert that the video player modal is visible
    const videoPlayer = page.locator('video');
    await expect(videoPlayer).toBeVisible();

    // Assert that the close button is visible and click it to close the modal
    const closeBtn = page.getByLabel('Close video');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();

    // Assert that the modal is no longer visible
    await expect(closeBtn).not.toBeVisible();
  });
});
