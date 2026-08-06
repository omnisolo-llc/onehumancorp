import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';

test.describe('Custom Rust Omnichannel Chat System', () => {
  const tenantId = 'e2e-chat-inbox-tenant';
  const inboxId = 'd82c6d4e-b5c9-4b68-b76b-9d41b6cb4f2a';
  const contactId = 'c02b28cf-2c80-482a-a9a7-938b812f84b6';
  const convId = '5ab0c6b1-0941-4cf1-8c44-3d1f112e8b2f';

  test.use({ viewport: { width: 375, height: 667 } }); // Mobile UI

  test.beforeAll(async () => {
    // Seed initial data for chat_inboxes, chat_contacts, chat_conversations, chat_messages
    await e2eDbQuery(`
      INSERT INTO tenants (id, name, tier) VALUES ('${tenantId}', 'E2E Bakery', 'free') ON CONFLICT DO NOTHING;
      INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ('${inboxId}', '${tenantId}', 'Instagram DM') ON CONFLICT DO NOTHING;
      INSERT INTO chat_contacts (id, tenant_id, name, email) VALUES ('${contactId}', '${tenantId}', 'Customer A', 'customer@example.com') ON CONFLICT DO NOTHING;
      INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ('${convId}', '${tenantId}', '${inboxId}', '${contactId}', 'open') ON CONFLICT DO NOTHING;
      INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content) VALUES ('d82c6d4e-b5c9-4b68-b76b-9d41b6cb4f2b', '${tenantId}', '${convId}', 'customer', 'Do you have vegan chocolate cake available this weekend?') ON CONFLICT DO NOTHING;
      INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content) VALUES ('d82c6d4e-b5c9-4b68-b76b-9d41b6cb4f2c', '${tenantId}', '${convId}', 'agent', 'Yes, we do! Would you like me to hold one for you?') ON CONFLICT DO NOTHING;
    `);
  });

  test('Maya (Baker) can open the Unified Inbox on mobile, see an Instagram DM conversation, and reply', async ({ page, request }) => {
    // Navigate to Chat Inbox
    await page.goto('/login');
    await page.evaluate((t) => { localStorage.setItem('tenant_id', t); localStorage.setItem('tenant', t); }, tenantId);

    await page.goto('/chat_inbox');

    // Should see the conversation
    const convCard = page.getByTestId(`conversation-${convId}`);
    await expect(convCard).toBeVisible();

    // Select conversation
    await convCard.click();

    // Should see messages
    await expect(page.getByText('Do you have vegan chocolate cake available this weekend?')).toBeVisible();
    await expect(page.getByText('Yes, we do! Would you like me to hold one for you?')).toBeVisible();

    // Send a new reply
    await page.getByTestId('chat-input').fill('We need a deposit to hold it.');
    await page.getByTestId('chat-send').click();

    // Assert that the new message appears
    await expect(page.getByText('We need a deposit to hold it.')).toBeVisible();
  });
});
