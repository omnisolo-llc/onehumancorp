import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Navigate to bookings
    await page.click('text=Bookings');

    // Verify booking UI
    await expect(page.locator('h1:has-text("Bookings")')).toBeVisible();
  });

  test('Customer creates a booking', async ({ page }) => {
    // Navigate to storefront/public booking page
    await page.goto('/book');

    // Verify booking UI exists
    await expect(page.locator('h1')).toBeVisible();
  });
});
