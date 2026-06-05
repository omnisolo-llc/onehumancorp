import { test, expect } from '@playwright/test';

test.describe('Native Booking Flow', () => {
  test('Provider can view upcoming bookings dashboard', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard/bookings');

    // Check if the title is present
    await expect(page.locator('h1')).toHaveText('Upcoming Bookings');

    // Wait for the mock data to load
    await page.waitForTimeout(1500);

    // Check if there are bookings rendered
    const bookingCards = page.locator('.bg-white\\/65');
    await expect(bookingCards).toHaveCount(2); // Since we have 2 mock bookings

    // Check the contents of the first booking
    await expect(bookingCards.nth(0)).toContainText('Plumbing Fix');
    await expect(bookingCards.nth(0)).toContainText('Alice Smith');
    await expect(bookingCards.nth(0)).toContainText('Scheduled');

    // Check the contents of the second booking
    await expect(bookingCards.nth(1)).toContainText('Painting');
    await expect(bookingCards.nth(1)).toContainText('Bob Johnson');
    await expect(bookingCards.nth(1)).toContainText('Pending');
  });
});
