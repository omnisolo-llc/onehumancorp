CREATE TABLE IF NOT EXISTS omnichannel_inbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL
);

ALTER TABLE omnichannel_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_inbox USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_channel (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    channel_type TEXT NOT NULL,
    webhook_url TEXT
);

ALTER TABLE omnichannel_channel ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_channel USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_contact (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    email TEXT,
    phone_number TEXT
);

ALTER TABLE omnichannel_contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_contact USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_contact_inbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES omnichannel_contact(id) ON DELETE CASCADE,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    source_id TEXT NOT NULL
);

ALTER TABLE omnichannel_contact_inbox ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_contact_inbox USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_conversation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    inbox_id UUID NOT NULL REFERENCES omnichannel_inbox(id) ON DELETE CASCADE,
    contact_id UUID NOT NULL REFERENCES omnichannel_contact(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'open'
);

ALTER TABLE omnichannel_conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_conversation USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE TABLE IF NOT EXISTS omnichannel_message (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES omnichannel_conversation(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type TEXT NOT NULL
);

ALTER TABLE omnichannel_message ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation ON omnichannel_message USING (tenant_id = current_setting('app.current_tenant_id')::uuid);
