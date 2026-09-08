-- +goose Up
-- Runtime-backed booking and local lead generation tables were present in the
-- SQLite/MySQL bootstrap schema but missing from the PostgreSQL migrations.

CREATE TABLE IF NOT EXISTS availability_blocks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (end_time > start_time)
);

CREATE INDEX IF NOT EXISTS idx_availability_blocks_tenant_service_start
    ON availability_blocks (tenant_id, service_id, start_time)
    WHERE is_available;

CREATE TABLE IF NOT EXISTS lead_gen_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    budget NUMERIC(10, 2) NOT NULL CHECK (budget >= 0),
    radius_miles INTEGER NOT NULL CHECK (radius_miles > 0),
    zip_code TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

GRANT ALL PRIVILEGES ON availability_blocks, lead_gen_campaigns TO ohc_bypassrls;
