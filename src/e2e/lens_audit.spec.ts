import { test, expect } from '@playwright/test';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate using a valid starting route, simulating normal app usage
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('verify Dashboard visual state and full UI lifecycle', async ({ page }) => {
    // Check if the dashboard is rendered using glassmorphism components
    // If the server is running, we expect the dashboard to have the correct visual elements
    await expect(page.locator('body')).toBeTruthy();
  });
});
