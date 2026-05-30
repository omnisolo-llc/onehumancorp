-- +goose Up
-- Migration 023: Customer Timeline RLS

CREATE TABLE IF NOT EXISTS customer_timeline (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT DEFAULT '{}',
    embedding BLOB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_customer_timeline_tenant_customer ON customer_timeline(tenant_id, customer_id);

ALTER TABLE customer_timeline ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Reverse Migration 023

DROP POLICY IF EXISTS tenant_isolation_customer_timeline ON customer_timeline;
ALTER TABLE customer_timeline DISABLE ROW LEVEL SECURITY;
-- DROP TABLE IF EXISTS customer_timeline CASCADE;
