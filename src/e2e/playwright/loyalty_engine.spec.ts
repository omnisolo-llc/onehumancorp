import { test, expect } from '@playwright/test';

test.describe('Loyalty Engine', () => {
  test('Customer view shows points', async ({ page }) => {
    await page.goto('/loyalty');
    await expect(page.locator('text="Your Points"')).toBeVisible();
  });
});
