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
});
