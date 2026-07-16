import { test, expect } from './fixtures';

test.describe('Mobile Shell Collision', () => {
  test.use({ viewport: { width: 375, height: 844 } });

  test('VoiceAssistant on mobile respects floating design and stays within app bounds', async ({ page }) => {
    // Navigate to a page that uses the AppShell.
    await page.goto('/dashboard');

    // Make sure we wait for it to load.
    await expect(page.locator('.app-sidebar, .app-topbar').first()).toBeVisible();

    // Check voice trigger element
    const voiceTrigger = page.locator('[data-voice-assistant-surface="trigger"]');
    await expect(voiceTrigger).toBeVisible();

    // Ensure there is no horizontal scroll
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(bodyWidth).toBeLessThanOrEqual(windowWidth + 1);
  });
});
