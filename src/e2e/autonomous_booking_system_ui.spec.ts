import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form Flow', async ({ page }) => {
    await page.goto(`/booking?tenant=${tenantId}&service_id=mock-service`);
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    await page.goto(`/admin/bookings?tenant=${tenantId}`);
  });
});
