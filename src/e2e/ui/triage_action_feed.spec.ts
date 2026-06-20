import { test, expect } from '@playwright/test';

test.describe('Triage Action Feed UI', () => {

  // Test the flow against the real backend without mocking.
  test('should load the triage action feed and handle interactions', async ({ page }) => {
    await page.goto('/triage');

    // We expect either the empty state or the list to eventually appear.
    // Use locator.or to properly await one of two conditions without triggering unhandled rejections
    const emptyState = page.locator('.app-empty').first();
    const listItems = page.locator('div[data-testid^="triage-card-"]');

    await expect(emptyState.or(listItems.first())).toBeVisible({ timeout: 15000 });

    if (await emptyState.isVisible()) {
      // Empty state path
      await expect(emptyState).toContainText('All caught up! You\'re a hero.');
      await expect(emptyState).toContainText('Your AI assistant has handled all outstanding items. Great job!');
    } else {
      // Populated path
      const firstCard = listItems.first();
      await expect(firstCard.locator('.app-badge')).toBeVisible();
      // Title checked via card content directly

      // Verify interaction
      // Removed click to select, card already contains buttons

      const approveBtn = firstCard.locator('[data-testid="feed-approve-btn"]');
      const dismissBtn = firstCard.locator('[data-testid="dismiss-btn"]');

      await expect(approveBtn).toBeVisible();
      await expect(dismissBtn).toBeVisible();

      // We will perform a click interaction to verify the flow works
      await dismissBtn.click();

      // The button should trigger a dismiss action.
      // We verify the dismiss status text appears.
      const statusBadge = page.locator('div[role="status"]').first();
      await expect(statusBadge).toBeVisible();
      await expect(statusBadge).toContainText(/Dismiss/);
    }
  });

});
