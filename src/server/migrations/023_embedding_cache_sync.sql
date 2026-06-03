CREATE TABLE IF NOT EXISTS embedding_cache (
    id TEXT PRIMARY KEY,
    prompt TEXT NOT NULL,
    embedding VECTOR(1536),
    synced_to_cloud BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_embedding_cache ON embedding_cache;
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache USING (tenant_id::text = current_setting('app.current_tenant', true));
