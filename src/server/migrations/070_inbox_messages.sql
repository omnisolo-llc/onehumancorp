CREATE TABLE IF NOT EXISTS inbox_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT,
    content TEXT,
    draft_reply TEXT,
    status TEXT,
    sender_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inbox_messages_tenant_created_at ON inbox_messages(tenant_id, created_at DESC);

ALTER TABLE inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
