import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard E2E', () => {
  // Use mobile viewport for mobile-first requirement
  test.use({ viewport: { width: 375, height: 667 } });

  test('Complete wizard flow from login to dashboard', async ({ page }) => {
    await page.goto('/');

    // 1. Login Screen
    await expect(page.locator('#login-screen')).toBeVisible();
    await page.fill('#login-email', 'test@example.com');
    await page.click('button:has-text("Sign In")');

    // Wait for redirect to dashboard
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });
});
