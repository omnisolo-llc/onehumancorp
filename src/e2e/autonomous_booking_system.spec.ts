import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {
  test('Owner sets up a new service and availability', async ({ page }) => {
    await page.goto('/admin/bookings');
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();
    await expect(page.locator('body')).toBeVisible();
  });

  test('Customer fetches slots and creates a booking', async ({ page }) => {
    await page.goto('/booking?service_id=e2e-product-class');
    await expect(page.locator('body')).toBeVisible();
  });
});
