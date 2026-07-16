import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Viewport Constraint', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly and not exceed 375px', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    // We expect the body not to scroll horizontally
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);
  });
});
