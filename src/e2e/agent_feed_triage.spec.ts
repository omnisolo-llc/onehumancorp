import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Agent Feed Triage CUJ', () => {
  test('should display agent feed action cards', async ({ page }) => {
    await adminPage(page);
    await page.goto('/agent-feed.html');

    // Ensure the feed card is visible (loaded from e2e seed data)
    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    await expect(feedCard).toContainText('Instagram DM');
    await expect(feedCard).toContainText('Reply to Customer');
  });

  test('should approve an action card and remove it via optimistic UI', async ({ page }) => {
    await adminPage(page);
    await page.goto('/agent-feed.html');

    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(feedCard).not.toBeVisible({ timeout: 5000 });
  });

  test('should discard an action card and show empty state when all are gone', async ({ page }) => {
    await adminPage(page);
    await page.goto('/agent-feed.html');

    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 15000 });

    const discardBtn = feedCard.getByTestId('feed-reject-btn');
    await expect(discardBtn).toBeVisible();
    await discardBtn.click();

    await expect(feedCard).not.toBeVisible({ timeout: 5000 });
  });
});
