import { test, expect } from '@playwright/test';

test.describe('Social Share Unlock Growth Loop', () => {
  test('User can unlock a discount by clicking the share button on checkout', async ({ page }) => {
    // Navigate to the checkout page with some items
    await page.goto('/checkout?tier=premium&price=99');

    // Verify the Unlock Offer Widget is visible
    const widgetHeading = page.locator('h3', { hasText: 'Want 10% off?' });
    await expect(widgetHeading).toBeVisible();

    const shareButton = page.locator('button', { hasText: 'Share to Unlock' });
    await expect(shareButton).toBeVisible();

    // Setup a listener to close the new tab that the share button opens to avoid hanging
    const [newPage] = await Promise.all([
      page.waitForEvent('popup'),
      shareButton.click()
    ]);

    // Verify it opened the twitter share URL
    expect(newPage.url()).toContain('twitter.com/intent/tweet');
    await newPage.close();

    // Verify the button state changes
    const verifyingText = page.locator('button', { hasText: 'Verifying Share...' });
    await expect(verifyingText).toBeVisible();

    // Wait for the simulated verification delay (2 seconds)
    // Then verify the success state and promo code are shown
    const successHeading = page.locator('h3', { hasText: 'Offer Unlocked!' });
    await expect(successHeading).toBeVisible({ timeout: 5000 });

    const promoCode = page.locator('div', { hasText: 'SHARED10' });
    await expect(promoCode).toBeVisible();
  });
});