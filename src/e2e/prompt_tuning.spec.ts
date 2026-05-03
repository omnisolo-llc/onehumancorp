import { test, expect } from '@playwright/test';

test.describe('Prompt Tuning Flow', () => {
  test('should execute full prompt tuning wizard flow', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Agents then to Prompt Tuning UI
    await page.locator('a:has-text("Agents")').click();
    await page.waitForURL('**/agents');

    // Select an agent to tune
    await page.locator('[class*="card"], [class*="agent"]').first().click();
    await page.locator('button:has-text("Tune"), button:has-text("Configure")').first().click();

    // Step 0: Tone
    await expect(page.locator('text=Prompt Tuning')).toBeVisible();
    await page.locator('text=Friendly & Warm').click();
    await page.locator('button:has-text("Next")').click();

    // Step 1: Focus
    await page.locator('text=Only about my products').click();
    await page.locator('button:has-text("Next")').click();

    // Step 2: Examples
    await page.locator('button:has-text("Next")').click();

    // Step 3: Review & Save
    await expect(page.locator('text=Review & Save')).toBeVisible();
    await page.locator('button:has-text("Save")').click();

    // Verify Success Toast
    await expect(page.locator('text=Your agent has been updated ✓')).toBeVisible();
  });
});
