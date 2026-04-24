import { test, expect } from '@playwright/test';

test('Dashboard UI displays plain-language labels and important metrics', async ({ page }) => {
  // 1. Login
  await page.goto('/');
  await page.fill('input[name="username"]', 'admin');
  await page.fill('input[name="password"]', 'admin');
  await page.click('button:has-text("Login")');

  // 2. Wait for navigation to Dashboard
  // Assuming after login we are on the dashboard. Let's wait for the overview section.
  await expect(page.locator('text=Overview')).toBeVisible();

  // 3. Verify that the grandmother test plain-language labels exist
  await expect(page.locator('text=Today\\'s Sales')).toBeVisible();
  await expect(page.locator('text=New Orders')).toBeVisible();
  await expect(page.locator('text=Pending Appointments')).toBeVisible();
  await expect(page.locator('text=Active AI Helpers')).toBeVisible();
  await expect(page.locator('text=System Status')).toBeVisible();
  await expect(page.locator('text=Tasks in Progress')).toBeVisible();
});
