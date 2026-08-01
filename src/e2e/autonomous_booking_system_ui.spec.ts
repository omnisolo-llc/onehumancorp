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
    await expect(page.getByText('9:00 AM')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('11:00 AM')).toBeVisible();

    // 4. Select a slot
    await page.getByText('9:00 AM').click();
    await expect(page.getByText('Selected: 9:00 AM')).toBeVisible();

    // 5. Submit booking
    await page.getByRole('button', { name: 'Confirm Booking' }).click();

    // Wait for checkout flow to trigger
    await expect(page.getByRole('heading', { name: 'Booking Confirmed' })).toBeVisible({ timeout: 10000 });
  });

  test('Admin Resource UI Flow', async ({ page }) => {
    await page.goto(`/admin/booking/resources?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Resources' })).toBeVisible();

    // Fill new resource
    await page.fill('input[name="name"]', 'Leo');
    await page.fill('input[name="description"]', 'Music Tutor');
    await page.selectOption('select[name="type"]', 'provider');
    await page.getByRole('button', { name: 'Add Resource' }).click();

    await expect(page.getByText('Leo - provider')).toBeVisible({ timeout: 5000 });
  });

  test('Admin Availability UI Flow', async ({ page }) => {
    await page.goto(`/admin/booking/availability?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Availability' })).toBeVisible();

    await page.selectOption('select[name="resource_id"]', 'leo-123');
    await page.fill('input[name="start_time"]', '2025-12-01T09:00');
    await page.fill('input[name="end_time"]', '2025-12-01T17:00');

    await page.getByRole('button', { name: 'Add Availability' }).click();

    await expect(page.getByText('2025-12-01T09:00 to 2025-12-01T17:00')).toBeVisible({ timeout: 5000 });
  });
});
