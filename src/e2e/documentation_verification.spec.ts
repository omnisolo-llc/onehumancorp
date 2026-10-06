import { test, expect } from './fixtures';

test.describe('Documentation UI Verification', () => {
  test('Help portal videos render', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/v1/ui/help.html');

    // Make sure the title renders properly
    await expect(page.locator('h1')).toContainText('In-App Help Center');

    // Open floating widget
    const helpBtn = page.locator('#ohc-floating-help-btn').first();
    await helpBtn.waitFor({ state: 'visible' });
    await helpBtn.click();

    // Open videos tab
    const videosTab = page.locator('[data-target="tab-videos"]');
    await videosTab.waitFor({ state: 'visible' });
    await videosTab.click();

    const response = await page.request.get('/api/v1/videos');
    expect(response.ok()).toBe(true);
    const videos = await response.json() as Array<{ title: string; video_url: string }>;
    expect(videos.length).toBeGreaterThan(0);
    const videoList = page.locator('#ohc-floating-help-widget #video-list');
    const video = videoList.getByRole('button', { name: new RegExp(videos[0].title.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')) });
    await expect(video).toBeVisible();
    await video.click();
    await expect(page.locator('#video-modal')).toBeVisible();
    await expect(page.locator('#video-player')).toHaveAttribute('src', videos[0].video_url);
    await page.getByRole('button', { name: 'Close video', exact: true }).click();
    await expect(page.locator('#video-modal')).toBeHidden();
  });

  test('Dashboard Walkthrough triggers', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/api/v1/ui/dashboard.html');

    // Click the Walkthrough button
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await walkthroughBtn.waitFor({ state: 'visible' });
    await walkthroughBtn.click();

    // Wait for the walkthrough bubble to appear and verify text content
    const walkthroughBubble = page.locator('#walkthrough-bubble');
    await walkthroughBubble.waitFor({ state: 'visible' });
    await expect(walkthroughBubble).toBeVisible();
    await expect(walkthroughBubble).toContainText('Business Analytics');

    // Close the walkthrough
    const closeBtn = walkthroughBubble.getByRole('button', { name: 'Close walkthrough', exact: true });
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();
    await expect(walkthroughBubble).toBeHidden();
  });
});
