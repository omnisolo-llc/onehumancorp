-- +goose Up
ALTER TABLE distributed_locks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE distributed_locks SET tenant_id = 'default_tenant' WHERE tenant_id IS NULL;
ALTER TABLE distributed_locks ALTER COLUMN tenant_id SET NOT NULL;

DO $$
BEGIN
    IF to_regclass('distributed_locks') IS NOT NULL THEN
        ALTER TABLE distributed_locks ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'distributed_locks'
                AND policyname = 'tenant_isolation_distributed_locks'
        ) THEN
            CREATE POLICY tenant_isolation_distributed_locks ON distributed_locks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('distributed_locks') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_distributed_locks ON distributed_locks;
        ALTER TABLE distributed_locks DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

ALTER TABLE distributed_locks DROP COLUMN IF EXISTS tenant_id;
