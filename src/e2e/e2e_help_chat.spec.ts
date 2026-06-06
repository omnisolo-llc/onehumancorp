import { test, expect } from '@playwright/test';

test.describe('Help Chat Interface', () => {
  test('User can open chat, send a message and receive backend response', async ({ page }) => {
    // Navigate to a valid page where chat button is visible
    await page.goto('/?test_chat=true');

    // Find and click the floating Ask anything button
    const askAnythingBtn = page.getByRole('button', { name: /open help chat/i });
    await expect(askAnythingBtn).toBeVisible();
    await askAnythingBtn.click();

    // Ensure chat window opens with the header
    const chatHeader = page.getByRole('heading', { name: /Ask AI Help/i });
    await expect(chatHeader).toBeVisible();

    // Verify initial greeting message
    await expect(page.getByText("Hi! I'm your AI Help Agent.")).toBeVisible();

    // Type a question
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('How do I add a product to my store?');

    // Submit the message
    const sendBtn = page.getByRole('button', { name: /send message/i });
    await sendBtn.click();

    // Wait for the backend reply (should use the fallback generic answer or specific from help articles if matched)
    await expect(page.getByText('How do I add a product to my store?')).toBeVisible();
    await expect(page.getByText(/Read the full article →/i)).toBeVisible({ timeout: 10000 });
  });
});
