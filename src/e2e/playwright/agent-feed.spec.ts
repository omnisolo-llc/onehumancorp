import { test, expect } from '../fixtures';

test.describe('Unified Agent Feed (Mobile MVP)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays action cards and allows approval without horizontal scrolling', async ({ page }) => {
    // Navigate to the feed page directly. The \`loginAs\` fixture (used in page)
    // logs us in. If we need to explicitly navigate to /feed we can:
    await page.goto('/feed');

    // Wait for feed items to load from the real backend, seeded by e2e-seed.sql
    const feedContainer = page.getByTestId('agent-feed');
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Wait a moment for network to settle so cards are rendered
    await page.waitForLoadState('networkidle');

    // Ensure there is no horizontal scroll
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);

    const cards = page.getByTestId('agent-feed-card');

    // There should be at least one card seeded by e2e-seed.sql
    // (e.g., e2e-feed-social, app-test-ab12-34f7-e43e-7264a9c4021d, etc.)
    await expect(cards.first()).toBeVisible({ timeout: 15000 });

    const count = await cards.count();
    expect(count).toBeGreaterThan(0);

    if (count > 0) {
      const firstCard = cards.first();
      const buttons = firstCard.locator('button');
      const buttonCount = await buttons.count();
      for (let i = 0; i < buttonCount; i++) {
          const boundingBox = await buttons.nth(i).boundingBox();
          // Touch targets must be >= 44x44
          expect(boundingBox?.width).toBeGreaterThanOrEqual(44);
          expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
      }

      // Try to find an "Approve" button or similar action button
      const firstApproveButton = buttons.filter({ hasText: /(Approve|Approve & Schedule|Yes, draft it!|Send Draft|Review Estimate|Send Follow-up)/i }).first();

      if (await firstApproveButton.isVisible()) {
        await firstApproveButton.click();

        // Wait for the card to disappear (optimistic update or real update)
        await expect(firstCard).not.toBeVisible({ timeout: 15000 });
      }
    }
  });
});
