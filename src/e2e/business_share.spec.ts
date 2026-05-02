import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display share options and copy link functionality', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard (assuming login goes to dashboard)
    await page.waitForURL('**/*');

    // Wait for the "Share Store" button on the dashboard and click it
    const shareStoreBtn = page.locator('button:has-text("Share Store")');
    await expect(shareStoreBtn).toBeVisible();
    await shareStoreBtn.click();

    // 3. Verify the Share Store component pops up
    await expect(page.locator('text=Share Your Store')).toBeVisible();

    // Verify OpenGraph Preview elements
    await expect(page.locator('text=Logo / Cover Image')).toBeVisible();
    await expect(page.locator('text=My Awesome Store')).toBeVisible();
    await expect(page.locator('text=The best place to buy things')).toBeVisible();
    await expect(page.locator('text=ohc://share?b=123')).toBeVisible();

    // Verify copying / sharing options are present
    const copyBtn = page.locator('button:has-text("📋 Copy Shareable Link")');
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    await expect(page.locator('button:has-text("📷 Share to Instagram")')).toBeVisible();
    await expect(page.locator('button:has-text("🐦 Share to X")')).toBeVisible();

    // Close the Share Store dialog
    const closeBtn = page.locator('button:has-text("Close")');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();
  });
});
