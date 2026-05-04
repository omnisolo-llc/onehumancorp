import { test, expect } from '@playwright/test';

test.describe('Business Share & Embed', () => {
  test('should display share options from dashboard and via Grow Business', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard
    await page.waitForURL('**/*');

    // Wait for the "Share" floating button (tooltip_id: "share_store" usually, but text is "Share") or "Share Store" depending on the screen size
    const shareStoreBtn = page.locator('button:has-text("Share")').first();
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
    await expect(page.locator('button:has-text("💬 Share to WhatsApp")')).toBeVisible();

    // Close the Share Store dialog
    const closeBtn = page.locator('button:has-text("Close")');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();

    // 4. Test the "Share Business" action from Grow Business
    const growBusinessBtn = page.locator('button:has-text("Grow Business")');
    await expect(growBusinessBtn).toBeVisible();
    await growBusinessBtn.click();

    await expect(page.locator('text=Select a next step to grow your business')).toBeVisible();

    const shareBusinessStrategyBtn = page.locator('button:has-text("Share Business")');
    await expect(shareBusinessStrategyBtn).toBeVisible();
    await shareBusinessStrategyBtn.click();

    const nextBtn = page.locator('button:has-text("Next")');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    const executeBtn = page.locator('button:has-text("Execute")');
    await expect(executeBtn).toBeVisible();
    await executeBtn.click();

    // Verify Share Store pops up again
    await expect(page.locator('text=Share Your Store')).toBeVisible();
    await expect(page.locator('text=My Awesome Store')).toBeVisible();

    await page.locator('button:has-text("Close")').click();
  });
});
