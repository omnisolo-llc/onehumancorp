import { test, expect } from '../fixtures';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should create and retrieve loyalty wallet balance', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/quote.html?id=quote-123');

  });

  test('Should apply points to checkout', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/quote.html?id=quote-123');
  });

  test('Dashboard should have a link to the loyalty widget', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard.html');
  });

});
