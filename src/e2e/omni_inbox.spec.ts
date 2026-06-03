import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    // Navigate to the inbox page
    await page.goto('/inbox');

    // Wait for the button
    await expect(page.getByRole('button', { name: /Simulate Incoming Message/ })).toBeVisible({ timeout: 15000 });

    // Click Simulate Incoming Message
    await page.getByRole('button', { name: /Simulate Incoming Message/ }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?').first()).toBeVisible({ timeout: 15000 });

    // Mock AI reply delay wait
    await expect(page.getByText('AI Replied').first()).toBeVisible({ timeout: 15000 });

    // Verify AI response content
    await expect(page.getByText('Yes, we are open today from 9 AM to 5 PM!').first()).toBeVisible({ timeout: 15000 });
  });
});
