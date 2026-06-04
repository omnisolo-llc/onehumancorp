import { test, expect } from '@playwright/test';

test.describe('Upsell Engine Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Reset local storage
    await page.goto('http://localhost:3000');
    await page.evaluate(() => localStorage.clear());

    // Set user as onboarded
    await page.evaluate(() => localStorage.setItem('has_onboarded', 'true'));
  });

  test('Checkout displays upsell recommendations and updates dashboard revenue', async ({ page }) => {
    // 1. Intercept the backend mock call
    await page.route('/api/v1/upsell/recommend', async route => {
      await route.fulfill({
        status: 200,
        json: {
          recommendations: [
            { id: 'upsell_1', name: 'Premium Matches', price: '5.00', image: '🔥', description: 'Perfect pair for your items' }
          ]
        }
      });
    });

    // 2. Navigate to checkout
    await page.goto('http://localhost:3000/checkout');

    // Verify the checkout page has loaded
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // 3. Verify Upsell section is present
    await expect(page.getByText('Frequently Bought Together')).toBeVisible();
    await expect(page.getByText('Premium Matches')).toBeVisible();

    // 4. Click Add
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Upsell added');
      await dialog.accept();
    });

    await page.getByRole('button', { name: 'Add' }).first().click();

    // Wait for Add to process and Upsell to be removed
    await expect(page.getByText('Premium Matches')).not.toBeVisible();

    // 5. Check Dashboard for the updated revenue metric
    await page.goto('http://localhost:3000/dashboard');
    await expect(page.getByText('AI Upsell Revenue')).toBeVisible();
    await expect(page.getByText('$5.00')).toBeVisible();
  });
});
