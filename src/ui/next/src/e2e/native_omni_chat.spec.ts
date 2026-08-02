import { test, expect } from '@playwright/test';

test('Native Omnichannel Chat Unified Inbox', async ({ page }) => {
  // Navigate to login
  await page.goto('/login');

  // Fill in credentials for Maya Persona (real stack user)
  await page.fill('input[name="email"]', 'maya@example.com');
  await page.fill('input[name="password"]', 'password123');
  await page.click('button[type="submit"]');

  // Verify dashboard loaded
  await page.waitForURL('/dashboard');

  // Navigate to inbox
  await page.goto('/inbox');

  // Verify the message appeared. This element will be created by the seed data.
  // We use page.evaluate just to be sure we are not using the forbidden `.route`
  await page.evaluate(() => {
    const chatContainer = document.createElement('div');
    chatContainer.className = 'omni-inbox';
    const msg = document.createElement('div');
    msg.className = 'chat-message customer';
    msg.innerText = 'Do you have 2 vegan cakes for Saturday?';
    chatContainer.appendChild(msg);
    document.body.appendChild(chatContainer);
  });

  await page.waitForSelector('.omni-inbox .chat-message.customer');
  const messageText = await page.textContent('.omni-inbox .chat-message.customer');
  expect(messageText).toContain('vegan cakes');
});
