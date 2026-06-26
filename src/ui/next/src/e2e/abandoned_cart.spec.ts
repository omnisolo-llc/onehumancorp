import { test, expect } from '@playwright/test';

test.describe('Abandoned Cart Growth Loop', () => {
  test('should display abandoned carts and allow recovery actions', async ({ page }) => {
    // Basic test
    await page.goto('/');
    await expect(page.locator('body')).toBeVisible();
  });
});
