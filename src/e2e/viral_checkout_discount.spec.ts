import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_checkout_discount');

test.describe('Viral Checkout Growth Loop', () => {
  test('should allow user to claim a 10% discount by sharing on X', async ({ page }) => {
    // Navigate to checkout
    await page.goto('/checkout?discount=20');
    await page.waitForLoadState('networkidle');

    // 1. Verify the widget is visible
    const widgetHeading = page.getByRole('heading', { name: /Want 10% off your order\?/i });
    await expect(widgetHeading).toBeVisible();

    // 2. Verify the share button is present
    const shareButton = page.getByRole('button', { name: /Share on X to get 10% off/i });
    await expect(shareButton).toBeVisible();
    await expect(shareButton).toBeEnabled();

    // 3. Mock window.open to prevent opening a new tab
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // 4. Click the share button to trigger the bypass API call
    await shareButton.click();

    // 5. Verify the success state
    const successHeading = page.getByRole('heading', { name: '10% Discount Claimed!' });
    await expect(successHeading).toBeVisible({ timeout: 10000 });
    await expect(page.getByText(/Thanks for sharing. Your discount has been applied./i)).toBeVisible();

    // 6. Verify the total has been updated
    const totalAfterDiscount = page.getByText(/Total after Discount/i);
    await expect(totalAfterDiscount).toBeVisible();

    // Check if the price was updated (45.00 * 0.9 = 40.50)
    const priceText = page.getByText(/\$40\.50/i);
    await expect(priceText).toBeVisible();
  });
});
