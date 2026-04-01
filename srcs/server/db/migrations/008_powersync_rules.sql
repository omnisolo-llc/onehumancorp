-- 008_powersync_rules.sql
-- Implement PowerSync sync rules to enforce strict Tenant isolation.

CREATE TABLE IF NOT EXISTS powersync_sync_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    definition TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
