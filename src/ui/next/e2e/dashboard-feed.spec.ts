import { test, expect } from '@playwright/test';
import { memberPage } from '../../../e2e/fixtures';

test.describe('Dashboard Unified Agent Feed (Mobile MVP)', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  memberPage('should render the unified agent feed on the dashboard at 375px', async ({ page }) => {
    await page.goto('/dashboard');

    // Check that we're on the dashboard
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible();

    // Check that the Unified Agent Feed section is visible
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Check the width limit logic
    const box = await feedSection.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);

    // Verify touch targets have at least 44px min-height (button minimums)
    const approveButton = page.locator('[data-testid="approve-proposal"]').first();
    if (await approveButton.isVisible()) {
      const btnBox = await approveButton.boundingBox();
      expect(btnBox?.height).toBeGreaterThanOrEqual(44);
    }

    // Verify no static graphs or old dashboard elements
    await expect(page.locator('text=Operations Map')).not.toBeVisible();
    await expect(page.locator('text=Business Analytics')).not.toBeVisible();
  });
});
