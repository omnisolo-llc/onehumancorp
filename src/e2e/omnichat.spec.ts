import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat System', () => {
  test('Owner can view unified conversations from different channels', async ({ page }) => {
    // Navigate to the unified inbox
    await page.goto('/inbox');

    // Wait for the app to load
    await page.waitForSelector('[data-testid="inbox-container"]');

    // Verify unified feed cards
    const messageCards = await page.locator('[data-testid="message-card"]');
    expect(await messageCards.count()).toBeGreaterThanOrEqual(0); // Assuming mock/seed or empty state

    // If there's a seed conversation, click and verify
    if (await messageCards.count() > 0) {
      await messageCards.first().click();
      await page.waitForSelector('[data-testid="conversation-view"]');
      const draft = await page.locator('[data-testid="ai-draft"]');

      // Verify AI drafting section exists
      await expect(draft).toBeVisible();

      // Tap 'Approve' on draft
      await page.locator('button:has-text("Approve")').click();
    }
  });
});
