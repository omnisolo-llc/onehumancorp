import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed (Mobile First 375px)', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('should display agent proposals and handle approval flow', async ({ page }) => {
    await page.goto('/dashboard');
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    const simulateBtn = page.getByTestId('simulate-quote-draft');
    await expect(simulateBtn).toBeVisible();
    await simulateBtn.click();

    const approveBtn = page.getByTestId('approve-proposal').first();
    await expect(approveBtn).toBeVisible({ timeout: 10000 });

    const box = await approveBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);

    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(hasHorizontalScroll).toBe(false);

    await approveBtn.click();
  });
});
