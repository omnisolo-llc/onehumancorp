CREATE TABLE IF NOT EXISTS capability_plugins (
    plugin_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    manifest_url TEXT NOT NULL,
    status TEXT NOT NULL,
    registered_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT NOT NULL,
    vector_embedding BYTEA,
    source_plugin TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_swarm_memory_embeddings_organization_id ON swarm_memory_embeddings(organization_id);

ALTER TABLE swarm_memory_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings;
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (organization_id::text = current_setting('app.current_tenant', true));
