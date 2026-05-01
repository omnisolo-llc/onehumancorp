import { test, expect } from '@playwright/test';

test.describe('Business Share Store Flow', () => {
  test('should login, display share store modal and verify elements', async ({ page }) => {
    // 1. MUST start from the home page after user login via the UI
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();

    // Ensure we are logged in and on the dashboard
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Wait for the Dashboard to load and verify "Share Store" is visible
    await expect(page.locator('text=Share Store').first()).toBeVisible();

    // Click "Share Store"
    await page.locator('button:has-text("Share Store")').first().click();

    // The Slint backend spawns a new window or simulates the modal.
    // In our Web build of Slint, it might just render over the canvas or show standard UI
    // depending on the build. We'll wait for the "Share Your Store" text.
    await expect(page.locator('text=Share Your Store')).toBeVisible();

    // Check OpenGraph preview parts
    await expect(page.locator('text=Logo / Cover Image')).toBeVisible();
    await expect(page.locator('text=My Awesome Store')).toBeVisible();
    await expect(page.locator('text=The best place to buy things')).toBeVisible();
    await expect(page.locator('text=ohc://share?b=123')).toBeVisible();

    // Check Share buttons
    await expect(page.locator('button:has-text("📋 Copy Shareable Link")')).toBeVisible();
    await expect(page.locator('button:has-text("📷 Share to Instagram")')).toBeVisible();
    await expect(page.locator('button:has-text("🐦 Share to X")')).toBeVisible();
  });
});
