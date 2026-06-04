import { test, expect } from './fixtures';

test.describe('Logs Surface', () => {
  test('exposes recent logs through diagnostics', async ({ page }) => {
<<<<<<< HEAD
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
    await page.goto('/diagnostics');
    const diagnostics = page.locator('#diagnostics-screen');

    await expect(diagnostics).toBeVisible();
    await expect(diagnostics).toContainText('Recent Logs');
    await expect(diagnostics).toContainText('Recent event log has no error, failure, or exception.');
  });

  test('refreshes and exports log-adjacent diagnostics data', async ({ page }) => {
<<<<<<< HEAD
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
    await page.goto('/diagnostics');
    await page.getByRole('button', { name: 'Refresh' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics data refreshed');
    await page.getByRole('button', { name: 'Export Report' }).click();
    await expect(page.locator('#diagnostics-result')).toContainText('Diagnostics report download ready');
  });
});
