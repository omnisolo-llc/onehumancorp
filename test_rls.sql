BEGIN;

-- Setup the schema
\i src/server/migrations/1009_native_omnichannel_chat.sql

-- Create two tenants
DO $$
DECLARE
    tenant_1 UUID := '11111111-1111-1111-1111-111111111111';
    tenant_2 UUID := '22222222-2222-2222-2222-222222222222';
    inbox_id_1 UUID := gen_random_uuid();
    inbox_id_2 UUID := gen_random_uuid();
BEGIN
    -- Insert data bypassing RLS (as superuser)
    INSERT INTO chat_inboxes (id, tenant_id, name) VALUES (inbox_id_1, tenant_1, 'Tenant 1 Inbox');
    INSERT INTO chat_inboxes (id, tenant_id, name) VALUES (inbox_id_2, tenant_2, 'Tenant 2 Inbox');

    -- Test Tenant 1 context
    PERFORM set_config('app.current_tenant_id', tenant_1::text, true);
    ASSERT (SELECT COUNT(*) FROM chat_inboxes) = 1, 'Tenant 1 should only see 1 inbox';
    ASSERT (SELECT name FROM chat_inboxes LIMIT 1) = 'Tenant 1 Inbox', 'Tenant 1 should see their own inbox';

    -- Test Tenant 2 context
    PERFORM set_config('app.current_tenant_id', tenant_2::text, true);
    ASSERT (SELECT COUNT(*) FROM chat_inboxes) = 1, 'Tenant 2 should only see 1 inbox';
    ASSERT (SELECT name FROM chat_inboxes LIMIT 1) = 'Tenant 2 Inbox', 'Tenant 2 should see their own inbox';
END $$;

ROLLBACK;
