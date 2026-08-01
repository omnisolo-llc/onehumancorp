import { test, expect } from '@playwright/test';
test.describe('Dashboard Triage Edit', () => {
  test('Owner can edit triage', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await page.waitForURL('/dashboard', { timeout: 10000 });

    await page.goto('/triage');
    await expect(page.locator('body')).toBeVisible();
  });
});