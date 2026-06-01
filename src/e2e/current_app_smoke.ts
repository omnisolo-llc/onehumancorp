import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    const baseUrl = process.env.BASE_URL || 'http://localhost:3000';
    await page.goto(baseUrl + '/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('Business Snapshot').first()).toBeVisible();

    await page.goto(baseUrl + '/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
    await expect(page.getByText('The Ambassador').first()).toBeVisible();

    await page.goto(baseUrl + '/website-builder');
    await expect(page.getByText('Your business, live in minutes.').first()).toBeVisible();

    await page.goto(baseUrl + '/integrations');
    await expect(page.getByText('Tool Integrations').first()).toBeVisible();

    await page.goto(baseUrl + '/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible();

    await page.goto(baseUrl + '/storefront-builder');
    await expect(page.getByText('Welcome to OHC Smart Builder').first()).toBeVisible();

    const ogCard = await request.get(baseUrl + '/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
    expect(ogCard.headers()['content-type']).toContain('image/png');
  });
}
