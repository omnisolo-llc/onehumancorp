import { test, expect } from './fixtures';

test.describe('Viral Success Milestone Share Card Loop', () => {
  test('should display the milestone card on the dashboard when a milestone is reached', async ({ page }) => {
    // Navigate to the Tauri dashboard
    await page.goto('/dashboard.html');

    // Wait for milestone container to be visible
    const milestoneCard = page.getByTestId('success-milestone-alert');
    await expect(milestoneCard).toBeVisible();

    // Verify Title
    await expect(page.locator('#milestone-title')).toHaveText('🎉 Milestone: 10th Order!');
    await expect(page.locator('#milestone-desc')).toContainText('You\'ve successfully processed your 10th order on OHC.');
    await expect(page.locator('#milestone-icon')).toHaveText('📈');

    // Verify Copy Link Button
    const copyBtn = page.getByTestId('milestone-share-btn');
    await expect(copyBtn).toBeVisible();
    await expect(copyBtn).toContainText('Copy Link');

    // Verify Share on X Button
    const xBtn = page.locator('#milestone-x-btn');
    await expect(xBtn).toBeVisible();
    await expect(xBtn).toContainText('Share on X');

    // Click Copy Link
    await copyBtn.click();
    await expect(copyBtn).toContainText('Copied Link!');
  });
});
