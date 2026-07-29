import { test, expect } from '../../../e2e/fixtures';

test.describe('Dashboard Triage Edit', () => {

  test('Shows dashboard edit', async ({ page }) => {
    await page.goto(`/triage`);
    await expect(page.locator('body')).toBeVisible();
  });
});
