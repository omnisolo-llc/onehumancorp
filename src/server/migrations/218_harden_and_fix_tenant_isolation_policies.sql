-- +goose Up
-- Migration 218: Harden and fix tenant isolation policies that mistakenly referenced non-existent app.current_tenant_id

-- 1. Fix social_post_proposals
DROP POLICY IF EXISTS tenant_isolation_social_post_proposals ON social_post_proposals;
CREATE POLICY tenant_isolation_social_post_proposals ON social_post_proposals
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 2. Fix availability_schedules
DROP POLICY IF EXISTS availability_schedules_tenant_isolation ON availability_schedules;
CREATE POLICY availability_schedules_tenant_isolation ON availability_schedules
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 3. Fix calendar_integrations
DROP POLICY IF EXISTS calendar_integrations_tenant_isolation ON calendar_integrations;
CREATE POLICY calendar_integrations_tenant_isolation ON calendar_integrations
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 4. Fix chat_inboxes
DROP POLICY IF EXISTS chat_inboxes_tenant_isolation_policy ON chat_inboxes;
CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 5. Fix chat_channels
DROP POLICY IF EXISTS chat_channels_tenant_isolation_policy ON chat_channels;
CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 6. Fix chat_contacts
DROP POLICY IF EXISTS chat_contacts_tenant_isolation_policy ON chat_contacts;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 7. Fix chat_conversations
DROP POLICY IF EXISTS chat_conversations_tenant_isolation_policy ON chat_conversations;
CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 8. Fix chat_messages
DROP POLICY IF EXISTS chat_messages_tenant_isolation_policy ON chat_messages;
CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Ensure all these tables explicitly have RLS enabled and forced
ALTER TABLE social_post_proposals ENABLE ROW LEVEL SECURITY;
ALTER TABLE social_post_proposals FORCE ROW LEVEL SECURITY;

ALTER TABLE availability_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE availability_schedules FORCE ROW LEVEL SECURITY;

ALTER TABLE calendar_integrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE calendar_integrations FORCE ROW LEVEL SECURITY;

ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_inboxes FORCE ROW LEVEL SECURITY;

ALTER TABLE chat_channels ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_channels FORCE ROW LEVEL SECURITY;

ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_contacts FORCE ROW LEVEL SECURITY;

ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_conversations FORCE ROW LEVEL SECURITY;

ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE chat_messages FORCE ROW LEVEL SECURITY;

-- +goose Down
-- Revert RLS policy hardening is discouraged for security reasons.
