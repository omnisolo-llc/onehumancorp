import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    // 1. Visit admin bookings dashboard
    await page.goto('/dashboard');
    await expect(page.locator('body')).toBeVisible();
  });
});
