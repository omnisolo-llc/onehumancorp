import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Unified Booking Owner Dashboard', () => {

  test('Owner can navigate to bookings from dashboard and see upcoming bookings', async ({ adminPage }) => {
    await adminPage.goto('/dashboard.html');
    await adminPage.waitForTimeout(500);

    // Verify Bookings link exists in the dashboard
    await expect(adminPage.getByRole('link', { name: 'Bookings' })).toBeVisible();

    // Click on Bookings link
    await adminPage.getByRole('link', { name: 'Bookings' }).click();

    // Verify we are on the bookings page
    await expect(adminPage).toHaveURL(/booking\.html/);

    // Verify Upcoming tab is active
    await expect(adminPage.locator('#tab-upcoming')).toHaveClass(/active/);

    // Verify booking list is loaded or shows empty state
    await expect(adminPage.locator('#upcoming-list')).toBeVisible();

    // Click on Requests tab
    await adminPage.locator('#tab-requests').click();
    await expect(adminPage.locator('#requests-view')).toBeVisible();

    // Fill form and create request
    await adminPage.locator('#description').fill('Needs a follow-up appointment next week.');
    await adminPage.getByRole('button', { name: 'Create Request' }).click();

    // Verify success state
    await expect(adminPage.getByRole('button', { name: 'Created! (Check Inbox)' })).toBeVisible();
  });
});
