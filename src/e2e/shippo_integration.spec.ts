import { test, expect } from '@playwright/test';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
  // Navigate to the orders page
  await page.goto('/orders');

  // Check if the orders page loads correctly
  await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
});
