import { test, expect } from '@playwright/test';

test.describe('Subscriptions', () => {
  test('Can view subscription plans', async ({ page }) => {
    await page.goto('/subscriptions');
    await expect(page.locator('text="Plans"')).toBeVisible();
  });
});
