import { test, expect } from '../../../../e2e/fixtures';

test('Dashboard Triage Edit Flow', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/dashboard');

    await expect(page.locator('h1')).toHaveText(/Dashboard/i);

    await page.locator('button:has-text("Quick Edit")').click();
    await page.locator('textarea[name="triageNote"]').fill('Follow up tomorrow');
    await page.locator('button:has-text("Save Note")').click();

    await expect(page.locator('.toast')).toHaveText(/Note Saved/i);
});
