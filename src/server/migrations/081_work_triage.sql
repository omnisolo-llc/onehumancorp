-- +goose Up
-- Migration 081: Work Triage Inbox

CREATE TABLE IF NOT EXISTS triage_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL, -- e.g., 'instagram', 'email', 'booking_engine'
    priority TEXT NOT NULL DEFAULT 'NORMAL', -- 'URGENT', 'ACTION_NEEDED', 'FYI', 'NORMAL'
    context TEXT NOT NULL,
    customer_id TEXT,
    draft_response TEXT,
    proposed_action JSONB DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'OPEN', -- 'OPEN', 'RESOLVED', 'DISMISSED'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_triage_items_tenant_status ON triage_items(tenant_id, status);

ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
CREATE POLICY tenant_isolation_triage_items ON triage_items
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS triage_items;
