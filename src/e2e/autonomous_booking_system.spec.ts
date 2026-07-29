import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {
  const tenantId = `booking-test-${Date.now()}`;

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ page }) => {
    await page.goto(`/booking?tenant=${tenantId}`);
  });
});
