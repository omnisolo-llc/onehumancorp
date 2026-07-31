import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability', async ({ page }) => {
    // Navigate to booking page
    await page.goto('/login');
  });
});
