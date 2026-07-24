import { test, expect } from './fixtures';

test.describe('Custom Omnichannel Chat System API', () => {
  test('creates inbox, channel adapters, contacts, conversations, and messages end-to-end', async ({ page }) => {
    const tenantId = 'e2e-tenant';
    const request = page.request;

    // 1. Create Inbox
    const createInboxRes = await request.post('/api/v1/custom-chat/inboxes', {
      data: {
        tenant_id: tenantId,
        name: 'Carlos Field Repair Chat',
      },
    });
    expect(createInboxRes.status()).toBe(201);
    const inbox = await createInboxRes.json();
    expect(inbox.id).toBeTruthy();
    expect(inbox.name).toBe('Carlos Field Repair Chat');

    // 2. Create Channel Adapter
    const createAdapterRes = await request.post('/api/v1/custom-chat/channel-adapters', {
      data: {
        tenant_id: tenantId,
        inbox_id: inbox.id,
        type_: 'whatsapp',
        credentials: '{"phoneNumber": "+123456789", "token": "whatsapp_token_abc"}',
      },
    });
    expect(createAdapterRes.status()).toBe(201);
    const adapter = await createAdapterRes.json();
    expect(adapter.id).toBeTruthy();
    expect(adapter.inbox_id).toBe(inbox.id);
    expect(adapter.type_).toBe('whatsapp');

    // 3. Create Contact
    const createContactRes = await request.post('/api/v1/custom-chat/contacts', {
      data: {
        tenant_id: tenantId,
        name: 'Fatima Preorder',
        identifier: 'fatima@cart.com',
      },
    });
    expect(createContactRes.status()).toBe(201);
    const contact = await createContactRes.json();
    expect(contact.id).toBeTruthy();
    expect(contact.name).toBe('Fatima Preorder');

    // 4. Create/Start Conversation
    const createConvRes = await request.post('/api/v1/custom-chat/conversations', {
      data: {
        tenant_id: tenantId,
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: 'open',
      },
    });
    expect(createConvRes.status()).toBe(201);
    const conversation = await createConvRes.json();
    expect(conversation.id).toBeTruthy();
    expect(conversation.status).toBe('open');

    // 5. Send/Create Message
    const createMsgRes = await request.post('/api/v1/custom-chat/messages', {
      data: {
        tenant_id: tenantId,
        conversation_id: conversation.id,
        content: 'Hi! Can I pre-order custom cookies today?',
        sender_type: 'customer',
      },
    });
    expect(createMsgRes.status()).toBe(201);
    const message = await createMsgRes.json();
    expect(message.id).toBeTruthy();
    expect(message.content).toBe('Hi! Can I pre-order custom cookies today?');

    // 6. List Conversations for Tenant
    const listConvsRes = await request.get(`/api/v1/custom-chat/conversations/${tenantId}`);
    expect(listConvsRes.status()).toBe(200);
    const convs = await listConvsRes.json();
    expect(Array.isArray(convs)).toBe(true);
    expect(convs.some((c: any) => c.id === conversation.id)).toBe(true);

    // 7. List Messages for Conversation
    const listMsgsRes = await request.get(`/api/v1/custom-chat/messages/${tenantId}/${conversation.id}`);
    expect(listMsgsRes.status()).toBe(200);
    const msgs = await listMsgsRes.json();
    expect(Array.isArray(msgs)).toBe(true);
    expect(msgs.some((m: any) => m.id === message.id)).toBe(true);
  });
});
