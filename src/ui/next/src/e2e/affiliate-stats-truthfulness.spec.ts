import { test, expect, E2E_STARTER_USER } from '../../../../e2e/fixtures';

for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`affiliate statistics at ${viewport.width}px`, () => {
    test.use({ viewport });

    test('an empty tenant displays actual zeros without invented referrals or payouts', async ({ page, loginAs }, testInfo) => {
      // This seeded account has no affiliate records. Read only; do not create
      // links, mutate shared tables, or substitute any application responses.
      await loginAs(page, E2E_STARTER_USER);
      const statistics = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === '/api/v1/growth/affiliate/stats' && response.request().method() === 'GET';
      });
      await page.goto('/dashboard/growth/affiliates', { waitUntil: 'domcontentloaded' });
      const response = await statistics;
      expect(response.status()).toBe(200);
      expect(await response.json()).toEqual({ total_affiliates: 0, total_commission_cents: 0 });

      const affiliates = page.getByText('Total Affiliates', { exact: true }).locator('..');
      const referrals = page.getByText('Active Referrals', { exact: true }).locator('..');
      const commissions = page.getByText('Commission Total (cents)', { exact: true }).locator('..');
      await expect(affiliates).toHaveText(/Total Affiliates\s*0/);
      await expect(referrals).toHaveText(/Active Referrals\s*Unavailable/);
      await expect(commissions).toHaveText(/Commission Total \(cents\)\s*0/);
      await expect(page.getByText('48', { exact: true })).toHaveCount(0);
      await expect(page.getByText('$1240', { exact: true })).toHaveCount(0);
      await expect(page.getByText('Commissions Paid', { exact: true })).toHaveCount(0);
      await expect(page.getByRole('alert').filter({
        hasText: 'Affiliate statistics could not be loaded. No totals are available.',
      })).toHaveCount(0);
      await affiliates.scrollIntoViewIfNeeded();
      await testInfo.attach('affiliate-statistics-empty-tenant', {
        body: await page.screenshot({ fullPage: true }), contentType: 'image/png',
      });
    });
  });
}

test('affiliate statistics stay protected for an anonymous visitor', async ({ anonymousPage }) => {
  const response = await anonymousPage.request.get('/dashboard/growth/affiliates', { maxRedirects: 0 });
  expect(response.status()).toBe(307);
  const location = new URL(response.headers().location, response.url());
  expect(location.pathname).toBe('/login');
  expect(location.searchParams.get('next')).toBe('/dashboard/growth/affiliates');
});
