import { test, expect } from '@playwright/test';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should load POS page and handle absence of inventory gracefully', async ({ page }) => {
    await page.goto('/pos/terminal');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('text=Not Clocked In').or(page.locator('text=Terminal Locked')).or(page.locator('text=Select Item')).first()).toBeVisible({ timeout: 15000 });
  });
});
