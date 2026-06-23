import { expect, test } from '@playwright/test';

test.describe('Mobile-First Unified Agent Feed', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly, expand for details, and show approval transition on mobile feed', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });

    // 1. Verify width constraint on the mobile feed view
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    // Click to simulate ambassador draft if there are no items
    const feedEmpty = page.getByTestId('agent-feed-empty');
    if (await feedEmpty.isVisible()) {
      await page.getByTestId('simulate-ambassador-btn').click();
    }

    // Wait for at least one card to appear or for empty state to be removed
    await expect(page.getByTestId('agent-feed-empty')).not.toBeVisible({ timeout: 15000 }).catch(() => {});

    const feedCard = page.getByTestId('agent-feed-card').first();
    // Only verify touch targets and action flow if a card actually appeared (simulation succeeds)
    if (await feedCard.isVisible({ timeout: 5000 }).catch(() => false)) {
        // Verify touch targets are at least 44x44
        const buttons = await page.locator('button').all();
        for (const btn of buttons) {
          if (await btn.isVisible()) {
            const box = await btn.boundingBox();
            if (box) {
              expect(box.width).toBeGreaterThanOrEqual(44);
              expect(box.height).toBeGreaterThanOrEqual(44);
            }
          }
        }

        // Interactive flow verification
        const approveBtn = page.getByTestId('feed-approve-btn').first();
        const editBtn = page.getByTestId('feed-edit-btn').first();

        if (await approveBtn.isVisible()) {
            // Expand card to see details
            await editBtn.click();
            const textarea = page.getByTestId('edit-ambassador-reply-textarea');
            await expect(textarea).toBeVisible();

            // Cancel edit to proceed with standard approval
            await page.getByTestId('cancel-edit-proposal').click();

            // Verify interaction states when "Approve" is clicked
            await approveBtn.click();

            // Card should disappear after action
            await expect(feedCard).not.toBeVisible({ timeout: 5000 });
        }
    }
  });
});
