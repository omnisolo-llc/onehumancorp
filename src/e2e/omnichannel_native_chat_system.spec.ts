import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { db } from './db_utils';

test.describe('Native Omnichannel Chat System', () => {
    test.beforeEach(async () => {
        // Seed conversation and messages via database to avoid mock data in UI
        await db.query(`
            DO $$
            DECLARE
                tenant_uuid uuid;
                inbox_uuid uuid := gen_random_uuid();
                customer_uuid uuid;
                conv_uuid uuid := gen_random_uuid();
                msg_uuid uuid := gen_random_uuid();
            BEGIN
                SELECT id INTO tenant_uuid FROM tenants WHERE slug = 'e2e-tenant' LIMIT 1;

                SELECT id INTO customer_uuid FROM customers WHERE tenant_id = tenant_uuid LIMIT 1;
                IF customer_uuid IS NULL THEN
                    customer_uuid := gen_random_uuid();
                    INSERT INTO customers (id, tenant_id, name, email) VALUES (customer_uuid, tenant_uuid, 'Sarah E2E', 'sarah@example.com');
                END IF;

                INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES (inbox_uuid, tenant_uuid, 'IG Inbox', 'instagram');

                INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (conv_uuid, tenant_uuid, inbox_uuid, customer_uuid, 'open');

                INSERT INTO chat_messages (id, tenant_id, conversation_id, inbox_id, content, message_type, status, sender_type)
                VALUES (msg_uuid, tenant_uuid, conv_uuid, inbox_uuid, 'Hello! Do you have vegan cake for Saturday?', 'incoming', 'sent', 'customer');

                INSERT INTO chat_messages (id, tenant_id, conversation_id, inbox_id, content, message_type, status, sender_type)
                VALUES (gen_random_uuid(), tenant_uuid, conv_uuid, inbox_uuid, 'Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?', 'outgoing', 'draft', 'ai');
            END $$;
        `);
    });

    test('Owner views conversation, sees AI draft, and approves it', async ({ page }) => {
        await page.goto('/dashboard/inbox');

        // Check if conversation exists
        await expect(page.locator('text=Sarah E2E')).toBeVisible();
        await page.click('text=Sarah E2E');

        // Verify incoming message
        await expect(page.locator('text=Hello! Do you have vegan cake for Saturday?')).toBeVisible();

        // Verify drafted response is visible
        await expect(page.locator('text=Yes we do! We have 3 left for this Saturday')).toBeVisible();

        // Click approve draft button (assuming the button text or aria-label)
        // Adjust the selector based on actual implementation. Using generic locator for now.
        const approveButton = page.locator('button', { hasText: 'Approve' });
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        // Optionally wait for some UI state change indicating it sent
        await expect(approveButton).not.toBeVisible();
    });
});
