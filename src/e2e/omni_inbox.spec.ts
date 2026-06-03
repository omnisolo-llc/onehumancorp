import { test, expect } from './fixtures';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('simulates incoming message and auto-replies correctly', async ({ page }) => {
    await page.goto('/inbox');

    // Wait for the initial real fetch to complete and empty state to be set so our simulated message isn't erased
    await page.waitForResponse(response => response.url().includes('/api/v1/inbox'));
    // Wait a little extra to let react effect apply
    await page.waitForTimeout(500);

    // Click Simulate Incoming Message
    // In our new test page it only says the emoji or title, so use title to be safe
    await page.locator('button[title="Simulate Incoming Message"]').click();

    // Verify user message is added
    await expect(page.getByText('Are you open today?')).toBeVisible();

    // Wait for AI Reply
    // The previous text 'AI Replied' was changed to 'AI Draft'
    const aiBadge = page.getByText('High Confidence');
    await expect(aiBadge).toBeVisible({ timeout: 10000 });

    // Verify reply content
    await expect(page.getByText(/Hi! Yes, we are open until 6 PM today/)).toBeVisible();
  });
});
