import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability via UI', async ({ page }) => {
    await page.goto(`/admin/bookings/resources/new`);
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ page }) => {
    await page.goto(`/booking`);
  });
});
