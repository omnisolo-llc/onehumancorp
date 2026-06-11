-- +goose Up
-- Migration 118: AI Incident Resolution Tables

CREATE TABLE IF NOT EXISTS incidents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'OPEN',
    affected_orders JSONB DEFAULT '[]',
    affected_inventory JSONB DEFAULT '[]',
    resolution_plan JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_incidents_tenant_id ON incidents(tenant_id);

DO $$
BEGIN
    IF to_regclass('incidents') IS NOT NULL THEN
        EXECUTE 'ALTER TABLE incidents ENABLE ROW LEVEL SECURITY';
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'incidents'
                AND policyname = 'tenant_isolation_incidents'
        ) THEN
            EXECUTE 'CREATE POLICY tenant_isolation_incidents ON incidents USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
        END IF;
    END IF;
END
$$;
