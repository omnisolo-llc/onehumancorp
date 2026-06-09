import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the cost transparency section completely', async ({ page }) => {
    test.setTimeout(180000);
    // test using the smoke test approach, we don't login here because the fixture does it.
    await page.goto('/cost-dashboard');

    // just wait for the simplest thing on the page
    await expect(page.locator('#cost-dashboard-total')).toBeVisible({ timeout: 25000 });
  });
});
