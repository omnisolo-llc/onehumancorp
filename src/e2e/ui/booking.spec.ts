import { test, expect } from '@playwright/test';

test.describe('Booking time slot reservation', () => {
  test('should submit booking request and show success', async ({ page }) => {
    // Navigate to booking page directly. The test database has the default tenant populated.
    await page.goto('/booking?tenant=default-store&service_id=service-1');

    // Fill out form
    await page.fill('input[type="text"]', 'John Doe');
    await page.fill('input[type="email"]', 'john@example.com');
    await page.fill('input[type="date"]', '2025-01-01');

    // Wait for slots to appear
    await page.waitForSelector('text=09:00 AM');

    // Select slot
    await page.click('text=09:00 AM');

    // Submit
    await page.click('button[type="submit"]');

    await expect(page.locator('text=Request Sent!')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Submit Another Request' })).toBeVisible();
  });
});
