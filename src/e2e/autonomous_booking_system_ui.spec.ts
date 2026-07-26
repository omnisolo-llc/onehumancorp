import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {

  test('Owner Admin Dashboard', async ({ page, adminUser, loginAs }) => {
    // 1. Visit admin bookings dashboard
    await loginAs(page, adminUser);
    await page.goto(`/admin/bookings`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible({ timeout: 15000 });

    // 2. We skip mocking and rely on real layout elements
    await expect(page.getByRole('button', { name: 'Add Resource' })).toBeVisible({ timeout: 15000 });
  });
});
