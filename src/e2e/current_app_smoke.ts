import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('Business Snapshot').first()).toBeVisible();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'The Ambassador' }).first()).toBeVisible();

    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen').first()).toBeVisible();

    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();

    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible();

    const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
    expect(ogCard.headers()['content-type']).toContain('image/png');
  });
}
