-- +goose Up
-- Migration 034: Add onboarding_state table

CREATE TABLE IF NOT EXISTS onboarding_state (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    current_step INTEGER NOT NULL DEFAULT 0,
    state_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, user_id)
);

DO $$
BEGIN
    IF to_regclass('onboarding_state') IS NOT NULL THEN
        ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
END
$$;

DROP TABLE IF EXISTS onboarding_state CASCADE;
