import { test, expect } from '../../../src/e2e/fixtures';

test.describe('Cart Recovery Feature', () => {
  test('should display the PoweredByOHC footer', async ({ page }) => {
    // 2. Act: Navigate to the cart recovery page
    await page.goto('/cart-recovery');

    // 3. Assert: The Powered By OHC footer should be visible
    const poweredBy = page.locator('text=Powered by OHC');
    await expect(poweredBy).toBeVisible();

    // Ensure the link works
    await expect(poweredBy).toHaveAttribute('href', /onboarding/);
  });
});
