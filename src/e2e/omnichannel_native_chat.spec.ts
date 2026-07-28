import { test, expect } from '@playwright/test';
import { setupMockUser } from './db_utils';
import { Pool } from 'pg';

test.describe('Omnichannel Native Chat System', () => {
  let tenantId: string;

  test.beforeEach(async ({ page, request }) => {
    tenantId = `t-omni-${Date.now()}`;
    await setupMockUser(request, tenantId, 'omni_owner@example.com', 'owner');

    // Seed real DB data instead of mock
    const pool = new Pool({
      connectionString: process.env.DATABASE_URL,
    });

    const inboxId = 'inbox-1';
    const contactId = 'contact-1';
    const conversationId = 'conv-1';

    await pool.query('INSERT INTO omnichannel_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)', [inboxId, tenantId, 'Main Inbox']);
    await pool.query('INSERT INTO omnichannel_contacts (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)', [contactId, tenantId, 'Test Customer', 'test@example.com']);
    await pool.query('INSERT INTO omnichannel_conversations (id, tenant_id, inbox_id, contact_id, channel) VALUES ($1, $2, $3, $4, $5)', [conversationId, tenantId, inboxId, contactId, 'instagram']);
    await pool.query('INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, content, message_type, sender_type) VALUES ($1, $2, $3, $4, $5, $6)', ['msg-1', tenantId, conversationId, 'Hello, is this available?', 'text', 'customer']);

    await pool.end();
  });

  test('user logs in, receives a message, and successfully sends a reply back', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/ui/dashboard.html');
    await page.evaluate(`localStorage.setItem('tenant_id', '${tenantId}')`);
    await page.reload();

    // Click on the Inbox link
    await page.locator('a[href="omnichannel-native.html"]').click();

    // Ensure we are on the native inbox view
    await expect(page).toHaveURL(/omnichannel-native\.html/);

    await expect(page.locator('h1').first()).toHaveText('Unified Inbox');

    // Click on the first conversation
    const conversationItem = page.locator('[data-testid="conversation-item"]').first();
    await expect(conversationItem).toBeVisible();
    await conversationItem.click();

    // We should now be in the conversation view
    await expect(page.locator('#chat-header-name')).toBeVisible();

    const messageBubblesCustomer = page.locator('.message-received');
    await expect(messageBubblesCustomer.last()).toHaveText('Hello, is this available?');

    // Now test sending a manual reply
    const chatInput = page.locator('#chat-input');
    await chatInput.fill('Yes, it is!');

    const sendBtn = page.locator('[data-testid="send-msg-btn"]');
    await sendBtn.click();

    // Verify it was appended
    const messageBubblesAgent = page.locator('.message-sent');
    await expect(messageBubblesAgent.last()).toHaveText('Yes, it is!');

    // Back to inbox
    await page.locator('button', { hasText: 'Back' }).click();
    await expect(page.locator('h1').first()).toHaveText('Unified Inbox');
  });
});
