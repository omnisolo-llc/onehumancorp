-- +goose Up
-- Migration 131: Add mutation_queue and sync_events tables

CREATE TABLE IF NOT EXISTS mutation_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('mutation_queue') IS NOT NULL THEN
        ALTER TABLE mutation_queue ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'mutation_queue'
                AND policyname = 'tenant_isolation_mutation_queue'
        ) THEN
            CREATE POLICY tenant_isolation_mutation_queue ON mutation_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS sync_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    batch_id TEXT,
    action_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('sync_events') IS NOT NULL THEN
        ALTER TABLE sync_events ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'sync_events'
                AND policyname = 'tenant_isolation_sync_events'
        ) THEN
            CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('mutation_queue') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_mutation_queue ON mutation_queue;
        ALTER TABLE mutation_queue DISABLE ROW LEVEL SECURITY;
    END IF;
    IF to_regclass('sync_events') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
        ALTER TABLE sync_events DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS mutation_queue CASCADE;
DROP TABLE IF EXISTS sync_events CASCADE;
