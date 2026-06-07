-- +goose Up
-- Migration 079: Enforce Missing Row Level Security Policies and tenant_id columns

-- Fix swarm_truth_embeddings (RLS policy was added in 005_add_hybrid_sync_metadata but tenant_id was missing)
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
CREATE INDEX IF NOT EXISTS idx_swarm_truth_embeddings_tenant_id ON swarm_truth_embeddings(tenant_id);

ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'swarm_truth_embeddings'
          AND policyname = 'tenant_isolation_swarm_truth_embeddings'
    ) THEN
        CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Fix agent_session_data
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
CREATE INDEX IF NOT EXISTS idx_agent_session_data_tenant_id ON agent_session_data(tenant_id);

ALTER TABLE agent_session_data ENABLE ROW LEVEL SECURITY;

-- Note: a custom RLS might exist for agent_session_data using agent_id, but we'll add the standard tenant_id one as primary
DO $$
BEGIN
    -- Drop old one if it exists to replace with standardized
    DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;

    CREATE POLICY tenant_isolation_agent_session_data ON agent_session_data
        USING (tenant_id::text = current_setting('app.current_tenant', true))
        WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
END
$$;

-- +goose Down
-- Revert agent_session_data
DROP POLICY IF EXISTS tenant_isolation_agent_session_data ON agent_session_data;
ALTER TABLE agent_session_data DROP COLUMN IF EXISTS tenant_id;

-- Revert swarm_truth_embeddings (keep RLS from 005 but remove column means RLS might fail, but this is a down migration)
-- DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
ALTER TABLE swarm_truth_embeddings DROP COLUMN IF EXISTS tenant_id;
