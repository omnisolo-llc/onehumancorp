import { test, expect } from '@playwright/test';

test.describe('Documentation UI Components', () => {

    test('Voice Assistant tooltip renders on hover', async ({ page }) => {
        await page.goto('/dashboard');

        // Ensure Voice Assistant button exists
        const voiceButton = page.locator('button[aria-label="Voice Assistant"]');
        await expect(voiceButton).toBeVisible();

        // Hover to display tooltip
        await voiceButton.hover();

        // Wait for the tooltip text
        const tooltipText = page.locator('div', { hasText: 'Hold to speak a command to your AI Assistant.' }).last();
        await expect(tooltipText).toBeVisible({ timeout: 5000 });
    });

    test('Voice Assistant tooltip hides on mouse leave', async ({ page }) => {
        await page.goto('/dashboard');
        const voiceButton = page.locator('button[aria-label="Voice Assistant"]');
        await expect(voiceButton).toBeVisible();

        await voiceButton.hover();
        await expect(page.locator('div', { hasText: 'Hold to speak a command to your AI Assistant.' }).last()).toBeVisible({ timeout: 5000 });

        await page.mouse.move(0, 0); // move mouse away
        await expect(page.locator('div', { hasText: 'Hold to speak a command to your AI Assistant.' }).last()).not.toBeVisible();
    });

    test('Help Widget API fetches tooltips successfully', async ({ request }) => {
        // Test backend endpoint directly
        const response = await request.get('/api/tooltips');
        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data['voice-assistant-tooltip']).toBe('Hold to speak a command to your AI Assistant.');
        expect(data['rate-limit-close-tooltip']).toBe('Dismiss this warning.');
    });

});
test.describe('Tooltip mobile simulation', () => {
  test.use({ viewport: { width: 375, height: 812 }, hasTouch: true });

  test('Voice Assistant tooltip displays on long press', async ({ page }) => {
    await page.goto('/dashboard');
    const voiceButton = page.locator('button[aria-label="Voice Assistant"]');
    await expect(voiceButton).toBeVisible();

    // Trigger a long press by simulating a touch event and holding it
    await voiceButton.evaluate((node) => {
      node.dispatchEvent(new Event('touchstart', { bubbles: true }));
    });

    // Wait for the long press timeout to trigger the tooltip display
    await page.waitForTimeout(600);

    const tooltipText = page.locator('div', { hasText: 'Hold to speak a command to your AI Assistant.' }).last();
    await expect(tooltipText).toBeVisible({ timeout: 5000 });

    // Trigger touch end
    await voiceButton.evaluate((node) => {
      node.dispatchEvent(new Event('touchend', { bubbles: true }));
    });
  });
});
