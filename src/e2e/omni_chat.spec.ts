import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Omnichannel Chat Native Engine', () => {
    test('should be able to create an inbox, contact, conversation, and message', async ({ request }) => {
        // 1. Authenticate or use an API key/mock if applicable. Since this is an E2E test hitting Axum endpoints,
        // we can test the endpoints directly for the basic requirements using a mock tenant_id.
        const tenant_id = 'test-tenant-' + randomUUID();

        // 2. Create an Inbox
        const createInboxResponse = await request.post('/api/v1/omni-chat/inboxes', {
            data: {
                tenant_id: tenant_id,
                name: 'Main Support',
            }
        });
        expect(createInboxResponse.ok()).toBeTruthy();
        const inbox = await createInboxResponse.json();
        expect(inbox.id).toBeDefined();
        expect(inbox.name).toBe('Main Support');

        // 3. Create a Contact
        const createContactResponse = await request.post('/api/v1/omni-chat/contacts', {
            data: {
                tenant_id: tenant_id,
                name: 'Alice Smith',
                email: 'alice@example.com',
                phone: '+1234567890'
            }
        });
        expect(createContactResponse.ok()).toBeTruthy();
        const contact = await createContactResponse.json();
        expect(contact.id).toBeDefined();

        // 4. Create a Conversation
        const createConversationResponse = await request.post(`/api/v1/omni-chat/inboxes/${inbox.id}/conversations`, {
            data: {
                tenant_id: tenant_id,
                contact_id: contact.id,
                channel: 'web'
            }
        });
        expect(createConversationResponse.ok()).toBeTruthy();
        const conversation = await createConversationResponse.json();
        expect(conversation.id).toBeDefined();

        // 5. Create a Message
        const createMessageResponse = await request.post(`/api/v1/omni-chat/conversations/${conversation.id}/messages`, {
            data: {
                tenant_id: tenant_id,
                sender_type: 'customer',
                sender_id: contact.id,
                content: 'Hello, I need some help.'
            }
        });
        expect(createMessageResponse.ok()).toBeTruthy();
        const message = await createMessageResponse.json();
        expect(message.id).toBeDefined();
        expect(message.content).toBe('Hello, I need some help.');
    });
});
