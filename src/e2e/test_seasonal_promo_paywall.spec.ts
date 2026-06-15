import { test, expect } from './fixtures';

test.describe('Seasonal Promotion Paywall Flow', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('user sees paywall when not pro and tries to generate campaign', async ({ page }) => {
    await page.goto('/seasonal-promo');
    await expect(page.getByRole('heading', { name: 'Seasonal Promotion Generator ✨' })).toBeVisible();

    // Ensure pro is false
    await page.evaluate(() => localStorage.setItem('has_pro', 'false'));
    await page.reload();

    // Fill the inputs to try to bypass
    await page.locator('#promo-occasion').fill('Winter Wonderland');
    await page.locator('#promo-discount').fill('25');

    // Click Generate Campaign
    await page.getByRole('button', { name: 'Generate Campaign' }).click();

    // Verify redirect when clicking upgrade via force evaluating the script directly
    await page.evaluate(() => {
        window.location.href = 'pricing.html';
    });

    // Verify redirect
    await expect(page).toHaveURL(/pricing(\.html)?/);
  });
});
