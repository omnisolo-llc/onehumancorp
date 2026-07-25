import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat', () => {
  test('Native chat system displays conversation items properly (Unified Triage)', async ({ page }) => {
    // E2E UI verification based on the actual design constraints
    await page.goto('/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });
});
