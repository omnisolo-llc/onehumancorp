-- +goose Up
-- Migration 148: Unified Inbox Triage (Threads, Messages, TriageActions)

CREATE TABLE IF NOT EXISTS unified_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_triage_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    action_payload TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('unified_threads') IS NOT NULL THEN
        ALTER TABLE unified_threads ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('unified_messages') IS NOT NULL THEN
        ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;

    IF to_regclass('unified_triage_actions') IS NOT NULL THEN
        ALTER TABLE unified_triage_actions ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
    DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
    DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
END
$$;

DROP TABLE IF EXISTS unified_triage_actions CASCADE;
DROP TABLE IF EXISTS unified_messages CASCADE;
DROP TABLE IF EXISTS unified_threads CASCADE;
