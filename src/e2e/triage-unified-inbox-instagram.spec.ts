import { test, expect } from '@playwright/test';

test.describe('Triage Unified Inbox', () => {
  test('Loads inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('body')).toBeVisible();
  });
});
