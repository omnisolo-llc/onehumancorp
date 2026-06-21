-- +goose Up
-- Migration 148: Unified Work Triage Architecture

CREATE TABLE IF NOT EXISTS unified_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    direction TEXT NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS unified_triage_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'dismissed', 'executed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE unified_threads ENABLE ROW LEVEL SECURITY;
ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE unified_triage_actions ENABLE ROW LEVEL SECURITY;

-- Create policies
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- Indices
CREATE INDEX IF NOT EXISTS idx_unified_threads_tenant ON unified_threads(tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_unified_messages_thread ON unified_messages(thread_id);
CREATE INDEX IF NOT EXISTS idx_unified_triage_actions_tenant_status ON unified_triage_actions(tenant_id, status);

-- +goose Down
DROP TABLE IF EXISTS unified_triage_actions CASCADE;
DROP TABLE IF EXISTS unified_messages CASCADE;
DROP TABLE IF EXISTS unified_threads CASCADE;
