import { test, expect } from '@playwright/test';

test.describe('Autonomous Supply Chain', () => {
  test('shows database-backed inventory state', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/inventory');
=======
    await page.goto('http://localhost:3000/inventory');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))

    await page.waitForSelector('h1', { timeout: 10000 });
    await expect(page.locator('h1')).toHaveText('Inventory');

    await expect(page.locator('text="Raw Materials"')).toBeVisible();
    await expect(page.locator('text="Loaded from `/api/ui/supply`."')).toBeVisible();
    await expect(page.locator('text=/No raw material rows found|Loading inventory|Low Stock|Healthy/')).toBeVisible();
  });
});
