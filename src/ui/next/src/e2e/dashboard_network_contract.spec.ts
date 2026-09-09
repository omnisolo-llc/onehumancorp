import { test, expect } from '../../../../e2e/fixtures';

test.describe('dashboard network contract', () => {
  test('does not request removed APIs or render an unbacked neighborhood feature', async ({ page }) => {
    const retiredApiRequests: string[] = [];

    page.on('request', (request) => {
      const url = new URL(request.url());
      if (
        url.pathname === '/api/v1/ledger/accounts' ||
        url.pathname === '/api/v1/user/usage' ||
        url.pathname === '/api/v1/mesh/v2/collective'
      ) {
        retiredApiRequests.push(request.url());
      }
    });

    await page.goto('/dashboard', { waitUntil: 'domcontentloaded' });
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await page.waitForTimeout(1500);

    expect(retiredApiRequests).toEqual([]);
    await expect(page.getByRole('heading', { name: 'Neighborhood Pulse' })).toHaveCount(0);
  });
});
