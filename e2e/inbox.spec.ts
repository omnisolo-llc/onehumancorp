import { test, expect } from '@playwright/test';

test.describe('Inbox Page E2E', () => {
  test('should load the inbox, fetch empty data from backend, and allow simulated incoming messages with AI glassmorphism UI', async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');

    // Wait for the customer inbox heading
    await expect(page.locator('h1', { hasText: 'Customer Inbox' })).toBeVisible();

    // Verify it is empty initially (no "Are you open today?" or similar mock data)
    await expect(page.locator('text=Are you open today?')).not.toBeVisible();

    // Click to simulate an incoming message
    await page.locator('button', { hasText: '🤖 Simulate Incoming Message' }).click();

    // Verify incoming message appears
    await expect(page.locator('text=Are you open today?')).toBeVisible();

    // Wait for AI draft reply to generate (it has a 500ms timeout)
    await expect(page.locator('text=AI Draft')).toBeVisible({ timeout: 2000 });

    // Verify glassmorphism style confidence tag
    await expect(page.locator('text=High Confidence')).toBeVisible();

    // Verify draft text
    await expect(page.locator('text=Hi! Yes, we are open until 6 PM today')).toBeVisible();

    // Send the draft reply
    await page.locator('button:has-text("Send")').nth(1).click();

    // Verify it appeared in the UI as 'Me'
    await expect(page.locator('.text-right text=Hi! Yes, we are open until 6 PM today')).toBeVisible();
  });
});
