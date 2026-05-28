-- Migration: 020_migration_engine.sql
CREATE TABLE IF NOT EXISTS platform_migrations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source_url TEXT NOT NULL,
    platform_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    metrics TEXT DEFAULT '{}',
    started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMPTZ,
    error_log TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_platform_migrations_tenant_id ON platform_migrations(tenant_id);
