import { test, expect } from '@playwright/test';

test.describe('OHC Setup HTML UI Integration', () => {
  test('Instant Build integration handles backend errors without mock data', async ({ page }) => {
    // Navigate to the real stack (not a file:// URL)
    await page.goto('/setup.html');

    // Wait for the page to load
    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // Click Instant Build
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // Ensure it navigates to the Instant Build step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // Test empty submission validation
    await page.getByRole('button', { name: 'Generate Storefront' }).click();
    await expect(page.getByText('Please tell us about your business.')).toBeVisible();

    // Fill bio with a very short failing query that real LLM might fail on, or just standard query.
    // If the real API works, it will proceed to 'You're Live!'. If it fails (e.g., no MINIMAX_API_KEY in test environment), it will show the error gracefully.
    await page.getByPlaceholder(/e.g. I run a local bakery/i).fill('Test bio for playwright.');

    // We do NOT mock the API here.
    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    // It should either succeed or show an error
    const result = await Promise.race([
      page.getByText("You're Live!").waitFor({ state: 'visible', timeout: 30000 }).then(() => 'success'),
      page.getByText("Failed to launch. Please try again.").waitFor({ state: 'visible', timeout: 30000 }).then(() => 'error')
    ]);

    expect(['success', 'error']).toContain(result);
  });
});
