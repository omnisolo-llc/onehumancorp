-- +goose Up
-- Migration: AI Autonomous Work Triage and Daily Work Generation

CREATE TABLE IF NOT EXISTS inbound_signals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    raw_payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'PROCESSED', 'FAILED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS daily_work_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    signal_id TEXT REFERENCES inbound_signals(id) ON DELETE SET NULL,
    intent TEXT NOT NULL,
    customer_info JSONB,
    suggested_actions JSONB,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'APPROVED', 'DISMISSED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS
ALTER TABLE inbound_signals ENABLE ROW LEVEL SECURITY;
ALTER TABLE daily_work_items ENABLE ROW LEVEL SECURITY;

-- Create policies for tenant isolation
DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS daily_work_items CASCADE;
DROP TABLE IF EXISTS inbound_signals CASCADE;
