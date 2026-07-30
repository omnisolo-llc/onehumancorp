-- Omnichannel Chat Engine Migration
-- Addresses requirements for multi-tenant unified inbox, channels, conversations, contacts, and messages.

CREATE TABLE IF NOT EXISTS channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    channel_type TEXT NOT NULL,
    credentials JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE channels ENABLE ROW LEVEL SECURITY;
CREATE POLICY channels_tenant_isolation_policy ON channels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE POLICY channels_tenant_isolation_policy_check ON channels FOR ALL WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_id UUID NOT NULL REFERENCES channels(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY inboxes_tenant_isolation_policy ON inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE POLICY inboxes_tenant_isolation_policy_check ON inboxes FOR ALL WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS chat_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone_number TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE POLICY chat_contacts_tenant_isolation_policy_check ON chat_contacts FOR ALL WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    contact_id UUID NOT NULL REFERENCES chat_contacts(id),
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY conversations_tenant_isolation_policy ON conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE POLICY conversations_tenant_isolation_policy_check ON conversations FOR ALL WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);


CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    sender_id UUID, -- NULL if system/AI
    content TEXT NOT NULL,
    is_ai_draft BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY messages_tenant_isolation_policy ON messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE POLICY messages_tenant_isolation_policy_check ON messages FOR ALL WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
