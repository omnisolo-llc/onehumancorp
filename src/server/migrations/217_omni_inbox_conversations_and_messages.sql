CREATE TABLE IF NOT EXISTS omni_conversations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_conversations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_conversations ON omni_conversations;
CREATE POLICY tenant_isolation_omni_conversations ON omni_conversations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS omni_conversation_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL REFERENCES omni_conversations(id) ON DELETE CASCADE,
    direction TEXT NOT NULL,
    content TEXT NOT NULL,
    ai_generated BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE omni_conversation_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_conversation_messages ON omni_conversation_messages;
CREATE POLICY tenant_isolation_omni_conversation_messages ON omni_conversation_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
