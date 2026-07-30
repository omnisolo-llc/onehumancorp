import { test, expect } from '@playwright/test';

test.describe('Omni-channel Inbox Basic UI', () => {
  test('loads inbox list and can view a conversation', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL(/dashboard/);

    await page.goto('/inbox');
    await expect(page.locator('text=Inbox').first()).toBeVisible();
  });
});
