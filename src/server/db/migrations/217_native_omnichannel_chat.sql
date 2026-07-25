CREATE TABLE IF NOT EXISTS omni_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    channel_id UUID,
    channel_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_inbox_tenant_isolation_policy ON omni_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_channel_web_widget (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    website_url TEXT NOT NULL,
    widget_color TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_channel_web_widget ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_channel_web_widget_tenant_isolation_policy ON omni_channel_web_widget FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_contact (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone_number TEXT,
    identifier TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_contact_tenant_isolation_policy ON omni_contact FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_contact_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES omni_contact(id),
    inbox_id UUID NOT NULL REFERENCES omni_inbox(id),
    source_id TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_contact_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_contact_inbox_tenant_isolation_policy ON omni_contact_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omni_inbox(id),
    contact_inbox_id UUID NOT NULL REFERENCES omni_contact_inbox(id),
    assignee_id UUID,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_conversation_tenant_isolation_policy ON omni_conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omni_message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omni_conversation(id),
    contact_id UUID NOT NULL REFERENCES omni_contact(id),
    sender_type TEXT NOT NULL,
    sender_id UUID NOT NULL,
    content TEXT NOT NULL,
    message_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omni_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY omni_message_tenant_isolation_policy ON omni_message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
