-- +goose Up
-- Migration 037: Add Triage Feed tables for the Agentic Work Triage UI

CREATE TABLE IF NOT EXISTS triage_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source TEXT NOT NULL,
    priority TEXT DEFAULT 'Normal',
    context TEXT,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS triage_proposed_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    triage_item_id TEXT NOT NULL REFERENCES triage_items(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('triage_items') IS NOT NULL THEN
        ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_triage_items ON triage_items
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('triage_proposed_actions') IS NOT NULL THEN
        ALTER TABLE triage_proposed_actions ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_triage_proposed_actions ON triage_proposed_actions
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_triage_proposed_actions ON triage_proposed_actions;
    DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
END
$$;

DROP TABLE IF EXISTS triage_proposed_actions CASCADE;
DROP TABLE IF EXISTS triage_items CASCADE;
