import { test, expect } from '@playwright/test';

test.describe('Mobile Unified Agent Feed @mobile', () => {
  // Use a simulated mobile viewport
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display Action Center and agent feed cards on mobile dashboard', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check if the unified agent feed structure is present
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();

    // Verify tabs are visible
    await expect(page.getByRole('button', { name: /Proposals/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Activity Feed/i })).toBeVisible();

    // The component might show "Loading Agent Proposals..." initially
    const loadingMessage = page.getByText('Loading Agent Proposals...');
    if (await loadingMessage.isVisible()) {
      await expect(loadingMessage).toBeHidden({ timeout: 15000 });
    }

    // Either we see the "All caught up!" state or actual feed items
    const allCaughtUp = page.getByText('All caught up!');
    const agentCard = page.locator('.bg-\\[rgba\\(255\\,255\\,255\\,0\\.65\\)\\]').filter({ hasText: 'Approval' }).first();
    const actionNeededCard = page.locator('.bg-\\[rgba\\(255\\,255\\,255\\,0\\.65\\)\\]').filter({ hasText: 'Action Needed' }).first();

    const isAllCaughtUpVisible = await allCaughtUp.isVisible();
    const isAgentCardVisible = await agentCard.isVisible();
    const isActionNeededCardVisible = await actionNeededCard.isVisible();

    // At least one of these states should be present
    expect(isAllCaughtUpVisible || isAgentCardVisible || isActionNeededCardVisible).toBeTruthy();

    // Check for standard dashboard widgets which should also be visible
    await expect(page.getByText('Success Milestones')).toBeVisible();
  });
});
