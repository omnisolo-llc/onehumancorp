import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed Mobile MVP', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('displays feed and ensures no horizontal scroll on mobile', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Make sure we wait for the page to load
    await page.waitForLoadState('networkidle');

    // The feed should be present and visible
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Ensure there is no horizontal scroll on the body
    const isScrollable = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(isScrollable).toBeFalsy();

    // Check if there are proposal action buttons with min 44x44
    // If the backend returns no proposals, this might be empty, but we can verify touch targets of the tab
    const proposalsTab = page.getByRole('button', { name: /Proposals \(\d+\)/ });
    const box = await proposalsTab.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });
});
