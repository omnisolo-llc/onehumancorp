import { test, expect } from '@playwright/test';

test.describe('Logs Surface', () => {
  test('exposes recent logs through diagnostics', async ({ page }) => {
    await page.goto('/diagnostics');
    const diagnostics = page.locator('#diagnostics-screen');

    await expect(diagnostics).toBeVisible();
    await expect(diagnostics).toContainText('Recent Logs');
    await expect(diagnostics).toContainText('Recent event log has no error, failure, or exception.');
  });

  test('refreshes and exports log-adjacent diagnostics data', async ({ page }) => {
    await page.goto('/diagnostics');
    await page.getByRole('button', { name: 'Refresh' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics data refreshed');
    await page.getByRole('button', { name: 'Export Report' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics report download ready');
  });
});
