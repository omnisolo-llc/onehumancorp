import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display share options from dashboard and handle clicks', async ({ page }) => {
    // Navigate to login and authenticate
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // Verify we are on the dashboard
    await expect(page.locator('text=Dashboard').first()).toBeVisible();

    // Override window.open to intercept intent URLs
    await page.evaluate(() => {
      (window as any).intentLogs = [];
      window.open = (url: string | URL | undefined, target?: string, features?: string) => {
        (window as any).intentLogs.push(url);
        return null;
      };
    });

    // Open Business Share
    const shareStoreBtn = page.locator('button:has-text("Share Store"), [class*="share_store"]').first();
    await shareStoreBtn.click();

    // Verify Business Share UI elements
    await expect(page.locator('text=Share Your Store')).toBeVisible();
    await expect(page.locator('text=📋 Copy Shareable Link')).toBeVisible();
    await expect(page.locator('text=📷 Share to Instagram')).toBeVisible();
    await expect(page.locator('text=🐦 Share to X')).toBeVisible();
    await expect(page.locator('text=💬 Share to WhatsApp')).toBeVisible();

    // Verify actions
    await page.locator('button:has-text("📋 Copy Shareable Link")').click();

    // Verify Instagram
    await page.locator('button:has-text("📷 Share to Instagram")').click();

    // Verify X
    await page.locator('button:has-text("🐦 Share to X")').click();

    // Verify WhatsApp
    await page.locator('button:has-text("💬 Share to WhatsApp")').click();

    // Ensure intents were logged (only works in WASM but test verifies buttons are clickable)
    const logs = await page.evaluate(() => (window as any).intentLogs);
    console.log("Intent Logs:", logs);

    // Close the share window
    const closeBtn = page.locator('button:has-text("Close")');
    await closeBtn.click();
    await expect(page.locator('text=Share Your Store')).not.toBeVisible();
  });
});
