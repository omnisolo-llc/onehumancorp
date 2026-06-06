-- +goose Up
-- Migration 077: Tenant Integrations

CREATE TABLE IF NOT EXISTS tenant_integrations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    integration_id TEXT NOT NULL,
    category TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'disconnected',
    name TEXT NOT NULL,
    icon TEXT NOT NULL,
    description TEXT,
    config JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, integration_id)
);

CREATE INDEX IF NOT EXISTS idx_tenant_integrations_tenant_id ON tenant_integrations(tenant_id);

-- +goose Down
DROP TABLE IF EXISTS tenant_integrations;
