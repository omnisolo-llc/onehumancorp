-- +goose Up
-- Migration 106: Add pos_terminal_sessions table

CREATE TABLE IF NOT EXISTS pos_terminal_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    offline_changes_count INT DEFAULT 0,
    pending_reconciliation JSONB DEFAULT '[]'::jsonb,
    last_conflict_resolved_at TIMESTAMPTZ,
    sync_status TEXT DEFAULT 'SYNCED',
    UNIQUE(tenant_id, device_id)
);

DO $$
BEGIN
    IF to_regclass('pos_terminal_sessions') IS NOT NULL THEN
        ALTER TABLE pos_terminal_sessions ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'pos_terminal_sessions'
                AND policyname = 'tenant_isolation_pos_terminal_sessions'
        ) THEN
            CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('pos_terminal_sessions') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions;
        ALTER TABLE pos_terminal_sessions DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS pos_terminal_sessions CASCADE;
