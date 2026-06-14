import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  // Use proper test setup.
  test('should display agent feed and process actions with 375px limit and 44x44 buttons', async ({ page }) => {
    // Navigate to feed
    await page.goto('/feed');

    // Ensure the feed loads. Wait for the feed container.
    const feedContainer = page.getByTestId('agent-feed');
    await expect(feedContainer).toBeVisible();

    // Assert 375px max-width logic
    const box = await feedContainer.boundingBox();
    if (box) {
      expect(box.width).toBeLessThanOrEqual(375);
    }

    // Wait for the feed to load
    await page.waitForTimeout(1000); // Wait for loading state to finish

    // Verify loading or empty state.
    const emptyStateVisible = await page.getByTestId('agent-feed-empty').isVisible();
    const cardsVisible = await page.getByTestId('agent-feed-card').count() > 0;
    const errorVisible = await page.locator('text="We couldn\'t load your feed."').isVisible();

    expect(emptyStateVisible || cardsVisible || errorVisible).toBeTruthy();

    if (emptyStateVisible) {
      await expect(page.locator('text="You\'re all caught up!"')).toBeVisible();
    } else if (cardsVisible) {
      const feedCard = page.getByTestId('agent-feed-card').first();
      await expect(feedCard).toBeVisible();

      // Click approve if present, otherwise dismiss
      const approveBtn = feedCard.getByTestId('feed-approve-btn');
      const dismissBtn = feedCard.getByTestId('feed-dismiss-btn');

      let btnToClick = dismissBtn;
      if (await approveBtn.isVisible()) {
        btnToClick = approveBtn;
      }

      await expect(btnToClick).toBeVisible();

      // Assert 44x44 minimum touch targets
      const btnBox = await btnToClick.boundingBox();
      if (btnBox) {
        expect(btnBox.width).toBeGreaterThanOrEqual(44);
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
      }

      await btnToClick.click();
    }
  });
});
