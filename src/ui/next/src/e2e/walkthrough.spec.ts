import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthroughs', () => {

  test('renders help widget and completes the store setup walkthrough', async ({ page }) => {
    await page.goto('/builder');

    // Open the help widget
    const helpButton = page.locator('button[aria-label="Help"]').first();
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Click on the store setup walkthrough
    const tourButton = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // Assert the first step is shown
    const speechBubble = page.locator('.ohc-walkthrough-bubble').first();
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Learn how to easily set up your store and accept your first payment.')).toBeVisible();

    // Click Next
    await page.getByRole('button', { name: 'Next' }).click();

    // Assert the second step is shown
    await expect(page.getByText('Tell us what you sell so we can create the perfect storefront for you.')).toBeVisible();

    // Click Next
    await page.getByRole('button', { name: 'Next' }).click();

    // Assert the third step is shown
    await expect(page.getByText('Click here and watch our AI build your store from scratch.')).toBeVisible();

    // Click Finish
    await page.getByRole('button', { name: 'Finish' }).click();

    // Assert the bubble is gone
    await expect(speechBubble).not.toBeVisible();
  });

  test('user can exit the walkthrough early by clicking the skip/close button', async ({ page }) => {
    await page.goto('/builder');

    // Open help widget
    const helpButton = page.locator('button[aria-label="Help"]').first();
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    const tourButton = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // Assert the first step is shown
    const speechBubble = page.locator('.ohc-walkthrough-bubble').first();
    await expect(speechBubble).toBeVisible();

    // Highlight overlay should be visible
    const highlightOverlay = page.locator('.ohc-walkthrough-overlay');
    await expect(highlightOverlay).toBeVisible();

    // Click the skip/close button in the walkthrough header
    const closeButton = speechBubble.locator('button', { hasNotText: 'Next' }).first();
    await expect(closeButton).toBeVisible();
    await closeButton.click();

    // Assert both the bubble and the overlay are removed from the DOM
    await expect(speechBubble).not.toBeVisible();
    await expect(highlightOverlay).not.toBeVisible();
  });
});
