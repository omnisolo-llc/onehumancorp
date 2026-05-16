import { test, expect } from '@playwright/test';

test('Dashboard should load and display core elements', async ({ page }) => {
  await page.goto('/login');
  await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
  await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
  await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

  await page.waitForURL('**/*');

  await expect(page.locator('text=Dashboard')).toBeVisible();
  await expect(page.locator('text=You have 2 new orders')).toBeVisible();

  // Verify Agent Actions feed
  await expect(page.locator('text=Your Support Agent replied to 3 customers')).toBeVisible();
});
