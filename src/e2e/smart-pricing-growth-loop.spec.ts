import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the smart pricing page
    await page.goto('/smart-pricing');
  });

  test('should display soft paywall for non-pro users on analyze click', async ({ page }) => {
    // Ensure local storage has_pro is not set or false (default)
    await page.evaluate(() => localStorage.removeItem('has_pro'));

    // Fill in product details
    await page.fill('#product-name', 'Artisan Coffee Beans');
    await page.fill('#current-price', '15.00');

    // Click analyze button
    await page.click('button:has-text("Analyze Market & Optimize Pricing")');

    // Wait for paywall modal to appear
    const modal = page.locator('text=Upgrade to Pro');
    await expect(modal).toBeVisible();

    // Claim trial extension to bypass
    await page.click('button:has-text("Share on X to get 7 Days Free")');

    // Paywall should disappear
    await expect(modal).toBeHidden();
  });

  test('should show recommendation for pro users', async ({ page }) => {
    // Set user as pro
    await page.evaluate(() => localStorage.setItem('has_pro', 'true'));

    // Reload page to apply storage changes
    await page.reload();

    // Fill in product details
    await page.fill('#product-name', 'Artisan Coffee Beans');
    await page.fill('#current-price', '15.00');

    // Start API request interception
    await page.route('/api/v1/growth/pricing/optimize', async (route) => {
      const json = {
        recommended_price: '18.00',
        explanation: 'AI analysis suggests a 20% increase for maximum profit.'
      };
      await route.fulfill({ json });
    });

    // Click analyze button
    await page.click('button:has-text("Analyze Market & Optimize Pricing")');

    // Check for results
    await expect(page.locator('text=Smart Pricing Recommendation')).toBeVisible();
    await expect(page.locator('text=18.00')).toBeVisible();
    await expect(page.locator('text=AI analysis suggests a 20% increase')).toBeVisible();
  });
});
