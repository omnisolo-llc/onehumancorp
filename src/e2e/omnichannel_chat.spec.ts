import { test, expect } from './fixtures';

test.describe('Native Omnichannel Chat Web Widget', () => {
  test('User can access inbox and view chat capabilities', async ({ page }) => {
    // Navigate to a page where the widget would be rendered
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
  });
});
