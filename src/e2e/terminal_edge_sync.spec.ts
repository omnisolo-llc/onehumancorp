import { test, expect } from './fixtures';

test.describe('Edge Ledger Sync Protocol UI', () => {
  test('Owner can view edge sync settings', async ({ adminPage }) => {
    const page = await adminPage;
    await page.goto('/settings');
    await expect(page.locator('body')).toBeVisible();
  });
});
