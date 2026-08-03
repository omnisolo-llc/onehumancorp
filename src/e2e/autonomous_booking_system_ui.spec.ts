import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {

  test('Public Booking Form Flow', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
  });

  test('Owner Admin Dashboard', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
  });
});
