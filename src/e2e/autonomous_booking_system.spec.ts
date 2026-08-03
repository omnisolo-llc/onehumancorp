import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {

  test('Owner sets up a new service and availability', async ({ request, loginAs, adminUser, page }) => {
    await loginAs(page, adminUser);
  });

  test('Customer fetches slots and creates a booking requiring a deposit', async ({ request, loginAs, adminUser, page }) => {
    await loginAs(page, adminUser);
  });
});
