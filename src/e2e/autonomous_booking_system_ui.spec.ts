import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {

  test('Public Booking Form Flow', async ({ page }) => {
    await page.goto(`/`);
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    await page.goto(`/`);
  });
});
