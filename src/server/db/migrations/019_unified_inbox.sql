-- Unified Omnichannel AI Ambassador Inbox

CREATE TABLE IF NOT EXISTS ohc_inbox_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_profile_id TEXT,
    source_channel TEXT NOT NULL,
    requires_human_escalation BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_inbox_threads_tenant
ON ohc_inbox_threads(tenant_id, updated_at DESC);

ALTER TABLE ohc_inbox_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_inbox_threads ON ohc_inbox_threads;
CREATE POLICY tenant_isolation_ohc_inbox_threads
ON ohc_inbox_threads
USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_inbox_messages (
    id TEXT PRIMARY KEY,
    thread_id TEXT NOT NULL REFERENCES ohc_inbox_threads(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    sender_type TEXT NOT NULL, -- 'CUSTOMER', 'AI', 'HUMAN'
    content TEXT NOT NULL,
    draft_content TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_inbox_messages_thread
ON ohc_inbox_messages(thread_id, created_at ASC);

CREATE INDEX IF NOT EXISTS idx_ohc_inbox_messages_tenant
ON ohc_inbox_messages(tenant_id);

ALTER TABLE ohc_inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_inbox_messages ON ohc_inbox_messages;
CREATE POLICY tenant_isolation_ohc_inbox_messages
ON ohc_inbox_messages
USING (tenant_id = current_setting('app.current_tenant', true));
