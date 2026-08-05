CREATE TABLE IF NOT EXISTS chat_webhook_ingress (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    payload JSONB NOT NULL,
    processed BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_webhook_ingress ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_webhook_ingress_tenant_isolation_policy ON chat_webhook_ingress FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS chat_outbox_messages (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    message_id UUID NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE chat_outbox_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY chat_outbox_messages_tenant_isolation_policy ON chat_outbox_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
