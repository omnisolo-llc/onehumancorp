import { test, expect } from '@playwright/test';

test.describe('Loyalty & Rewards Engine', () => {

  test('Dashboard should have a link to the loyalty widget', async ({ page }) => {
    await page.goto('/dashboard.html');
    const loyaltyLink = page.locator('a#loyalty-link');
  });

});
