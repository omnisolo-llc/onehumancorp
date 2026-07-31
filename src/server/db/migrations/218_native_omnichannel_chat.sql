-- +goose Up
-- Migration 218: Add native omnichannel chat tables

CREATE TABLE IF NOT EXISTS inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL, -- 'whatsapp' | 'web_widget'
    channel_id UUID,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone_number TEXT,
    custom_attributes JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES inboxes(id),
    contact_id UUID NOT NULL REFERENCES contacts(id),
    assignee_id UUID,
    status TEXT NOT NULL DEFAULT 'open', -- 'open' | 'resolved' | 'snoozed'
    last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    sender_id UUID,
    sender_type TEXT NOT NULL, -- 'contact' | 'agent' | 'system'
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_inboxes_tenant_id ON inboxes(tenant_id);
CREATE INDEX idx_contacts_tenant_id ON contacts(tenant_id);
CREATE INDEX idx_conversations_tenant_id_inbox_id ON conversations(tenant_id, inbox_id);
CREATE INDEX idx_conversations_tenant_id_contact_id ON conversations(tenant_id, contact_id);
CREATE INDEX idx_messages_tenant_id_conversation_id_created_at ON messages(tenant_id, conversation_id, created_at);

ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY inboxes_tenant_isolation ON inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY contacts_tenant_isolation ON contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY conversations_tenant_isolation ON conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY messages_tenant_isolation ON messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
