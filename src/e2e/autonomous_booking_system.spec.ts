import { test, expect } from '@playwright/test';

test.describe('Booking System', () => {
  test('Loads booking', async ({ page }) => {
    await page.goto('/booking');
    await expect(page.locator('body')).toBeVisible();
  });
});
