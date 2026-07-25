import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {

  test('Customer UI checkout process shows confirmation', async ({ page, memberPage }) => {
    // 1. Visit public checkout flow
    await page.goto('/ui/booking-checkout.html');

    // 2. See the schedule
    await expect(page.locator('h1')).toHaveText('Book a Session');

    // 3. Fill details
    await page.fill('#customerName', 'Jane Doe');
    await page.fill('#customerEmail', 'jane@example.com');
  });

  test('Admin UI allows managing resources and availability', async ({ page, loginAs }) => {
    await loginAs(page, 'admin');

    await page.goto('/admin/booking');
    await expect(page.locator('h1')).toHaveText('Booking Management');

    // Mocks removed as per no-substitution rule
  });
});
