import { test, expect } from '@playwright/test';

test.describe('Social Media Auto-Posting', () => {
  test('should display connected accounts and schedule', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard
    await page.waitForURL('**/*');

    // Wait for the "Auto-Post" button on the dashboard and click it
    const autoPostBtn = page.locator('button:has-text("Auto-Post")').first();
    await expect(autoPostBtn).toBeVisible();
    await autoPostBtn.click();

    // 3. Verify the component is visible
    await expect(page.locator('text=Social Media Auto-Posting').first()).toBeVisible();

    // Verify Connected Accounts Section
    await expect(page.locator('text=Connected Accounts')).toBeVisible();
    await expect(page.locator('text=📷 Instagram')).toBeVisible();
    await expect(page.locator('text=📘 Facebook')).toBeVisible();
    await expect(page.locator('text=🐦 X')).toBeVisible();

    // Check default state matching rust slint file (instagram connected, facebook not, x connected)
    const facebookConnectBtn = page.locator('text=📘 Facebook').locator('..').locator('button:has-text("Connect")').first();
    await expect(facebookConnectBtn).toBeVisible();

    // Verify Schedule Section
    await expect(page.locator('text=AI Generated Post Schedule')).toBeVisible();

    // Verify at least one scheduled post is visible
    await expect(page.locator('text=Instagram').nth(1)).toBeVisible();
    await expect(page.locator('text=Check out our new products!')).toBeVisible();

    // Verify approving a post
    const approveBtn = page.locator('button:has-text("Approve")').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(page.locator('text=Post approved successfully! AI will schedule it.')).toBeVisible();

    // Close the dialog
    const closeBtn = page.locator('button:has-text("Close")');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();
  });
});
