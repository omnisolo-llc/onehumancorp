import { test, expect } from '../fixtures';

test('Nora views new proposal requests', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/proposals');

    await expect(page.locator('h1')).toHaveText(/Proposals/i);

    await page.locator('button:has-text("New Proposal")').click();
    await page.locator('input[name="clientName"]').fill('Test Client');
    await page.locator('button:has-text("Save Draft")').click();

    await expect(page.locator('.toast')).toHaveText(/Proposal Saved/i);
});
