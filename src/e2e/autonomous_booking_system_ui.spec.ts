import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  test('Dashboard shows booking options', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text="Bookings"')).toBeVisible();
  });
});
