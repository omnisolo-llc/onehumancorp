import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // Wait for AI Reply
    // Since we are now using a real backend webhook, we poll the page for the incoming reply.
    // The webhook creates a new 'pending' inbox message with a draft_reply.
    const aiBadge = page.getByText('AI Replied').first();
    await expect(aiBadge).toBeVisible({ timeout: 15000 });

    // Verify reply content exists (we use a generic check since Minimax generates the actual text)
    await expect(page.locator('.mt-3.ml-4.bg-\\[\\#f9f5ff\\]')).toBeVisible();
  });
});
