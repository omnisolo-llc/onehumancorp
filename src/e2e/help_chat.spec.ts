import { test, expect } from './fixtures';
import { aiJudge } from './ai-judge';

test('AI Help Chat integration without mock', async ({ page }) => {
  // 1. Navigate to the homepage where the ✨ button is available
  await page.goto('/');

  // 2. Open the Help Chat
  const askAnythingBtn = page.locator('button', { hasText: 'Ask anything' });
  await askAnythingBtn.click();

  // 3. Ensure the chat is open by verifying the Help Agent header
  await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();

  // 4. Type a question
  const input = page.locator('input[placeholder="Ask me anything..."]');
  await input.fill('What is your refund policy?');

  // 5. Send the question
  const sendBtn = page.locator('button[type="submit"]');
  await sendBtn.click();

  // 6. Verify user message appears in chat
  await expect(page.locator('text=What is your refund policy?')).toBeVisible();

  // 7. Wait for AI response to appear (we don't check exact text because it's generative)
  // We can just verify that a new agent message container appeared
  // The first agent message is "Hi! I'm your AI Help Agent..."
  // The second one should be the reply.
  const agentMessages = page.locator('.bg-white.border.border-gray-200.text-gray-800');

  // Wait for at least 2 agent messages (greeting + reply)
  await expect(agentMessages).toHaveCount(2, { timeout: 30000 });

  // 8. Optionally use AI judge to score the reply quality if it's dynamic
  const replyText = await agentMessages.nth(1).innerText();
  const evaluation = await aiJudge(replyText, 'A helpful and concise reply about a refund policy for a bakery.');
  expect(evaluation.score).toBeGreaterThan(9);
});
