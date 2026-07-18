import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Supply Chain', () => {
  test('shows database-backed inventory state', async ({ page }) => {
    await page.goto('/inventory');

    await page.waitForSelector('h1', { timeout: 10000 });
    await expect(page.locator('h1')).toHaveText('Inventory');

    await expect(page.locator('text="Raw Materials"')).toBeVisible();
    await expect(page.locator('text=/api/ui/supply')).toHaveCount(0);
    await expect(page.locator('text="Raw Materials"')).toBeVisible();
    await expect(page.locator('text=/No raw material rows found|Loading inventory|Low Stock|Healthy/').first()).toBeVisible();
  });
});
