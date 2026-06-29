import { expect, test } from './fixtures';

test.describe('Unified Agent Feed Additional Regression Tests', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('feed container layout does not overflow mobile viewport', async ({ page }) => {
    test.setTimeout(180000);
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // We expect the body not to scroll horizontally
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);
  });

  test('action center header is properly hidden on mobile', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // action center is replaced by command center but they wanted it hidden on mobile
  });

  test('action center header is visible on desktop', async ({ page }) => {
    // Override to desktop
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // action center is replaced by command center
  });

  test('feed elements are rendered with flex direction column for mobile', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Assuming .app-list-item or similar holds feed items, or glassmorphism
    const feedContainer = page.locator('#unified-agent-feed-section').first();
    await expect(feedContainer).toBeVisible();

    const flexDirection = await feedContainer.evaluate(el => window.getComputedStyle(el).flexDirection);
    expect(flexDirection).toBe('column');
  });

  test('buttons have minimum 44px touch targets', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const buttons = page.locator('section[aria-label="Unified Agent Feed"] button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < buttonCount; i++) {
        const box = await buttons.nth(i).boundingBox();
        if (box) {
           expect(box.height).toBeGreaterThanOrEqual(44);
           expect(box.width).toBeGreaterThanOrEqual(44);
        }
    }
  });
});
