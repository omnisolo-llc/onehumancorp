import { test, expect } from './fixtures';

test.describe('Documentation UI Components', () => {

    test('Help Chat opens properly', async ({ page }) => {
        await page.goto('/api/ui/help.html');

        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeVisible();

        await chatButton.click();

        const chatOverlay = page.locator('#ohc-help-chat-overlay');
        await expect(chatOverlay).toBeVisible({ timeout: 5000 });
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
