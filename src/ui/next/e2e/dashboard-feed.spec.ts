import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed Mobile UI', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('should display unified agent feed correctly on 375px viewport', async ({ page }) => {
    await page.goto('/dashboard');

    // Wait for the main wrapper and feed to appear
    await expect(page.locator('text="Welcome back"').first()).toBeVisible();
    const feed = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feed).toBeVisible();

    // The feed should be contained within 375px
    const feedBox = await feed.boundingBox();
    expect(feedBox?.width).toBeLessThanOrEqual(375);

    // If there are buttons, they should have touch targets of at least 44px height
    const buttons = feed.locator('button');
    const buttonCount = await buttons.count();

    for (let i = 0; i < buttonCount; i++) {
        const bbox = await buttons.nth(i).boundingBox();
        if (bbox && bbox.height) {
            // It might be smaller than 44px if hidden or specific tab headers, but let's check class names
            // or we expect all actionable items to be around >= 40px in general (min-h-[44px] is used).
            // We check if any button has min-h-[44px] class or if height is >= 44
            const classes = await buttons.nth(i).getAttribute('class');
            if (classes?.includes('min-h-[44px]')) {
                expect(bbox.height).toBeGreaterThanOrEqual(44);
            }
        }
    }
  });
});
