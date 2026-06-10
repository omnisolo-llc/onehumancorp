-- Migration 113: Universal Inbox (Communication Threads & Messages)

CREATE TABLE IF NOT EXISTS communication_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL, -- e.g., 'instagram', 'whatsapp', 'email', 'webchat'
    status TEXT DEFAULT 'open', -- 'open', 'closed', 'resolved'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_communication_threads_tenant ON communication_threads(tenant_id, created_at DESC);

ALTER TABLE communication_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_communication_threads ON communication_threads;
CREATE POLICY tenant_isolation_communication_threads ON communication_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS communication_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES communication_threads(id) ON DELETE CASCADE,
    direction TEXT NOT NULL, -- 'inbound' or 'outbound'
    sender_id TEXT,
    content TEXT NOT NULL,
    original_content TEXT,
    translated_from_language TEXT,
    draft_reply TEXT,
    status TEXT DEFAULT 'unread', -- 'unread', 'read', 'sent', 'failed'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_communication_messages_thread ON communication_messages(thread_id, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_communication_messages_tenant ON communication_messages(tenant_id, created_at DESC);

ALTER TABLE communication_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_communication_messages ON communication_messages;
CREATE POLICY tenant_isolation_communication_messages ON communication_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
