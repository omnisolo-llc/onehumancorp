import { test, expect } from '@playwright/test';

test('Dashboard Send Message Panel', async ({ page }) => {
  // Navigate to the dashboard page
  await page.goto('http://localhost:3000/dashboard');

  // Verify the "Send Message" section exists
  const heading = page.locator('h2', { hasText: 'Send Message' });
  await expect(heading).toBeVisible();

  // Find the input field and button
  const input = page.locator('input[placeholder="Content"]');
  const sendButton = page.locator('button', { hasText: 'Send Message' });

  // Type a message
  const testMessage = "Hello, Team! Are we ready to launch?";
  await input.fill(testMessage);

  // Verify the button is enabled when input is not empty
  await expect(sendButton).toBeEnabled();

  // Send the message
  await sendButton.click();

  // Verify the user message appears in the transcript area
  const userMessageBubble = page.locator('div', { hasText: testMessage }).last();
  await expect(userMessageBubble).toBeVisible();

  // Verify the system message appears in the transcript area shortly after
  const systemMessageBubble = page.locator('div', { hasText: "I've drafted an action for your approval." }).last();
  await expect(systemMessageBubble).toBeVisible({ timeout: 5000 });
});
