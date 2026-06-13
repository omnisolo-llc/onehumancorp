-- +goose Up
-- Migration 126: Add team_invites tables

CREATE TABLE IF NOT EXISTS team_invites (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    team_id TEXT NOT NULL,
    inviter_id TEXT NOT NULL,
    invitee_id TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('team_invites') IS NOT NULL THEN
        ALTER TABLE team_invites ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
END
$$;

DROP TABLE IF EXISTS team_invites CASCADE;
