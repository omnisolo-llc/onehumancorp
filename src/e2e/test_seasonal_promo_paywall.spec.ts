import { test, expect } from './fixtures';

test.describe('Seasonal Promotion Paywall Flow', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('user sees paywall when not pro and tries to generate campaign', async ({ page }) => {
    // Note: hitting Next.js route without .html
    await page.goto('/seasonal-promo');
    await expect(page.getByRole('heading', { name: 'Seasonal Promotion Generator ✨' })).toBeVisible();

    // Ensure pro is false
    await page.evaluate(() => localStorage.setItem('has_pro', 'false'));
    await page.reload();

    // Fill the inputs to try to bypass
    await page.locator('#promo-occasion').fill('Winter Wonderland');
    await page.locator('#promo-discount').fill('25');

    // Click generate which triggers the soft paywall modal in Next.js
    await page.getByRole('button', { name: 'Generate Campaign' }).click();

    // Check paywall modal content
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    // Click upgrade
    const upgradeButton = page.getByRole('button', { name: 'Upgrade to Pro' });
    await upgradeButton.click();

    // Verify redirect
    await expect(page).toHaveURL(/pricing/);
  });
});
