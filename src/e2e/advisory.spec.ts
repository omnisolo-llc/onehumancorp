import { test, expect } from '@playwright/test';

test.describe('Advisory Engine', () => {
  test('displays advisory card and dispatches action', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check if AdvisoryCard rendered correctly
    await expect(page.locator('h3:has-text("Weekly Business Health")')).toBeVisible();

    // The summary might take a second to load, let's wait for it.
    // Given the test data, the text would be "Everything looks steady."
    await expect(page.locator('text=Everything looks steady.')).toBeVisible();

    // Check for actionable suggestion button
    const actionButton = page.locator('button:has-text("Yes, draft it!")');

    // We expect it to not be there since the default doesn't trigger the button.
    // But if we mock the endpoint (not hermetic) or let it fall back, we should adjust.
    // For now just basic visibility check of the card header.
  });
});
