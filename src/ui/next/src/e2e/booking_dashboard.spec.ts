import { test, expect } from '@playwright/test';

test.describe('Provider Booking Dashboard CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard as a logged in user which then goes to booking
    await page.goto('/dashboard/bookings', { waitUntil: 'networkidle' });
  });

  test('Provider sees the Provider Dashboard title and actions', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'Provider Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Manage your upcoming bookings')).toBeVisible();
    await expect(page.locator('button', { hasText: '+ New Booking' })).toBeVisible();
    await expect(page.locator('text=View Calendar')).toBeVisible();
  });

  test('Provider sees the three metric summary cards', async ({ page }) => {
    await expect(page.locator('text=Pending Requests')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Confirmed Upcoming')).toBeVisible();
    await expect(page.locator("text=Today's Bookings")).toBeVisible();
  });

  test('Provider sees Upcoming Bookings section', async ({ page }) => {
    await expect(page.locator('h2', { hasText: 'Upcoming Bookings' })).toBeVisible({ timeout: 15000 });
  });

  test('Empty state or loading state is handled correctly', async ({ page }) => {
    const noBookings = page.locator('text=No upcoming bookings');
    const someBooking = page.locator('text=Manage').first();

    await Promise.any([
      expect(noBookings).toBeVisible({ timeout: 15000 }),
      expect(someBooking).toBeVisible({ timeout: 15000 })
    ]);
  });

  test('Provider can see the Manage button for an existing booking or it shows empty state', async ({ page }) => {
    const noBookings = page.locator('text=No upcoming bookings');
    const manageButton = page.locator('button', { hasText: 'Manage' }).first();

    await Promise.any([
      expect(noBookings).toBeVisible({ timeout: 15000 }),
      expect(manageButton).toBeVisible({ timeout: 15000 })
    ]);
  });

});
