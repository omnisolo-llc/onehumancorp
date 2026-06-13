import { test, expect } from '@playwright/test';

test.describe('Loyalty Program Page E2E', () => {
  test('should allow user to generate and view loyalty campaign', async ({ page }) => {
    // Go to the loyalty program page
    await page.goto('/loyalty-program');

    // Expect the page title to be visible
    await expect(page.getByText('Customer Loyalty Program 🤝')).toBeVisible();

    // Fill in the details
    await page.fill('input[placeholder="e.g. 10"]', '15'); // give amount
    await page.fill('input[placeholder="e.g. 10"]', '15'); // get amount
    await page.selectOption('select', 'Store Credit');

    // Click generate
    await page.getByText('Generate Email').click();

    // Check that the real backend response is shown
    await expect(page.getByDisplayValue(/Subject: Welcome to/)).toBeVisible({ timeout: 15000 });
    await expect(page.getByDisplayValue(/15/)).toBeVisible();
  });
});
