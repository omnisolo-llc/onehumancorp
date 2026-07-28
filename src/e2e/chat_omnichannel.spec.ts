import { test, expect } from '@playwright/test';
import { db } from './utils/db'; // Assuming there's a db util

test.describe('Omnichannel Chat E2E', () => {
  const tenantId = 'test-tenant';

  test.beforeEach(async () => {
    // We would normally seed the database here if needed,
    // but the issue says "no mock data in UI code". The E2E tests often use real paths
    // or seeds. I'll rely on the UI to create data if possible, but creating inboxes
    // via API is needed to test real-time messages.
  });

  test('Owner can view real-time chat messages and reply', async ({ page, request }) => {
    // 1. Log in as an owner (adjust path according to actual login system in OHC, usually we just hit the home page and wait for auth to settle or we use a fixture)
    // Here we'll just navigate to the chat page assuming the session is managed or we can hit the endpoint.
    // For many OHC E2E tests, navigating to '/' is enough if auth is mocked in e2e mode, or we just navigate to the page directly.
    await page.goto('/chat');
    await expect(page.locator('text=Omnichannel Support')).toBeVisible();

    // 2. Create an inbox and conversation via API (as if a webhook came in)
    const inboxRes = await request.post('/api/v1/chat/inboxes', {
        data: { name: "Web Widget" }
    });
    expect(inboxRes.ok()).toBeTruthy();
    const inbox = await inboxRes.json();

    const contactRes = await request.post('/api/v1/chat/contacts', {
        data: { name: "Test Contact" }
    });
    const contact = await contactRes.json();

    const convRes = await request.post('/api/v1/chat/conversations', {
        data: { inbox_id: inbox.id, contact_id: contact.id }
    });
    const conv = await convRes.json();

    // 3. Send a message to the conversation
    const msgRes = await request.post(`/api/v1/chat/conversations/${conv.id}/messages`, {
        data: { sender_type: "contact", content: "Hello, I need help with my cake order." }
    });
    expect(msgRes.ok()).toBeTruthy();

    // 4. Verify message appears in UI via WebSocket
    await expect(page.locator('text=Hello, I need help with my cake order.')).toBeVisible();

    // 5. Owner sends a reply
    await page.fill('input[placeholder="Type a message..."]', 'Of course, what is your order number?');
    await page.click('button:has-text("Send")');

    // 6. Verify owner reply appears
    await expect(page.locator('text=Of course, what is your order number?')).toBeVisible();

    // Check if the reply was saved to backend
    const msgsCheck = await request.get(`/api/v1/chat/conversations/${conv.id}/messages`);
    const msgsData = await msgsCheck.json();
    expect(msgsData.some((m: any) => m.content === 'Of course, what is your order number?')).toBeTruthy();
  });
});
