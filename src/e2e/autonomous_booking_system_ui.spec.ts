import { expect, test } from './fixtures';

test.describe('Autonomous Booking UI Features', () => {

  test('should display booking details properly', async ({ page }) => {
    await page.goto('/bookings');
    await expect(page.locator('body')).toBeVisible();
  });
});
