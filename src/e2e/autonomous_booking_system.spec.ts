import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability', async ({ page }) => {
    await page.goto(`/`);
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ page }) => {
    await page.goto(`/`);
  });
});
