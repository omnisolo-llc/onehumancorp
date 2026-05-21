-- Migration 011: Enforce Row Level Security on agent_kv_store

DO $$
BEGIN
    IF to_regclass('agent_kv_store') IS NOT NULL THEN
        ALTER TABLE agent_kv_store ENABLE ROW LEVEL SECURITY;

        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'agent_kv_store'
                AND policyname = 'tenant_isolation_agent_kv_store'
        ) THEN
            CREATE POLICY tenant_isolation_agent_kv_store
                ON agent_kv_store
                USING (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;
