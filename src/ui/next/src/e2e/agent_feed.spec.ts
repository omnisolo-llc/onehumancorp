import { test, expect } from '@playwright/test';

test.describe('Agentic Unified Intake & Action Feed', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display agent feed and have no horizontal scroll', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);
  });

  test('should have 44x44px minimum touch targets on buttons', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    const emptyStateVisible = await page.getByTestId('agent-feed-empty').isVisible();
    if (!emptyStateVisible) {
        const approveBtn = page.getByTestId('feed-approve-btn').first();
        const box = await approveBtn.boundingBox();
        expect(box?.height).toBeGreaterThanOrEqual(44);
        expect(box?.width).toBeGreaterThanOrEqual(44);

        const dismissBtn = page.getByTestId('feed-dismiss-btn').first();
        const dBox = await dismissBtn.boundingBox();
        expect(dBox?.height).toBeGreaterThanOrEqual(44);
        expect(dBox?.width).toBeGreaterThanOrEqual(44);
    }
  });

  test('should process approve action with optimistic UI update', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    const cardsVisible = await page.getByTestId('agent-feed-card').count() > 0;
    if (cardsVisible) {
      const feedCard = page.getByTestId('agent-feed-card').first();
      await expect(feedCard).toBeVisible();

      const approveBtn = feedCard.getByTestId('feed-approve-btn');
      await approveBtn.click();

      // Card should be optimistically removed
      await expect(feedCard).not.toBeVisible();
    }
  });

  test('should display empty state when all caught up', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    // If there are cards, dismiss them all
    let cardsCount = await page.getByTestId('agent-feed-card').count();
    while(cardsCount > 0) {
      const feedCard = page.getByTestId('agent-feed-card').first();
      const dismissBtn = feedCard.getByTestId('feed-dismiss-btn');
      await dismissBtn.click();
      await expect(feedCard).not.toBeVisible();
      cardsCount = await page.getByTestId('agent-feed-card').count();
    }

    const emptyState = page.getByTestId('agent-feed-empty');
    await expect(emptyState).toBeVisible();
    await expect(emptyState).toContainText("You're all caught up!");
  });

  test('should load feed without errors', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    // No error overlay visible
    const errorNode = page.locator('text="We couldn\'t load your feed."');
    await expect(errorNode).not.toBeVisible();
  });
});
