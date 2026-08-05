import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  test('Owner can view booking system', async ({ adminPage }) => {
    const page = await adminPage;
    await page.goto('/booking');
    await expect(page.locator('body')).toBeVisible();
  });
});
