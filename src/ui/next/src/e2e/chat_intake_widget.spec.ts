import { test, expect } from '@playwright/test';

test.describe('Agentic Chat Intake Widget', () => {
  test('Completes an intake session via conversational UI', async ({ page }) => {
    // Navigate to the chat intake widget route. Assuming it's exposed at /chat-intake-widget
    await page.goto('/chat-intake-widget?tenant=my-business');

    // Verify initial message from the agent
    await expect(page.locator('text="What do you need help with?"')).toBeVisible({ timeout: 10000 });

    // Send a message
    await page.fill('textarea[placeholder="Type your message..."]', 'I need a cake for my wedding');
    await page.click('button:has(svg)');

    // Verify response asking for more details
    await expect(page.locator('text="What flavor and dietary restrictions do you have?"')).toBeVisible({ timeout: 10000 });

    // Send second message fulfilling the condition
    await page.fill('textarea[placeholder="Type your message..."]', 'Chocolate, vegan');
    await page.click('button:has(svg)');

    // Verify session completes
    await expect(page.locator('text="Got it. We will send you a quote shortly."')).toBeVisible({ timeout: 10000 });

    // Verify textarea is disabled and placeholder changed
    await expect(page.locator('textarea[placeholder="Session completed"]')).toBeDisabled();
  });
});
