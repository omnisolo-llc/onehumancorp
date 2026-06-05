-- Migration 056: Migration Jobs

CREATE TABLE IF NOT EXISTS migration_jobs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    extracted_products JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_migration_jobs_tenant_id ON migration_jobs(tenant_id);

ALTER TABLE migration_jobs ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'migration_jobs'
            AND policyname = 'tenant_isolation_migration_jobs'
    ) THEN
        EXECUTE 'CREATE POLICY tenant_isolation_migration_jobs ON migration_jobs USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
    END IF;
END
$$;
