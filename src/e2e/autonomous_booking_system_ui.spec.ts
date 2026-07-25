import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto(`/booking`);
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    // 1. Visit admin bookings dashboard
    await page.goto(`/admin/bookings`);
  });
});
