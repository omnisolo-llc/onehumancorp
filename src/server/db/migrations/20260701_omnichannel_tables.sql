CREATE TABLE IF NOT EXISTS omnichannel_contacts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    external_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE omnichannel_contacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_contacts_tenant_isolation_policy ON omnichannel_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_inboxes (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE omnichannel_inboxes ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_inboxes_tenant_isolation_policy ON omnichannel_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES omnichannel_contacts(id),
    inbox_id UUID NOT NULL REFERENCES omnichannel_inboxes(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE omnichannel_conversations ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_conversations_tenant_isolation_policy ON omnichannel_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversations(id),
    sender_type TEXT NOT NULL,
    sender_id UUID,
    content TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE omnichannel_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_messages_tenant_isolation_policy ON omnichannel_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_outbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES omnichannel_messages(id),
    channel_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE omnichannel_outbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_outbox_tenant_isolation_policy ON omnichannel_outbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
