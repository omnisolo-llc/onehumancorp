-- +goose Up
-- Migration 030: Add Recovery Agent tables

CREATE TABLE IF NOT EXISTS recovery_campaigns (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    auto_send BOOLEAN DEFAULT FALSE,
    delay_minutes INTEGER DEFAULT 60,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS recovery_attempts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source_event_id TEXT NOT NULL,
    assistant_message_id TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('recovery_campaigns') IS NOT NULL THEN
        ALTER TABLE recovery_campaigns ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('recovery_attempts') IS NOT NULL THEN
        ALTER TABLE recovery_attempts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_recovery_campaigns ON recovery_campaigns;
    DROP POLICY IF EXISTS tenant_isolation_recovery_attempts ON recovery_attempts;
END
$$;

DROP TABLE IF EXISTS recovery_attempts CASCADE;
DROP TABLE IF EXISTS recovery_campaigns CASCADE;
