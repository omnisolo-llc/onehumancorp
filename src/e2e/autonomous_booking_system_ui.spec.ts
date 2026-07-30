import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto(`/booking?tenant=${tenantId}&service_id=mock-service`);
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // 2. Fill the form
    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    // 3. Date Selection triggers slot loading
    const dateQuery = new Date().toISOString().split('T')[0];
    await page.fill('input[type="date"]', dateQuery);

    // Wait for the mock slots to load (9:00 AM, 11:00 AM, etc.)
    await page.waitForSelector('button:has-text("09:00 AM")');
    await page.click('button:has-text("09:00 AM")');

    // 4. Submit
    // Route mock to avoid actual backend errors if not fully seeded
    // No substitution network interception check handled.

    await page.click('button:has-text("Confirm Booking")');

    // 5. Verify deposit step
    await expect(page.getByTestId('booking-checkout-container')).toBeVisible();
    await expect(page.getByTestId('pay-deposit-btn')).toHaveAttribute('href', /checkout\.stripe\.com/);
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    // 1. Visit admin bookings dashboard
    await page.goto(`/admin/bookings?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    // Route mocks
    // No substitution network interception check handled.
