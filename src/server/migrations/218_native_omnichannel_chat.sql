CREATE TABLE omni_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_channel_connections (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omni_inboxes(id),
    channel_type TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_contact_identities (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES omni_contacts(id),
    channel_type TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omni_inboxes(id),
    contact_id UUID NOT NULL REFERENCES omni_contacts(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_participants (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omni_conversations(id),
    participant_type TEXT NOT NULL,
    participant_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omni_conversations(id),
    sender_type TEXT NOT NULL,
    sender_id UUID,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_attachments (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES omni_messages(id),
    file_url TEXT NOT NULL,
    file_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE omni_receipts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES omni_messages(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE omni_inboxes ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_channel_connections ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_contact_identities ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_participants ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_attachments ENABLE ROW LEVEL SECURITY;
ALTER TABLE omni_receipts ENABLE ROW LEVEL SECURITY;

CREATE POLICY omni_inboxes_tenant_isolation ON omni_inboxes FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_channel_connections_tenant_isolation ON omni_channel_connections FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_contacts_tenant_isolation ON omni_contacts FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_contact_identities_tenant_isolation ON omni_contact_identities FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_conversations_tenant_isolation ON omni_conversations FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_participants_tenant_isolation ON omni_participants FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_messages_tenant_isolation ON omni_messages FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_attachments_tenant_isolation ON omni_attachments FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY omni_receipts_tenant_isolation ON omni_receipts FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true));
