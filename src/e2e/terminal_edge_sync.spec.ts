import { test, expect } from './fixtures';

test('Terminal Edge Sync', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/settings/pos');

    await expect(page.locator('h1')).toHaveText(/Point of Sale/i);

    await page.locator('button:has-text("Sync Terminals")').click();

    await expect(page.locator('.toast')).toHaveText(/Sync Initiated/i);
});
