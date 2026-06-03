import { test, expect } from './fixtures';

test.describe('Growth Loop: Milestone Viral Share', () => {
  test('User can share milestone and unlock reward', async ({ page }) => {
    await page.goto('/dashboard');

    // Wait for the Milestone Growth Loop component to appear
    await page.locator('text=Milestone Unlocked!').first().waitFor();

    // Verify the share button is visible
    const shareBtn = page.locator('text=Share & Claim Reward');
    await expect(shareBtn).toBeVisible();

    // Click share and verify success message
    await shareBtn.click();
    await expect(page.locator('text=Reward Claimed!')).toBeVisible();
  });
});
