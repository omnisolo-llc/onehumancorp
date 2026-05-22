import { test, expect } from './fixtures';

test.describe('Viral Storefront E2E', () => {
  test('exposes referral share entry points for storefront growth', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=DEFAULT');
    await expect(page.getByRole('button', { name: /Share to Instagram/ })).toBeVisible();
  });

  test('opens storefront publish domain workflow', async ({ page }) => {
    await page.goto('/storefront-builder');
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await expect(page.getByRole('heading', { name: 'Publish Site' })).toBeVisible();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await expect(page.getByPlaceholder('mybusiness')).toBeVisible();
  });

  test('displays Powered by OHC footer in storefront preview', async ({ page }) => {
    await page.goto('/storefront-builder');
    await expect(page.locator('.powered-by-footer')).toBeVisible();
    await expect(page.locator('.powered-by-footer')).toContainText('⚡ Powered by OHC');
    await expect(page.locator('.powered-by-footer a')).toHaveAttribute('href', 'ohc://join?ref=storefront');
  });
});
