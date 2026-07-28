import { test, expect } from './fixtures';

test('Unified Inbox Instagram Flow', async ({ page, adminUser, loginAs }) => {
    await loginAs(adminUser);

    await page.goto('/inbox');

    await expect(page.locator('h1')).toHaveText(/Unified Inbox/i);

    await page.locator('button:has-text("Compose")').click();
    await page.locator('input[name="recipient"]').fill('test_customer');
    await page.locator('textarea[name="message"]').fill('Hello there!');
    await page.locator('button:has-text("Send")').click();

    await expect(page.locator('.toast')).toHaveText(/Message Sent/i);
});
