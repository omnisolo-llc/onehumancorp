import { test, expect } from '@playwright/test';

test.describe('Omnibox Global Search', () => {
  test('Cmd+K opens omnibox and searches', async ({ page }) => {
    await page.goto('/dashboard');

    // Press Cmd+K (Mac) or Ctrl+K (Windows/Linux)
    await page.keyboard.press('Meta+K');

    // Check if the input is visible
    const omniboxInput = page.getByPlaceholder('Search customers, orders, messages... (Cmd+K)');
    await expect(omniboxInput).toBeVisible();

    // Type a query
    await omniboxInput.fill('John');

    // Should show loading then results (we can just wait for network idle or a specific result)
    // In a real E2E with mocked backend, we would expect specific items.
    // Here we just verify the overlay exists and functions.
    await page.keyboard.press('Escape');
    await expect(omniboxInput).not.toBeVisible();
  });
});
