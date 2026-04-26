import { test, expect } from '@playwright/test';

test('Navigate to Help Center', async ({ page }) => {
  await page.goto('/login');
  await page.fill('input[type="email"]', 'maya@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign in")');
  await page.waitForURL('**/dashboard');

  await page.click('text="Help Center"');
  await page.waitForURL('**/help');

  await expect(page.locator('text="Help Center"').first()).toBeVisible();
  await expect(page.locator('text="Getting Started"').first()).toBeVisible();
});
