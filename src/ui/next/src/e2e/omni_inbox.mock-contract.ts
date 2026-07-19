import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omnichannel Inbox UI', () => {
  test('Owner sees sender id and known customer in inbox', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/inbox');

    // Check if the mock message 'e2e-inbox-msg-1' is rendered with sender 'maya_bakes'
    const msg = page.locator('.app-list-item', { hasText: 'Do you have vegan options' }).first();
    await expect(msg).toBeVisible({ timeout: 10000 });
    await msg.click();

    await expect(page.getByText('maya_bakes')).toBeVisible();
    await expect(page.getByText('Known Customer')).toBeVisible();
  });
});
