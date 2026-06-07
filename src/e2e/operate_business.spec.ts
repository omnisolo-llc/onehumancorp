import { test, expect } from './fixtures';

test.describe('Maya (Home Baker) CUJ', () => {
  test('Maya operates her custom cake business', async ({ page }) => {
    // Navigate from Home Page to Dashboard via UI instead of direct URL
    await page.goto('/login');
    await page.getByPlaceholder('you@email.com').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Triage messages
    await page.getByRole('link', { name: 'Inbox' }).click();
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // Manage custom-order deposits
    await page.getByRole('link', { name: 'Orders' }).click();
    await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
  });
});
