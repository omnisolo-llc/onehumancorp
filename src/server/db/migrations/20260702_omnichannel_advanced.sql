CREATE TABLE IF NOT EXISTS omnichannel_contact (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_contact_tenant_isolation_policy ON omnichannel_contact FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_inbox_tenant_isolation_policy ON omnichannel_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id),
    contact_id UUID REFERENCES omnichannel_contact(id),
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_conversation_tenant_isolation_policy ON omnichannel_conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversation(id),
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_message_tenant_isolation_policy ON omnichannel_message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
