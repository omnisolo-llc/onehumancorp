import { test, expect } from './fixtures';

test.describe('Unlock Pro Features Virality Widget', () => {
  test('Widget should be visible on dashboard and interactions should work', async ({ page }) => {
    // 1. Visit the dashboard (the fixture logs in and navigates)
    await page.goto('/dashboard');

    // 2. Locate the widget
    const widget = page.locator('[data-testid="unlock-pro-features-widget"]');
    await expect(widget).toBeVisible();

    // 3. Check for specific content
    await expect(widget.locator('text=Unlock Pro Features')).toBeVisible();

    // 4. Test the copy link button
    const copyButton = widget.locator('button', { hasText: /Copy Invite Link/i });
    if (await copyButton.isVisible()) {
        await copyButton.click();
        await expect(widget.locator('text=Copied Link!')).toBeVisible();
    } else {
        // If it's already unlocked (e.g. >= 3 invites), check for unlocked state
        await expect(widget.locator('text=Pro Features Unlocked!')).toBeVisible();
    }
  });
});