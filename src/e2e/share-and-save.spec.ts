import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Share & Save Viral Loop in Checkout', () => {
  test('User sees Share & Save widget, shares, and gets discount', async ({ page }) => {
    // 1. Navigate to the checkout page directly
    await page.goto('/checkout');
    await page.waitForLoadState('networkidle');

    // 2. Verify the Share & Save widget is visible
    const widget = page.getByTestId('share-and-save-widget');
    await expect(widget).toBeVisible();
    await expect(widget.getByText('Share & Save 10%')).toBeVisible();

    // 3. Verify the initial total ($45.00)
    await expect(page.getByText('$45.00').first()).toBeVisible();

    // 4. Click the "Share on X" button
    // Intercept window.open to prevent actually opening a new tab in the test
    await page.evaluate(() => {
      window.open = () => null;
    });
    await widget.getByTestId('share-x-btn').click();

    // 5. Verify the success state is shown
    const successWidget = page.getByTestId('share-and-save-success');
    await expect(successWidget).toBeVisible();
    await expect(successWidget.getByText('Discount Applied!')).toBeVisible();

    // 6. Verify the total is updated with the 10% discount ($45.00 * 0.9 = $40.50)
    await expect(page.getByText('$40.50').first()).toBeVisible();
  });
});
