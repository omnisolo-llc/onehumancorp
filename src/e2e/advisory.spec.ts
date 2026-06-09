import { test, expect } from './fixtures';
import { scoreWithAiJudge } from './ai-judge';

test.describe('Advisory Insights & Morning Briefing', () => {
  // Use a longer timeout for AI generation tests
  test.setTimeout(60000);

  test('should display the Morning Briefing card on dashboard and allow chat', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the briefing card to become visible
    const briefingCard = page.locator('#briefing-card');
    await expect(briefingCard).toBeVisible({ timeout: 15000 });

    // The content should load and not be the default "Loading your briefing..."
    const briefingContent = page.locator('#briefing-content');
    await expect(briefingContent).not.toHaveText('Loading your briefing...', { timeout: 30000 });

    const contentText = await briefingContent.textContent();
    expect(contentText?.length).toBeGreaterThan(10);

    // Open chat overlay
    const chatBtn = page.locator('#ohc-help-btn');
    await chatBtn.click();

    // Ensure chat overlay is visible
    const chatOverlay = page.locator('#ohc-help-chat-overlay');
    await expect(chatOverlay).toBeVisible();

    // Type a message in the chat
    const chatInput = page.locator('#ohc-help-input');
    await chatInput.fill('How many active orders do I have today?');

    // Send message
    const sendBtn = page.locator('#ohc-help-send');
    await sendBtn.click();

    // Wait for AI reply
    // The chat creates a new div.msg-ai with the answer
    // Wait for the thinking message to disappear
    await expect(page.locator('#loading-msg')).not.toBeVisible({ timeout: 30000 });

    // Grab the last AI message
    const aiMessages = page.locator('.msg-ai');
    const lastAiMessage = aiMessages.last();

    const replyText = await lastAiMessage.textContent();
    expect(replyText?.length).toBeGreaterThan(10);
    expect(replyText).not.toBe("I'm having trouble processing that right now.");
    expect(replyText).not.toBe("Error connecting to AI.");

    // Evaluate response quality with AI Judge
    const score = await scoreWithAiJudge(
      `User asked: 'How many active orders do I have today?'. Assistant answered: '${replyText}'. The test seeds database with 2 pending orders. Does the assistant answer mention 2 orders or indicate contextually relevant business stats?`,
      replyText || ''
    );
    expect(score).toBeGreaterThanOrEqual(8);
  });
});
