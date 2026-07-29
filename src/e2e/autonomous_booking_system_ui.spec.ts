import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  test('Public Booking Form Flow', async ({ page }) => {
    await page.goto('/booking?service_id=e2e-product-class');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    await page.goto('/admin/bookings');
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();
    await expect(page.locator('body')).toBeVisible();
  });
});
