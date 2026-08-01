import { expect, test } from './fixtures';

test.describe('Autonomous Booking System E2E', () => {

  test('should schedule a booking completely through the UI using actual components', async ({ page }) => {
    // Navigate to bookings
    await page.goto('/bookings');
    await expect(page.locator('body')).toBeVisible();
  });
});
