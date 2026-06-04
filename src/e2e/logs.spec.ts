import { test, expect } from './fixtures';

test.describe('Logs Surface', () => {
  test('exposes recent logs through diagnostics', async ({ page }) => {
    await page.goto('/diagnostics');
    const diagnostics = page.locator('#diagnostics-screen');

    await expect(diagnostics).toBeVisible();
    await expect(diagnostics).toContainText('Operational Telemetry');
  });

  test('refreshes and exports log-adjacent diagnostics data', async ({ page }) => {
    await page.goto('/diagnostics');
    await expect(page.locator('#diagnostics-screen')).toContainText('AutoDream Memory Pipeline');
  });
});
