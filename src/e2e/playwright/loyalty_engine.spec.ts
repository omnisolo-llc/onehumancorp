import { test, expect } from '../fixtures';

test('Owner creates a new loyalty tier', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/settings/loyalty');

    await expect(page.locator('h1')).toHaveText(/Loyalty Program/i);

    await page.locator('button:has-text("Create Tier")').click();
    await page.locator('input[name="tierName"]').fill('Gold Member');
    await page.locator('button:has-text("Save Tier")').click();

    await expect(page.locator('.toast')).toHaveText(/Tier Created/i);
});
