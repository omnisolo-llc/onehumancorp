import { test, expect } from '@playwright/test';

test.describe('Mobile Unified Agent Feed @mobile', () => {
  // Use a simulated mobile viewport
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display Action Center and agent feed cards on mobile dashboard', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForTimeout(5000);

    // Check if the unified agent feed structure is present
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Verify layout and background constraints
    await expect(feedSection).toHaveClass(/max-w-full/);
    await expect(feedSection).toHaveClass(/dark:bg-slate-950/);

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
    expect(isAllCaughtUpVisible || isAgentCardVisible || isActionNeededCardVisible || (await page.getByText(/All caught up!\s*Your agents are currently monitoring the business/).isVisible())).toBeTruthy();

    if (isAgentCardVisible || isActionNeededCardVisible) {
       // Verify touch targets have minimum height requirements for mobile UX
       const approveButton = page.getByRole('button', { name: /Approve/i }).first();
       if (await approveButton.isVisible()) {
           await expect(approveButton).toHaveClass(/min-h-\[44px\]/);
           await expect(approveButton).toHaveClass(/min-w-\[44px\]/);
       }
    }
  });
});
