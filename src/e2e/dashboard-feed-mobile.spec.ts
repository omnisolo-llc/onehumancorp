import { expect, test } from './fixtures';

test.describe('Unified Agent Feed Mobile MVP', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays feed and ensures no horizontal scroll on mobile', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('domcontentloaded');

    const feedSection = page.locator('#unified-agent-feed-container').first();
    await expect(feedSection).toBeVisible({ timeout: 15000 });

    // Ensure there is no horizontal scroll on the body
    const isScrollable = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(isScrollable).toBeFalsy();

    // Check tabs touch targets
    const proposalsTab = page.getByRole('button', { name: /Proposals/ });
    const box = await proposalsTab.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });

  test('should allow approving an action card in the feed', async ({ page }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#unified-agent-feed-container').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for approve buttons in the feed
    const approveButtons = feedContainer.locator('[data-testid="feed-approve-btn"]');
    // We expect there to be at least one card generated for triage
    await expect(approveButtons.first()).toBeVisible({ timeout: 15000 });

    // Verify touch target for action buttons
    const box = await approveButtons.first().boundingBox();
    if (box) {
        expect(box.height).toBeGreaterThanOrEqual(44);
        expect(box.width).toBeGreaterThanOrEqual(44);
    }

    // Store count to verify the count decreases
    const initialCount = await approveButtons.count();

    await approveButtons.first().click();

    // Expect the card to disappear or change state, count should be less
    await expect(async () => {
       const newCount = await page.locator('#unified-agent-feed-container').locator('[data-testid="feed-approve-btn"]').count();
       expect(newCount).toBeLessThan(initialCount);
    }).toPass({ timeout: 10000 });
  });

  test('should allow dismissing an action card in the feed', async ({ page }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#unified-agent-feed-container').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for dismiss/reject buttons in the feed
    const rejectButtons = feedContainer.locator('[data-testid="feed-dismiss-btn"]');
    // We expect there to be at least one card generated for triage
    await expect(rejectButtons.first()).toBeVisible({ timeout: 15000 });

    // Store count to verify the count decreases
    const initialCount = await rejectButtons.count();

    await rejectButtons.first().click();

    // Expect the card to disappear or change state, count should be less
    await expect(async () => {
       const newCount = await page.locator('#unified-agent-feed-container').locator('[data-testid="feed-dismiss-btn"]').count();
       expect(newCount).toBeLessThan(initialCount);
    }).toPass({ timeout: 10000 });
  });
});
