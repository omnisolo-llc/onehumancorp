import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Mobile Unified Feed MVP', () => {
  // Use a mobile viewport to simulate 375px
  test.use({ viewport: { width: 375, height: 667 } });

  test('should load the unified feed and process agent cards', async ({ adminPage: page }) => {
    // We will inject the feed UI HTML to test it directly in isolation,
    // or navigate to a test route if it were integrated in the app.
    // For this E2E test, we will navigate to the page and verify the interactions.
    await page.goto('/unified-feed.html');

    // Wait for the main feed container
    await expect(page.locator('#feed-container')).toBeVisible();

    // Verify 3 agent cards are visible initially
    const cards = page.locator('[data-testid="agent-feed-card"]');
    await expect(cards).toHaveCount(3);

    // Verify horizontal scrolling is not possible
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBe(375);

    // Verify touch targets are at least 44x44
    const primaryBtns = page.locator('.btn-primary, .btn-primary-ops');
    const firstBtnBox = await primaryBtns.first().boundingBox();
    expect(firstBtnBox?.height).toBeGreaterThanOrEqual(44);
    expect(firstBtnBox?.width).toBeGreaterThanOrEqual(44);

    // Process a card
    const firstApproveBtn = page.locator('[data-testid="feed-approve-btn"]').first();
    await firstApproveBtn.click();

    // Verify the card goes into a processing state then disappears
    // Card should eventually be removed, leaving 2 cards
    await expect(cards).toHaveCount(2, { timeout: 2000 });

    // Process remaining cards
    await page.locator('[data-testid="feed-approve-btn"]').first().click();
    await expect(cards).toHaveCount(1, { timeout: 2000 });

    await page.locator('[data-testid="feed-approve-btn"]').first().click();

    // Verify empty state is shown
    await expect(cards).toHaveCount(0, { timeout: 2000 });
    await expect(page.locator('[data-testid="triage-feed-empty"]')).toBeVisible();
  });
});
