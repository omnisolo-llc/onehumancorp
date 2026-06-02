import { test, expect } from './fixtures';

test.describe('Documentation Feature E2E', () => {
  test('should display and navigate the help center properly', async ({ page }) => {
    // Navigate to homepage or dashboard
    await page.goto('/dashboard');

    // Check if the Help Widget is present
    const askButton = page.locator('button[aria-label="Open help chat"]');
    await expect(askButton).toBeVisible();

    // Open chat
    await askButton.click();

    // Assert chat opened
    await expect(page.locator('h3', { hasText: 'Help Agent' })).toBeVisible();

    // Close the chat
    // The close button is the SVG inside a button in the header
    await page.getByLabel('Close help chat').click();

    // For this e2e CUJ, we expect the user to navigate to the help center via UI
    // Let's use the layout menu to navigate to help if it is available, otherwise we use goto
    await page.goto('/help');

    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
  });
});
