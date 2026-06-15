import { test, expect } from './fixtures';
import { e2eTenantId, e2eUserId } from './fixtures'; // Depending on what's available, we'll try to just log in

test.describe('Growth Loop: Social Share', () => {
  test('User can share success page and hit tracking endpoint', async ({ page }) => {
    // Navigate to the success page using a logged-in context via fixtures
    await page.goto(`/success.html?tenant=${e2eTenantId}`);

    // Wait for the Social Share component to appear
    await page.locator('text=Claim your Launch Reward').waitFor();

    // Verify the share button is visible
    const shareBtn = page.locator('text=Share on X');
    await shareBtn.waitFor();

    // The handler uses window.open which creates a popup.
    const [popup, request] = await Promise.all([
      page.waitForEvent('popup'),
      page.waitForRequest(req => req.url().includes('/api/v1/growth/milestone/track-share') && req.method() === 'POST'),
      shareBtn.click(),
    ]);

    expect(popup.url()).toContain('twitter.com');
    await popup.close();

    // Verify the tracking API was called with the correct payload
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.platform).toBe('x');
    expect(postData.milestone_id).toBe('store_launch');
  });

  test('User can share milestone and hit tracking endpoint', async ({ page }) => {
    // Navigate to the milestone page directly after logged in
    await page.goto('/milestones');

    await page.locator('text=Your Achievements').waitFor();

    // We expect at least one milestone is unlocked or shown
    const firstUnlockedMilestone = page.locator('.milestone-item.reached').first();

    // Check if the share button appears. If a milestone is active, share container is visible.
    const shareContainer = page.locator('#share-container');
    const isVisible = await shareContainer.isVisible();
    if (!isVisible && await firstUnlockedMilestone.isVisible()) {
        await firstUnlockedMilestone.click();
    }

    // Once visible, wait for X button
    const shareBtn = page.locator('text=Share on X');
    await expect(shareBtn).toBeVisible({ timeout: 15000 });

    // Click the share button and wait for popup
    const [popup, request] = await Promise.all([
      page.waitForEvent('popup'),
      page.waitForRequest(req => req.url().includes('/api/v1/growth/milestone/track-share') && req.method() === 'POST'),
      shareBtn.click(),
    ]);

    expect(popup.url()).toContain('twitter.com');
    await popup.close();

    // Verify the tracking API was called with the correct payload
    const postData = JSON.parse(request.postData() || '{}');
    expect(postData.platform).toBe('x');
    // We don't strictly assert the exact milestone id here since it's dynamically populated by DB
    expect(postData.milestone_id).toBeTruthy();
  });
});
