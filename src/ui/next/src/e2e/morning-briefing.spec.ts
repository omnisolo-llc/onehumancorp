import { test, expect } from '../../../../e2e/fixtures';

test.describe('Morning Briefing & Insight Chat Dashboard Integration', () => {
  test('should display the morning briefing and allow chatting', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Check if the Morning Briefing card is visible
    const briefingText = page.locator('[data-testid="morning-briefing-text"]');
    await expect(briefingText).toBeVisible({ timeout: 15000 });

    // Verify it's not the loading state
    await expect(briefingText).not.toHaveText('Loading your Morning Briefing...', { timeout: 10000 });

    // Type a message in the Insight Chat
    const chatInput = page.locator('[data-testid="insight-chat-input"]');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('message');

    // Submit the message
    const submitBtn = page.locator('[data-testid="insight-chat-submit"]');
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // Ensure the message gets added to history and agent responds
    await expect(page.getByText('message')).toBeVisible();

    // Agent response checks (matches one of the expected mocked responses based on keyword)
    await expect(page.getByText(/You have no recent messages|Your latest messages are from:/i)).toBeVisible({ timeout: 10000 });
  });
});
