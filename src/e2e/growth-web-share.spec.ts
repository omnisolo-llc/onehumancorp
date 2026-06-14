import { test, expect } from './fixtures';

test.describe('Growth Virality: Web Share API', () => {
  test('Dashboard Invite & Earn displays and triggers Share via Device if available', async ({ page }) => {
    // We mock navigator.share before navigating to the page
    await page.addInitScript(() => {
      // Mock navigator.share
      (window.navigator as any).share = async (data: any) => {
        (window as any).__lastSharedData = data;
        return Promise.resolve();
      };
    });

    await page.goto('/dashboard.html');

    const getInviteBtn = page.locator('#dashboard-invite-btn');
    await expect(getInviteBtn).toBeVisible();
    await getInviteBtn.click();

    const linkInput = page.locator('#dashboard-invite-link');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).not.toHaveValue('');

    const shareDeviceBtn = page.locator('#dashboard-share-device-btn');
    await expect(shareDeviceBtn).toBeVisible();
    await shareDeviceBtn.click();

    // Verify navigator.share was called with the correct data
    const lastSharedData = await page.evaluate(() => (window as any).__lastSharedData);
    expect(lastSharedData).toBeTruthy();
    expect(lastSharedData.title).toBe('Join me on OHC');
    expect(lastSharedData.url).toContain('http');
  });
});
