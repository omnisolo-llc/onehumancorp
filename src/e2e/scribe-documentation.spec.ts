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

    test('Help Widget API fetches tooltips successfully', async ({ request }) => {
        // Test backend endpoint directly
        const response = await request.get('/api/v1/tooltips');
        expect(response.ok()).toBeTruthy();

        const data = await response.json();
        expect(data['voice-assistant-tooltip']).toBe('Hold to speak a command to your AI Assistant.');
        expect(data['rate-limit-close-tooltip']).toBe('Dismiss this warning.');
    });

});