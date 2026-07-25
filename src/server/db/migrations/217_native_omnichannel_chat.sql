CREATE TABLE IF NOT EXISTS omnichannel_inbox (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_inbox_tenant_isolation_policy ON omnichannel_inbox FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_channel (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    channel_type TEXT NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_channel ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_channel_tenant_isolation_policy ON omnichannel_channel FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversation (
    id UUID PRIMARY KEY,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    contact_id UUID,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_conversation_tenant_isolation_policy ON omnichannel_conversation FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_message (
    id UUID PRIMARY KEY,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversation(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL,
    content TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    sender_id UUID,
    status TEXT NOT NULL DEFAULT 'sent',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE omnichannel_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY omnichannel_message_tenant_isolation_policy ON omnichannel_message FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
