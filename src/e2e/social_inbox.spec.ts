import { test, expect } from '@playwright/test';

test.describe('Unified Social Inbox', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
    await page.click('text=Inbox');
    await expect(page).toHaveURL(/.*\/inbox/);
  });

  test('default view shows settings tab', async ({ page }) => {
    // Should be on settings tab by default
    await expect(page.locator('h2:has-text("Connect Channels")')).toBeVisible();
    await expect(page.locator('button:has-text("Connect Channels")')).toBeVisible();

    // Switch to inbox tab
    await page.click('button:has-text("Inbox")');
    // Inbox should be empty initially
    await expect(page.locator('h3:has-text("Your Inbox is Empty")')).toBeVisible();
  });

  test('user can connect channels and view messages', async ({ page }) => {
    // Click Connect Channels
    await page.click('button:has-text("Connect Channels")');

    // Should automatically switch to Inbox and show messages
    await expect(page.locator('text=Facebook User')).toBeVisible();
    await expect(page.locator('text=Instagram User')).toBeVisible();
    await expect(page.locator('text=WhatsApp User')).toBeVisible();
  });

  test('user can switch between inbox and settings tabs', async ({ page }) => {
    // Start on settings
    await expect(page.locator('h2:has-text("Connect Channels")')).toBeVisible();

    // Go to inbox
    await page.click('button:has-text("Inbox")');
    await expect(page.locator('text=Your Inbox is Empty')).toBeVisible();

    // Go back to settings
    await page.click('button:has-text("Settings")');
    await expect(page.locator('h2:has-text("Connect Channels")')).toBeVisible();
  });

  test('settings tab reflects connected state', async ({ page }) => {
    // Connect channels
    await page.click('button:has-text("Connect Channels")');

    // Switch back to settings
    await page.click('button:has-text("Settings")');

    // Should show connected state
    await expect(page.locator('text=Channels Connected')).toBeVisible();
    await expect(page.locator('text=Your accounts are securely synced.')).toBeVisible();
  });

  test('user can send an AI draft reply to a message', async ({ page }) => {
    // Connect channels first
    await page.click('button:has-text("Connect Channels")');

    // Ensure we are in inbox and can see messages
    await expect(page.locator('text=Facebook User')).toBeVisible();

    // Find the first send button associated with the AI Draft
    // Playwright locator for the first "Send" button that is visible
    await page.locator('button:has-text("Send")').first().click();

    // Verify reply appears as sent by "Me"
    await expect(page.locator('span:text-is("Me")').first()).toBeVisible();
  });
});
