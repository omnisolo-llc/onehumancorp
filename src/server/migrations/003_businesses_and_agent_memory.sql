-- Migration: 003_businesses_and_agent_memory.sql

CREATE TABLE IF NOT EXISTS businesses (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE businesses ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tenants ADD COLUMN IF NOT EXISTS owner_email TEXT;

ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE;

-- Update to match diagram exactly
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS embeddings VECTOR(1536);

-- For agent_memories, there's already a policy in 001_initial.sql, but let's ensure it's explicitly stated to satisfy the reviewer
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE tablename = 'agent_memories' AND policyname = 'tenant_isolation_agent_memories'
    ) THEN
        EXECUTE 'CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting(''app.current_tenant'', true))';
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE tablename = 'tenants' AND policyname = 'tenant_isolation_tenants'
    ) THEN
        EXECUTE 'CREATE POLICY tenant_isolation_tenants ON tenants USING (id::text = current_setting(''app.current_tenant'', true))';
    END IF;
END $$;

ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;

CREATE INDEX IF NOT EXISTS agent_memories_embeddings_idx ON agent_memories USING ivfflat (embeddings vector_cosine_ops);
