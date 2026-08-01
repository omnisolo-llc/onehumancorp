import { test, expect } from '@playwright/test';
test.describe('Autonomous Booking System UI CUJ', () => {
  test('Customer navigates to booking flow', async ({ page }) => {
    await page.goto('/booking');
    await expect(page.locator('body')).toBeVisible();
  });
});