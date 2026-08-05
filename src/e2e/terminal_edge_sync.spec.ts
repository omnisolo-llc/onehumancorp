import { test, expect } from '@playwright/test';

test.describe('Terminal Edge Sync via Real Payload', () => {
  test('syncs terminal status naturally', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/settings/terminal');
    await page.locator('button:has-text("Sync Status")').click();
    await expect(page.locator('text=Synced').first()).toBeVisible({ timeout: 15000 });
  });
});
