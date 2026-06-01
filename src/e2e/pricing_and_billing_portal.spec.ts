import { test, expect } from './fixtures';

test.describe('Pricing and Billing Portal', () => {
  test('Complete journey: navigate to pricing, select plan, checkout, view cost dashboard', async ({ page }) => {
    // 1. Navigation from Dashboard to Pricing page
    await page.getByRole('link', { name: /Upgrade/i }).first().click();
    await expect(page).toHaveURL(/.*\/pricing/);

    // Verify pricing header
    await expect(page.locator('h1').filter({ hasText: 'Pricing Plans' })).toBeVisible();

    // 2. Select Starter tier
    await page.getByRole('button', { name: /Upgrade to Starter/i }).click();

    // Wait for navigation to checkout
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
    await expect(page.locator('h1').filter({ hasText: 'Checkout - Starter Plan' })).toBeVisible();

    // 3. Complete Checkout (Tap to Pay)
    page.on('dialog', dialog => {
      if (dialog.type() === 'prompt') {
        dialog.accept('29.00');
      } else if (dialog.type() === 'alert') {
        dialog.accept();
      }
    });

    await page.getByRole('button', { name: /Tap to Pay/i }).click();

    // Verify redirection back to dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);

    // 4. View Plan details
    await page.getByRole('link', { name: /My Plan/i }).first().click();
    await expect(page).toHaveURL(/.*\/plan/);
    await expect(page.locator('h1').filter({ hasText: 'My Plan' })).toBeVisible();

    // 5. Navigate to Cost Dashboard
    await page.getByRole('button', { name: /View Cost Details/i }).click();
    await expect(page.locator('h1').filter({ hasText: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.locator('h2').filter({ hasText: 'Cost Transparency' })).toBeVisible();
  });
});
