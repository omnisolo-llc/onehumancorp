import { expect, test } from '@playwright/test';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow interacting with dashboard', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    await page.goto('/dashboard');
    await expect(page.locator('text=Activity Feed').first()).toBeVisible({ timeout: 15000 });
  });
});
