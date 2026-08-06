import { test, expect } from '@playwright/test';
import { setupTestUser } from './setupTests';

test.describe('Native Rust Chat System', () => {
  let token: string;
  let tenantId: string;

  test.beforeAll(async ({ request }) => {
    const setup = await setupTestUser(request);
    token = setup.token;
    tenantId = setup.tenantId;

    // Call the DEV endpoint to create a test conversation
    const res = await request.post('/api/v1/chat-dev/dev/test-conversation', {
      headers: { Authorization: `Bearer ${token}` }
    });
    expect(res.ok()).toBeTruthy();
    const conv = await res.json();

    // Create a message from customer
    const msgRes = await request.post(`/api/v1/chat/conversations/${conv.id}/messages`, {
      headers: { Authorization: `Bearer ${token}` },
      data: { content: "Do you make vegan cakes?" }
    });
    expect(msgRes.ok()).toBeTruthy();

    // Create an AI draft
    const draftRes = await request.post(`/api/v1/chat/conversations/${conv.id}/drafts`, {
      headers: { Authorization: `Bearer ${token}` },
      data: { content: "Yes, we do! Here is our vegan menu..." }
    });
    expect(draftRes.ok()).toBeTruthy();
  });

  test('owner can view and reply to a conversation, and approve AI draft', async ({ page }) => {
    // Set token in localStorage and navigate to chat page
    await page.addInitScript((authToken) => {
      window.localStorage.setItem('token', authToken);
    }, token);

    await page.goto('/chat');

    // Wait for the conversation to appear in the sidebar
    const convItem = page.locator('button', { hasText: 'Customer' }).first();
    await expect(convItem).toBeVisible({ timeout: 10000 });
    await convItem.click();

    // Wait for messages to load
    const customerMsg = page.locator('text="Do you make vegan cakes?"');
    await expect(customerMsg).toBeVisible();

    const draftMsg = page.locator('text="Yes, we do! Here is our vegan menu..."');
    await expect(draftMsg).toBeVisible();

    // Approve the draft
    const approveBtn = page.locator('button:has-text("Approve & Send")');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The draft button should disappear (it is now a normal sent message)
    await expect(approveBtn).toBeHidden({ timeout: 5000 });

    // Send a manual reply
    const replyInput = page.locator('input[placeholder="Type a reply..."]');
    await replyInput.fill("Let me know if you want to place an order!");
    await page.locator('button:has-text("Send")').click();

    // The manual reply should appear
    const sentMsg = page.locator('text="Let me know if you want to place an order!"');
    await expect(sentMsg).toBeVisible();
  });
});
