import { test, expect } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage', () => {
  test('Dashboard loads', async ({ page }) => {
    await page.goto(`/`);
  });
});
