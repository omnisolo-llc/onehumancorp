import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Owner Setup and Public Booking Flow', async ({ page }) => {
    // Admin Setup
    await page.goto(`/admin/bookings?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    // Create Resource
    const newResNameInput = page.locator('input[type="text"]').first();
    await newResNameInput.fill('Studio A');
    await page.getByRole('button', { name: 'Add Resource' }).click();

    // Create Availability Block
    const timeInputs = page.locator('input[type="datetime-local"]');
    await timeInputs.nth(0).fill('2025-02-01T09:00');
    await timeInputs.nth(1).fill('2025-02-01T17:00');
    await page.getByRole('button', { name: 'Add Block' }).click();

    // Public Booking
    await page.goto(`/booking?tenant=${tenantId}&service_id=res-1`);
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    const dateQuery = '2025-02-01';
    await page.fill('input[type="date"]', dateQuery);

    await page.waitForSelector('button:has-text("09:00 AM")');
    await page.click('button:has-text("09:00 AM")');

    await page.click('button:has-text("Confirm Booking")');

    await expect(page.getByTestId('booking-checkout-container')).toBeVisible();
  });
});
