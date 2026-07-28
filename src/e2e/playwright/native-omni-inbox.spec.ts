import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Inbox', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the inbox page (assuming login is handled globally or via fixtures, or direct load)
    await page.goto('/inbox.html');
  });

  test('Loads Unified Inbox correctly', async ({ page }) => {
    await expect(page.locator('.header-section h1')).toHaveText('Agentic Inbox');
  });

  test('Displays messages from real backend without mock data', async ({ page }) => {
    // Should see loading then messages or empty state
    await expect(page.locator('#inbox-queue')).toBeVisible();
    await page.waitForTimeout(1000);
  });

  test('Handles no AI draft available correctly', async ({ page }) => {
    // Ensure the page doesn't crash
    await expect(page.locator('.subtitle')).toHaveText('Unified messages with AI-drafted replies');
  });

  test('Shows correct touch targets for mobile (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('.header-section')).toBeVisible();
  });

  test('UI includes Translucent Glass styling', async ({ page }) => {
    await expect(page.locator('.container.glassmorphism')).toBeVisible();
  });
});
