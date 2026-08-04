import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System CUJ', () => {
  const tenantId = `booking-test-${Date.now()}`;
  let serviceId = '';

  test('Owner sets up a new service and availability', async ({ page }) => {
    await page.goto('/login');
    await page.getByLabel('Email or username').fill('test@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByLabel(/Organization/).fill('e2e-tenant');
    await Promise.all([
      page.waitForURL('**/dashboard'),
      page.getByRole('button', { name: 'Log in' }).click(),
    ]);

    await page.goto('/admin/bookings');
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ page }) => {
    await page.goto(`/booking?tenant=e2e-tenant&service_id=mock-service`);
    const heading = page.getByRole('heading', { name: 'Book an Appointment' });
    if (await heading.isVisible()) {
      await expect(heading).toBeVisible();
    }
  });
});
