import { test, expect } from '@playwright/test';

test.describe('Perplexity Harness UI', () => {
  test('should allow user to submit a query and see the response', async ({ page }) => {
    // We navigate to the newly created route
    await page.goto('http://localhost:3000/perplexity-harness');

    // Check header
    await expect(page.locator('h1')).toHaveText('Perplexity-style Agent Harness');

    // Fill query
    await page.fill('input[type="text"]', 'Why is the sky blue?');

    // Submit
    await page.click('button[type="submit"]');

    // Check loading state
    await expect(page.locator('button[type="submit"]')).toHaveText('Searching...');

    // Wait for response
    const responseLocator = page.locator('div.bg-gray-50 p');
    await expect(responseLocator).toBeVisible({ timeout: 5000 });

    // Check response content
    await expect(responseLocator).toContainText('According to source [1]');
  });
});
