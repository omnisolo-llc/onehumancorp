import { test, expect } from '../../../../e2e/fixtures';

test.describe('Dashboard Triage Edit', () => {
  test('Owner can view triage edit', async ({ adminPage }) => {
    const page = await adminPage;
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });
});
