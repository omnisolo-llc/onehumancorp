import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form Flow', async ({ page }) => {
    await page.goto(`/booking?tenant=${tenantId}&service_id=mock-service`);
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    const dateQuery = new Date().toISOString().split('T')[0];
    await page.fill('input[type="date"]', dateQuery);
  });
});
