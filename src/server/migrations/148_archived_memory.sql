CREATE TABLE IF NOT EXISTS archived_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER DEFAULT 0,
    reliability_score INTEGER DEFAULT 50,
    owner_override BOOLEAN DEFAULT FALSE,
    metadata TEXT
);
ALTER TABLE IF EXISTS archived_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_archived_memory ON archived_memory USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
CREATE INDEX IF NOT EXISTS archived_memory_tenant_idx ON archived_memory(tenant_id);
