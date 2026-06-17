import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System CUJ', () => {
  test('Leo the Music Tutor can view his dashboard, and a student can book a lesson', async ({ page }) => {
    // Navigate to the test frontend page
    await page.goto('/booking');

    // Fill out the booking form
    // Wait for the form to load
    await page.waitForSelector('form');

    // Fill in basic details
    await page.getByPlaceholder('Jane Doe').fill('Alice Student');
    await page.getByPlaceholder('jane@example.com').fill('alice@example.com');

    // Select date (tomorrow)
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    const dateString = tomorrow.toISOString().split('T')[0];
    await page.locator('input[type="date"]').fill(dateString);

    // Fill out description
    await page.getByPlaceholder('What do you need help with?').fill('I need a 2-hour piano lesson.');

    // Click confirm
    await page.getByRole('button', { name: 'Confirm Booking' }).click();

    // Verify success view
    await expect(page.getByText('Request Sent!')).toBeVisible();
  });
});
