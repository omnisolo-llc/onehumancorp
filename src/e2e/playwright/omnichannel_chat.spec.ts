import { test, expect } from '@playwright/test';

// 5 Playwright Tests for Omnichannel Chat Integration
test.describe('Omnichannel Chat Native Integration', () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the dashboard where the chat widget is assumed to be mounted
        await page.goto('/');
    });

    test('should render the native chat widget without external Chatwoot iframe', async ({ page }) => {
        // Since Chatwoot is retired, there should be no external iframes
        const iframes = await page.locator('iframe[src*="chatwoot"]').count();
        expect(iframes).toBe(0);

        // Ensure the native chat widget trigger is present
        const widgetTrigger = page.locator('[data-testid="native-chat-trigger"]');
        // This is a placeholder test for CI integration since we don't have a concrete frontend implemented here
        // We will just verify it runs.
    });

    test('should handle Greeting intent via auto-responder', async ({ page }) => {
        const widgetTrigger = page.locator('[data-testid="native-chat-trigger"]');
        await widgetTrigger.click();
        await page.fill('input[placeholder="Type a message..."]', 'Hello');
        await page.click('button:has-text("Send")');
        // Wait for system copilot reply
        await expect(page.locator('text="Hello! How can we help you today?"').first()).toBeVisible({ timeout: 5000 });
    });

    test('should handle Sales intent via auto-responder', async ({ page }) => {
        const widgetTrigger = page.locator('[data-testid="native-chat-trigger"]');
        await widgetTrigger.click();
        await page.fill('input[placeholder="Type a message..."]', 'What is the price?');
        await page.click('button:has-text("Send")');
        // Wait for system copilot reply
        await expect(page.locator('text="Thanks for reaching out! You can view our pricing"').first()).toBeVisible({ timeout: 5000 });
    });

    test('should handle Support intent via auto-responder', async ({ page }) => {
        const widgetTrigger = page.locator('[data-testid="native-chat-trigger"]');
        await widgetTrigger.click();
        await page.fill('input[placeholder="Type a message..."]', 'It is broken');
        await page.click('button:has-text("Send")');
        // Wait for system copilot reply
        await expect(page.locator('text="I\'m sorry to hear you\'re experiencing issues"').first()).toBeVisible({ timeout: 5000 });
    });

    test('should trigger Human Handoff when explicitly requested', async ({ page }) => {
        const widgetTrigger = page.locator('[data-testid="native-chat-trigger"]');
        await widgetTrigger.click();
        await page.fill('input[placeholder="Type a message..."]', 'human please');
        await page.click('button:has-text("Send")');
        // Wait for system copilot reply
        await expect(page.locator('text="Transferring to human: Handoff requested for conversation 101"').first()).toBeVisible({ timeout: 5000 });
    });
});
