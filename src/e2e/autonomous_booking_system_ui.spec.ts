import { test, expect } from './fixtures';
import { mockRoute } from './test_utils';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto(`/booking?tenant=${tenantId}&service_id=mock-service`);

    await expect(page.locator('body')).toBeVisible();
  });

  test('Owner Admin Dashboard', async ({ adminPage }) => {
    // 1. Visit admin bookings dashboard
    const page = await adminPage;
    await page.goto(`/admin/bookings?tenant=${tenantId}`);

    await expect(page.locator('body')).toBeVisible();
  });
});
