-- +goose Up

-- Fix social_post_proposals RLS policy
DROP POLICY IF EXISTS tenant_isolation_social_post_proposals ON social_post_proposals;
CREATE POLICY tenant_isolation_social_post_proposals ON social_post_proposals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Fix chat_inboxes RLS policy
DROP POLICY IF EXISTS chat_inboxes_tenant_isolation_policy ON chat_inboxes;
CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix chat_channels RLS policy
DROP POLICY IF EXISTS chat_channels_tenant_isolation_policy ON chat_channels;
CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix chat_contacts RLS policy
DROP POLICY IF EXISTS chat_contacts_tenant_isolation_policy ON chat_contacts;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix chat_conversations RLS policy
DROP POLICY IF EXISTS chat_conversations_tenant_isolation_policy ON chat_conversations;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix chat_messages RLS policy
DROP POLICY IF EXISTS chat_messages_tenant_isolation_policy ON chat_messages;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);


-- +goose Down
-- Intentionally blank
