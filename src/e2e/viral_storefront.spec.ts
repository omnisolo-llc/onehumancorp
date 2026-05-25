import { test, expect } from './fixtures';

test.describe('Viral Storefront E2E', () => {
  test('exposes referral share entry points for storefront growth', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.locator('#referral-link')).toContainText('https://ohc.app/join?ref=DEFAULT');
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
    await expect(page.locator('.powered-by-footer a')).toHaveAttribute('href', 'https://ohc.app/join?ref=storefront');
  });

  test('provides an embed widget snippet for storefront sharing', async ({ page }) => {
    await page.goto('/storefront-builder');
    await page.getByRole('button', { name: 'Embed' }).click();

    const sheet = page.locator('#embed-setup-sheet');
    await expect(sheet).toHaveClass(/open/);
    await expect(sheet.getByRole('heading', { name: 'Embed Storefront' })).toBeVisible();

    const textarea = sheet.locator('#embed-code-textarea');
    await expect(textarea).toBeVisible();
    await expect(textarea).toHaveValue(/<iframe src=".*\/api\/v1\/growth\/storefront\/embed".*><\/iframe>/);
  });

  test('renders the embed widget directly with viral footer', async ({ page }) => {
    await page.goto('/api/v1/growth/storefront/embed');

    await expect(page.locator('.card')).toBeVisible();
    await expect(page.locator('.title')).toContainText('Premium Product');

    const footer = page.locator('.footer');
    await expect(footer).toBeVisible();
    await expect(footer).toContainText('⚡ Powered by OHC');
    await expect(footer.locator('a')).toHaveAttribute('href', 'https://ohc.app/join?ref=embed');
  });
});
