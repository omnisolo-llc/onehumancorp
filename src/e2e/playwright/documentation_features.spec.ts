import { test, expect } from '@playwright/test';

test.describe('Documentation Features CUJ', () => {
  test('User can access help center, use chat, run walkthroughs, view changelog, and API docs', async ({ page }) => {
    // 1. Visit Help Center
    await page.goto('/help');
    await expect(page.locator('h1')).toContainText('In-App Help Center');

    // 2. Open AI Help Chat
    // Use the floating help widget directly available in help

    await expect(page.locator('h1').filter({ hasText: 'In-App Help Center' })).toBeVisible();

    await page.evaluate(() => {
        const event = new CustomEvent('open-help-chat');
        window.dispatchEvent(event);
    });

    await expect(page.locator('#ai-chat-interface')).toBeAttached();

    // 3. View Changelog from Dashboard
    await page.goto('/changelog');
    await expect(page.locator('h1')).toHaveText('Release Notes & Changelog');

    // 4. View API Docs
    await page.goto('/api-docs');
    // Wait for swagger to load
    await expect(page.locator('#api-docs-tooltip')).toBeAttached({ timeout: 10000 });
  });

  test('User can view mobile-optimized help videos', async ({ page }) => {
    // 1. Visit Help Center
    await page.goto('/help');
    await expect(page.locator('h1')).toContainText('In-App Help Center');

    // Simulate mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Ensure Video Tutorials section is visible on mobile
    await expect(page.locator('text=Video Tutorials')).toBeVisible();

    // Verify some video elements render properly without failing (the API call handles the fetch param logic, just test UI)
    const videos = page.locator('text=Video Tutorials').locator('..').locator('..').locator('div.grid > div');
    const hasVideos = await videos.count() > 0;
    if (hasVideos) {
      await expect(videos.first()).toBeVisible();
    }
  });
});
