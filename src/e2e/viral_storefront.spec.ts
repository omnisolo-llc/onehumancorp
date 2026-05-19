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

  test('displays Powered by OHC virality loop on published storefront builder preview', async ({ page }) => {
    await page.goto('/storefront-builder');
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveAttribute('href', '/referrals?ref=storefront');
  });
});
