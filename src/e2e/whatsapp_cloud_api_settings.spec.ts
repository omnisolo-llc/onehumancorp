import { test, expect } from '@playwright/test';

test.describe('WhatsApp Settings', () => {
  test('saves settings natively', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    await page.goto('/settings/whatsapp');
    await page.locator('input[name="api_key"]').fill('fake-key');
    await page.locator('button:has-text("Save")').click();

    await expect(page.locator('text=Saved').first()).toBeVisible();
  });
});
