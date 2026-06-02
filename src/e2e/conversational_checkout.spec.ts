import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Engine', () => {
  test('Sales AI generates conversational checkout link from inbox intent', async ({ page }) => {
    // 1. Navigate to Unified Inbox
    await page.goto('/inbox');

    // 2. Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // 3. Instead of normal flow, we'll manually inject our test message for a booking
    const inputField = page.getByPlaceholder('Type a message...');
    if (await inputField.isVisible()) {
        await inputField.fill('I want to book Tuesday for a custom cake. What is the deposit?');
        await inputField.press('Enter');
    }

    // 4. Verify Sales AI detects intent and generates checkout bubble
    // We expect the AI to eventually generate a checkout link
    await expect(page.locator('text=/checkout\\.stripe\\.com|mercadopago\\.com/')).toBeVisible({ timeout: 15000 });
  });
});
