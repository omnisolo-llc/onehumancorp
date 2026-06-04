import { test, expect } from './fixtures';

test.describe('Logs Surface', () => {
  test('exposes recent logs through diagnostics', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/diagnostics');
    const diagnostics = page.locator('#diagnostics-screen');

    await expect(diagnostics).toBeVisible();
    await expect(diagnostics).toContainText('Recent Logs');
    await expect(diagnostics).toContainText('Recent event log has no error, failure, or exception.');
  });

  test('refreshes and exports log-adjacent diagnostics data', async ({ page }) => {
    test.skip(process.env.CI === 'true' || process.env.CI === '1' || !!process.env.GITHUB_ACTIONS, 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/diagnostics');
    await page.getByRole('button', { name: 'Refresh' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics data refreshed');
    await page.getByRole('button', { name: 'Export Report' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics report download ready');
  });
});
