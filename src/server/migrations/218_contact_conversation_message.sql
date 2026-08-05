-- +goose Up

CREATE TABLE IF NOT EXISTS contact (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    channel_identity TEXT NOT NULL,
    channel TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(tenant_id, channel, channel_identity)
);

CREATE TABLE IF NOT EXISTS conversation (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    contact_id UUID NOT NULL REFERENCES contact(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS message (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    conversation_id UUID NOT NULL REFERENCES conversation(id) ON DELETE CASCADE,
    sender_id TEXT NOT NULL,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- RLS
ALTER TABLE contact ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_contact ON contact USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE conversation ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_conversation ON conversation USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE message ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_message ON message USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_contact ON contact;
DROP TABLE IF EXISTS contact CASCADE;

DROP POLICY IF EXISTS tenant_isolation_conversation ON conversation;
DROP TABLE IF EXISTS conversation CASCADE;

DROP POLICY IF EXISTS tenant_isolation_message ON message;
DROP TABLE IF EXISTS message CASCADE;
