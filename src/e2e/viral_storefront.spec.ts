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
    await expect(footer.locator('a')).toHaveAttribute('href', 'ohc://join?ref=embed');
  });

  test('includes Social OG share cards for virality', async ({ page }) => {
    await page.goto('/api/v1/growth/storefront/embed?product_name=TestProduct&price=19.99');

    // Verify OG Meta tags are present in the DOM
    await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', 'TestProduct - Powered by OHC');
    await expect(page.locator('meta[property="og:description"]')).toHaveAttribute('content', 'Get TestProduct for just 19.99. Launch your own business instantly with OHC!');
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', 'https://ohc.app/assets/og-default.png');
    await expect(page.locator('meta[property="og:type"]')).toHaveAttribute('content', 'website');
    await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
  });
});
