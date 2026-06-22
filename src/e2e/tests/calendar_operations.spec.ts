import { test, expect } from '@playwright/test';

test.describe('Calendar & Operations Assistant', () => {
  test('should display upcoming appointments and integrate with Operations Assistant', async ({ page }) => {
    // Intercept the API call to return our mock bookings since we can't easily seed the local db with Playwright yet
    await page.route('**/api/ui/bookings*', async route => {
      const json = [
        {
          id: "book_1",
          customer_name: "Sarah Jenkins",
          product_title: "Guitar Lesson - Jazz Scales",
          start_time: new Date(Date.now() + 2 * 60 * 60 * 1000).toISOString(),
          status: "confirmed",
          ai_scheduled: true
        },
        {
          id: "book_2",
          customer_name: "Michael Chen",
          product_title: "Guitar Lesson - Basics",
          start_time: new Date(Date.now() + 4 * 60 * 60 * 1000).toISOString(),
          status: "pending",
          ai_scheduled: false
        }
      ];
      await route.fulfill({ json });
    });

    await page.goto('/calendar?tenant_id=e2e-tenant');

    // Wait for the calendar to load
    await expect(page.locator('h1').filter({ hasText: 'Calendar & Bookings' })).toBeVisible();

    // Verify that appointments are loaded
    await expect(page.locator('h2').filter({ hasText: 'Upcoming Appointments' })).toBeVisible();

    // In our mock data, there are 2 bookings
    const bookings = page.locator('.app-card').first().locator('.border.border-gray-100.rounded-lg');
    await expect(bookings).toHaveCount(2);

    // Verify booking details
    await expect(bookings.first()).toContainText('Guitar Lesson - Jazz Scales');
    await expect(bookings.first()).toContainText('Sarah Jenkins');
    await expect(bookings.first()).toContainText('Confirmed');

    await expect(bookings.nth(1)).toContainText('Guitar Lesson - Basics');
    await expect(bookings.nth(1)).toContainText('Michael Chen');
    await expect(bookings.nth(1)).toContainText('Pending');

    // Check Operations Agent panel
    await expect(page.locator('h2').filter({ hasText: 'Operations Agent' })).toBeVisible();
  });
});
