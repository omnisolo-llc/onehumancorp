import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('Business Snapshot').first()).toBeVisible();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' }).first()).toBeVisible();
    await expect(page.getByText(/The Ambassador/)).toBeVisible();

    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' }).first()).toBeVisible();

    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();

    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible();

    await page.goto('/storefront-builder');

    const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
    expect(ogCard.headers()['content-type']).toContain('image/png');
  });
}
