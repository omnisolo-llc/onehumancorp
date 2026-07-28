-- Native Rust Omnichannel Chat Schema
-- Based on the research report for OHC legacy replacement

CREATE TABLE inboxes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    enable_auto_assignment BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_inboxes_tenant_id ON inboxes(tenant_id);

CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    provider_type VARCHAR(50) NOT NULL,
    credentials JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_channels_tenant_id ON channels(tenant_id);
CREATE INDEX idx_channels_inbox_id ON channels(inbox_id);

CREATE TABLE contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone_number VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_contacts_tenant_id ON contacts(tenant_id);
CREATE INDEX idx_contacts_email ON contacts(tenant_id, email);
CREATE INDEX idx_contacts_phone ON contacts(tenant_id, phone_number);

CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES contacts(id) ON DELETE CASCADE,
    status VARCHAR(50) NOT NULL DEFAULT 'open',
    snoozed_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_conversations_tenant_id ON conversations(tenant_id);
CREATE INDEX idx_conversations_inbox_id ON conversations(inbox_id);
CREATE INDEX idx_conversations_contact_id ON conversations(contact_id);
CREATE INDEX idx_conversations_status ON conversations(tenant_id, status);

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id UUID REFERENCES contacts(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    message_type VARCHAR(50) NOT NULL,
    external_source_ids JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_messages_tenant_id ON messages(tenant_id);
CREATE INDEX idx_messages_conversation_id ON messages(conversation_id);
CREATE INDEX idx_messages_created_at ON messages(conversation_id, created_at);

-- Row Level Security Setup
ALTER TABLE inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY inboxes_tenant_isolation ON inboxes
    USING (tenant_id = nullif(current_setting('app.current_tenant_id', true), '')::uuid);

ALTER TABLE channels ENABLE ROW LEVEL SECURITY;
CREATE POLICY channels_tenant_isolation ON channels
    USING (tenant_id = nullif(current_setting('app.current_tenant_id', true), '')::uuid);

ALTER TABLE contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY contacts_tenant_isolation ON contacts
    USING (tenant_id = nullif(current_setting('app.current_tenant_id', true), '')::uuid);

ALTER TABLE conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY conversations_tenant_isolation ON conversations
    USING (tenant_id = nullif(current_setting('app.current_tenant_id', true), '')::uuid);

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY messages_tenant_isolation ON messages
    USING (tenant_id = nullif(current_setting('app.current_tenant_id', true), '')::uuid);
