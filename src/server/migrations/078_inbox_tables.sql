CREATE TABLE IF NOT EXISTS inbox_messages_v2 (
    message_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    channel_type TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inbox_messages_v2_tenant_created_at ON inbox_messages_v2(tenant_id, created_at DESC);

ALTER TABLE inbox_messages_v2 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages_v2 ON inbox_messages_v2;
CREATE POLICY tenant_isolation_inbox_messages_v2 ON inbox_messages_v2 USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ai_drafts (
    draft_id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL REFERENCES inbox_messages_v2(message_id) ON DELETE CASCADE,
    proposed_content TEXT NOT NULL,
    approval_status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ai_drafts_message_id ON ai_drafts(message_id);

-- Cannot enforce RLS nicely without a direct tenant_id column, but we can do a JOIN policy
ALTER TABLE ai_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_drafts ON ai_drafts;
CREATE POLICY tenant_isolation_ai_drafts ON ai_drafts
USING (
    EXISTS (
        SELECT 1 FROM inbox_messages_v2 m
        WHERE m.message_id = ai_drafts.message_id
        AND m.tenant_id::text = current_setting('app.current_tenant', true)
    )
)
WITH CHECK (
    EXISTS (
        SELECT 1 FROM inbox_messages_v2 m
        WHERE m.message_id = ai_drafts.message_id
        AND m.tenant_id::text = current_setting('app.current_tenant', true)
    )
);
