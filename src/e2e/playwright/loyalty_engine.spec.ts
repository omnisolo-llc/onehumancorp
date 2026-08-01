import { test, expect } from '../fixtures';

test.describe('Loyalty Engine AI Integration', () => {
  test('Agent identifies eligible reward and proposes applying it', async ({ page }) => {
    // Go to the loyalty component test page or owner dashboard where this is surfaced
    await page.goto('/ui/loyalty-agent.html');

    // Assert the customer's point balance is visible
    const balanceCard = page.locator('.loyalty-balance-card');
    await expect(balanceCard).toBeVisible();

    // Look for the AI agent's proactive suggestion
    const suggestion = page.locator('.ai-agent-suggestion');
    await expect(suggestion).toBeVisible();
  });

  test('Mobile layout handles translucent cards and responsive grids', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/ui/loyalty-agent.html');

    // Check for horizontal scrolling (which breaks the mobile mandate)
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(bodyWidth).toBeLessThanOrEqual(windowWidth);

    // Verify touch target size on primary action button
    const applyBtn = page.locator('button#apply-reward');
    const box = await applyBtn.boundingBox();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });
});
