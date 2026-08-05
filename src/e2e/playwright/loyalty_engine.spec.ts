import { test, expect } from '@playwright/test';

test.describe('Loyalty Engine Logic', () => {
  test('creates a loyalty entry natively', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/loyalty');
    await page.locator('button:has-text("Activate")').click();
    await expect(page.locator('text=Active').first()).toBeVisible();
  });
});
