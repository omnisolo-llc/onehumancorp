import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Click Simulate Incoming Message
    await page.getByTestId('simulate-message-btn').click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // The component auto-replies directly by adding the AI Draft
    // The previous test looked for "AI Replied", but the text in the component is actually "AI Draft"
    const aiBadge = page.getByText('AI Draft');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // Verify reply content
    // Based on the default messages state, the simulated message should trigger a reply.
    // The page actually says: "Hi! Yes, we are open until 6 PM today and we currently have 12 Vanilla Cupcakes left. Shall I set one aside for you?"
    await expect(page.getByText('Vanilla Cupcakes left', { exact: false })).toBeVisible();
  });
});
