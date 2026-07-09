-- +goose Up
-- Migration: Autonomous Project Intake & Proposal Agent

CREATE TABLE IF NOT EXISTS project_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    raw_intent TEXT NOT NULL,
    extracted_requirements TEXT,
    status TEXT NOT NULL DEFAULT 'NEW',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_project_requests_tenant ON project_requests(tenant_id);

ALTER TABLE project_requests ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_project_requests ON project_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_project_requests ON project_requests;
DROP TABLE IF EXISTS project_requests CASCADE;
