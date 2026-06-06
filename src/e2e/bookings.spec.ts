import { test, expect } from '@playwright/test';

test.describe('Bookings Page', () => {
  test('should display new booking requests on mobile viewport', async ({ page }) => {
    // Set viewport to mobile size
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/bookings');

    await expect(page.getByText('Agenda')).toBeVisible();
    await expect(page.getByText('Guitar Lesson')).toBeVisible();
    await expect(page.getByText('John Doe')).toBeVisible();
    await expect(page.getByText('10:00 AM - 11:00 AM')).toBeVisible();

    await expect(page.getByRole('button', { name: 'Approve' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Decline' })).toBeVisible();
  });
});
