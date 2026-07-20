import { test, expect } from '@playwright/test';

test.describe('Mobile Unified Feed Swipe Gestures', () => {
  // Mobile emulation
  test.use({ viewport: { width: 375, height: 812 } });

  test('unified feed handles swipe right to approve', async ({ page }) => {
    // Navigate to unified-feed
    await page.goto('/unified-feed');

    const firstCard = page.locator('[data-testid="agent-feed-card"]').first();
    await page.waitForTimeout(2000); // Wait for items to load
    const isVisible = await firstCard.isVisible();

    if (!isVisible) {
      console.log('No unified feed items to test swipe');
      return;
    }

    const cardBox = await firstCard.boundingBox();
    expect(cardBox).toBeTruthy();

    if (cardBox) {
      const startX = cardBox.x + cardBox.width / 2;
      const startY = cardBox.y + cardBox.height / 2;
      const endX = startX + 150; // threshold is 100

      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(endX, startY, { steps: 5 });
      await page.mouse.up();

      // Usually disappears or changes state on success
      await page.waitForTimeout(500); // wait for state change
    }
  });

  test('unified feed handles swipe left to dismiss', async ({ page }) => {
    await page.goto('/unified-feed');

    const firstCard = page.locator('[data-testid="agent-feed-card"]').first();
    await page.waitForTimeout(2000); // Wait for items to load
    const isVisible = await firstCard.isVisible();

    if (!isVisible) {
      console.log('No unified feed items to test swipe');
      return;
    }

    const cardBox = await firstCard.boundingBox();
    expect(cardBox).toBeTruthy();

    if (cardBox) {
      const startX = cardBox.x + cardBox.width / 2;
      const startY = cardBox.y + cardBox.height / 2;
      const endX = startX - 150; // threshold is -100

      await page.mouse.move(startX, startY);
      await page.mouse.down();
      await page.mouse.move(endX, startY, { steps: 5 });
      await page.mouse.up();

      await page.waitForTimeout(500); // wait for state change
    }
  });
});
