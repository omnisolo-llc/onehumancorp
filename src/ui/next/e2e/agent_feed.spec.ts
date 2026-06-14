import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test('should display agent feed and process actions', async ({ page }) => {
    // Navigate to feed
    await page.goto('/feed');

    await expect(page.getByTestId('agent-feed')).toBeVisible();

    const emptyStateVisible = await page.getByTestId('agent-feed-empty').isVisible();
    const cardsVisible = await page.getByTestId('agent-feed-card').count() > 0;

    expect(emptyStateVisible || cardsVisible).toBeTruthy();

    if (emptyStateVisible) {
      await expect(page.locator('text="You\'re all caught up!"')).toBeVisible();
    } else {
      const feedCard = page.getByTestId('agent-feed-card').first();
      await expect(feedCard).toBeVisible();

      // Click card to expand
      await feedCard.click();

      // Ensure textarea is visible when expanded
      const editArea = feedCard.getByTestId('feed-edit-textarea');
      await expect(editArea).toBeVisible();

      // Optionally edit the text
      await editArea.fill('Edited AI response text');

      // Click send/approve
      const approveBtn = feedCard.getByTestId('feed-approve-btn');
      await expect(approveBtn).toBeVisible();
      await expect(approveBtn).toHaveText('Send');
      await approveBtn.click();
    }
  });
});
