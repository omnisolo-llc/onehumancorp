CREATE TABLE IF NOT EXISTS inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name TEXT NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now())
);
ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS inboxes_isolation_policy ON inboxes;
CREATE POLICY inboxes_isolation_policy ON inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_inboxes_tenant_id ON inboxes(tenant_id);

CREATE TABLE IF NOT EXISTS channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    type INTEGER NOT NULL,
    name TEXT NOT NULL,
    provider_config_json JSONB,
    created_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now())
);
ALTER TABLE channels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS channels_isolation_policy ON channels;
CREATE POLICY channels_isolation_policy ON channels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_channels_tenant_id ON channels(tenant_id);

CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    identifier TEXT NOT NULL,
    email TEXT,
    phone_number TEXT,
    name TEXT,
    custom_attributes_json JSONB,
    created_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    UNIQUE(tenant_id, identifier)
);
ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS contacts_isolation_policy ON contacts;
CREATE POLICY contacts_isolation_policy ON contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_contacts_tenant_id ON contacts(tenant_id);

CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    channel_id UUID NOT NULL REFERENCES channels(id),
    status INTEGER NOT NULL DEFAULT 1,
    ai_handoff_state INTEGER NOT NULL DEFAULT 1,
    assigned_agent_id UUID,
    snoozed_until_unix BIGINT,
    contact_last_seen_at_unix BIGINT,
    agent_last_seen_at_unix BIGINT,
    created_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now())
);
ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS conversations_isolation_policy ON conversations;
CREATE POLICY conversations_isolation_policy ON conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_conversations_tenant_id ON conversations(tenant_id);

-- OHC has omni_messages but the new schema maps it to messages directly or expands it
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    sender_id UUID,
    message_type INTEGER NOT NULL,
    content_type INTEGER NOT NULL,
    original_content TEXT,
    translated_content TEXT,
    source_language TEXT,
    target_language TEXT,
    content_attributes_json JSONB,
    draft_reply TEXT,
    status TEXT,
    created_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at_unix BIGINT NOT NULL DEFAULT extract(epoch from now())
);
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS messages_isolation_policy ON messages;
CREATE POLICY messages_isolation_policy ON messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE INDEX IF NOT EXISTS idx_messages_tenant_id ON messages(tenant_id);
