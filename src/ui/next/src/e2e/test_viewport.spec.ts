import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Viewport Constraint', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and not exceed 375px', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/triage');
    await expect(page.locator('h1', { hasText: 'Work Triage' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('.glassmorphism').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // We expect the body not to scroll horizontally
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    // Verify touch targets of the buttons in the feed if available
    const buttons = page.locator('div.glassmorphism button');
    const buttonCount = await buttons.count();
    for (let i = 0; i < buttonCount; i++) {
        const box = await buttons.nth(i).boundingBox();
        if (box) {
           expect(box.width).toBeGreaterThanOrEqual(44);
           expect(box.height).toBeGreaterThanOrEqual(44);
        }
    }
  });
});
