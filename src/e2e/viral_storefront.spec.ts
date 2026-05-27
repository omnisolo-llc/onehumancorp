import { test, expect } from './fixtures';

test.describe('Viral Storefront E2E', () => {
  test('exposes referral share entry points for storefront growth', async ({ page }) => {
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.locator('#referral-link')).toContainText('https://ohc.store/join?ref=DEFAULT');
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
    await expect(page.locator('.powered-by-footer a')).toHaveAttribute('href', 'https://ohc.store/join?ref=storefront&utm_source=powered_by');
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

  test('displays Customer Referral block in storefront preview', async ({ page }) => {
    await page.goto('/storefront-builder');
    await expect(page.locator('.builder-block').filter({ hasText: 'Refer a Friend' })).toBeVisible();
    await expect(page.locator('.builder-block').filter({ hasText: 'Get 10% off your next order!' })).toBeVisible();
    await expect(page.locator('.builder-block a[href="https://ohc.store/join?ref=storefront-referral"]')).toBeVisible();
  });

  test('renders the embed widget directly with viral footer', async ({ page }) => {
    await page.goto('/api/v1/growth/storefront/embed');

    await expect(page.locator('.card')).toBeVisible();
    await expect(page.locator('.title')).toContainText('Premium Product');

    const footer = page.locator('.footer');
    await expect(footer).toBeVisible();
    await expect(footer).toContainText('⚡ Powered by OHC');
    await expect(footer.locator('a')).toHaveAttribute('href', 'https://ohc.store/join?ref=embed&utm_source=embed_widget');
  });

  test('generates social share og card with branding', async ({ request }) => {
    const response = await request.get('/api/v1/growth/storefront/og-card?tenant=test&product_name=NovaPremium');
    expect(response.ok()).toBeTruthy();
    expect(response.headers()['content-type']).toContain('image/svg+xml');

    const svg = await response.text();
    expect(svg).toContain('NovaPremium');
    expect(svg).toContain('⚡ Powered by OHC');
  });
});
