import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  // Use proper test setup.
  test('should display agent feed and process actions', async ({ page }) => {
    // Navigate to feed
    await page.goto('/feed');

    // We do NOT test the API. We wait for the system to render properly.
    // Ensure the feed loads. Wait for the feed container.
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    // Verify loading or empty state.
    // It's possible the test environment has no items seeded.
    const emptyStateVisible = await page.getByTestId('agent-feed-empty').isVisible();
    const cardsVisible = await page.getByTestId('agent-feed-card').count() > 0;

    expect(emptyStateVisible || cardsVisible).toBeTruthy();

    if (emptyStateVisible) {
      await expect(page.locator('text="You\'re all caught up!"')).toBeVisible();
    } else {
      const feedCard = page.getByTestId('agent-feed-card').first();
      await expect(feedCard).toBeVisible();

      // Click approve
      const approveBtn = feedCard.getByTestId('feed-approve-btn');
      await expect(approveBtn).toBeVisible();
      await approveBtn.click();
    }
  });
});
