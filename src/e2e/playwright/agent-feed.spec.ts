import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile MVP)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays action cards and allows approval without horizontal scrolling and without mock data', async ({ page }) => {
    await page.goto('/feed');
    await page.waitForSelector('[data-testid="agent-feed"]');

    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);

    const simulateBtn = page.locator('[data-testid="simulate-ambassador-btn"]');
    await expect(simulateBtn).toBeVisible();
    await simulateBtn.click();

    await page.waitForSelector('[data-testid="agent-feed-card"]', { timeout: 10000 });
    const cards = page.locator('[data-testid="agent-feed-card"]');
    await expect(cards).toHaveCount(1, { timeout: 10000 });

    const buttons = cards.first().locator('button');
    const buttonCount = await buttons.count();
    for (let i = 0; i < buttonCount; i++) {
        const boundingBox = await buttons.nth(i).boundingBox();
        expect(boundingBox?.width).toBeGreaterThanOrEqual(44);
        expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
    }

    const sendDraftButton = page.locator('[data-testid="feed-approve-btn"]').first();
    await expect(sendDraftButton).toBeVisible();
    await sendDraftButton.click();

    // Verify success state by ensuring the card disappears or success message shows up
    await expect(cards).toHaveCount(0, { timeout: 10000 });
  });
});
