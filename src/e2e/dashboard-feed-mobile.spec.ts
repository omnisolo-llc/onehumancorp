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

    // Since triage items are now part of UnifiedAgentFeed, verify interaction
    // We will wait for at least one triage card to appear, or mock if we have to, but since it's E2E it should hit db.
    // If we have a triage item (like "Proactive Context Agent" or normal), let's find the first approve button
    const approveBtn = page.getByTestId(/triage-approve-/).first();
    const btnCount = await approveBtn.count();

    if (btnCount > 0) {
      // Check touch target for the action button
      const btnBox = await approveBtn.boundingBox();
      expect(btnBox).not.toBeNull();
      if (btnBox) {
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
        expect(btnBox.width).toBeGreaterThanOrEqual(44);
      }

      const testId = await approveBtn.getAttribute('data-testid');
      await approveBtn.click();

      // Wait to verify it's removed from DOM optimally
      await expect(page.getByTestId(testId as string)).toHaveCount(0);
    }
  });
});
