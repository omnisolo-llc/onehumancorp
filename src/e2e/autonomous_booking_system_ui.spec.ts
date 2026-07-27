import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form UI mounts properly', async ({ page }) => {
    await page.goto(`/booking?tenant=${tenantId}&service_id=test-service`);
    await expect(page.locator('body')).toBeVisible();
  });
});
